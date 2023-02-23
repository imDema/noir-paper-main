const EPS: f64 = 1e-8;
const DAMPENING: f64 = 0.85;

fn main() {
    let (config, args) = EnvironmentConfig::from_args();
    let mut env = StreamEnvironment::new(config);

    env.spawn_remote_workers();

    let pages_source = CsvSource::<u64>::new(path_pages).has_headers(false);
    let links_source = CsvSource::<(u64, u64)>::new(path_links).has_headers(false);

    let adj_list = env
        .stream(links_source)
        // construct adjacency list
        .group_by_fold(
            |(x, _y)| *x,
            Vec::new(),
            |edges, (_x, y)| edges.push(y),
            |edges1, mut edges2| edges1.append(&mut edges2),
        )
        .unkey();

    let (dropme, result) = env
        .stream(pages_source)
        // distribute the ranks evenly
        .map(move |x| (x, 1.0 / num_pages as f64, 1.0 / num_pages as f64))
        .iterate(
            num_iterations,
            // state maintains whether a new iteration is needed
            false,
            move |s, _| {
                let mut splits = s.split(2);
                // keep ranks before being updated
                let old_ranks = splits.pop().unwrap().map(|(x, _, rank)| (x, rank));

                splits
                    .pop()
                    .unwrap()
                    .map(|(x, _, rank)| (x, rank))
                    .join(adj_list, |(x, _rank)| *x, |(x, _adj)| *x)
                    .flat_map(|(_, ((_x, rank), (_, adj)))| {
                        // distribute the rank of the page between the connected pages
                        let rank_to_distribute = rank / adj.len() as f64;
                        adj.into_iter().map(move |y| (y, rank_to_distribute))
                    })
                    .drop_key()
                    .group_by_sum(|(y, _)| *y, |(_y, rank_to_distribute)| rank_to_distribute)
                    // apply dampening factor
                    .map(move |(_y, rank)| rank * DAMPENING + (1.0 - DAMPENING) / num_pages as f64)
                    .unkey()
                    // generate the pair of old and new rank for each page
                    .join(old_ranks, |(y, _)| *y, |(y, _)| *y)
                    .map(|(_, ((_, new), (_, old)))| (old, new))
                    .unkey()
                    .map(|(y, (old, new))| (y, old, new))
            },
            // a new iteration is needed if at least one page's rank has changed
            |changed: &mut bool, (_x, old, new)| {
                *changed = *changed || (new - old).abs() / new > EPS
            },
            |state, changed| *state = *state || changed,
            |state| {
                let condition = *state;
                *state = false;
                condition
            },
        );
    let result = result.collect_vec();
    dropme.for_each(|_| {});

    env.execute();

    if let Some(mut res) = result.get() {}
}
