const EPS: f64 = 1e-8;
const DAMPENING: f64 = 0.85;

fn main() {
    let (config, args) = EnvironmentConfig::from_args();

    let mut env = StreamEnvironment::new(config);

    env.spawn_remote_workers();

    let q = Instant::now();
    let links = BufReader::new(File::open(path_links).unwrap());
    let adjacency_list: HashMap<u64, Vec<u64>, BuildHasherDefault<fxhash::FxHasher>> =
        csv::ReaderBuilder::default()
            .from_reader(links)
            .into_deserialize()
            .map(|r| r.unwrap())
            .fold(
                HashMap::with_capacity_and_hasher(num_pages, Default::default()),
                |mut acc, (x, y)| {
                    acc.entry(x).or_default().push(y);
                    acc
                },
            );

    let adjacency_list = Arc::new(adjacency_list);

    let pages_source = CsvSource::<u64>::new(path_pages).has_headers(false);
    let (dropme, result) = env
        .stream(pages_source)
        // distribute the ranks evenly
        .map(move |x| (x, 0.0, 1.0 / num_pages as f64))
        .iterate(
            num_iterations,
            // state maintains whether a new iteration is needed
            false,
            move |s, _| {
                s.flat_map(move |(x, _, rank)| {
                    if let Some(adj) = adjacency_list.get(&x) {
                        // distribute the rank of the page between the connected pages
                        let distribute = rank / adj.len() as f64;
                        adj.iter().map(move |y| (*y, distribute)).collect()
                    } else {
                        vec![]
                    }
                })
                .group_by_sum(|(x, _)| *x, |(_, rank)| rank)
                .rich_map({
                    let mut prev = 0.0;
                    move |(x, rank)| {
                        let rank = rank * DAMPENING + (1.0 - DAMPENING) / num_pages as f64;
                        (*x, replace(&mut prev, rank), rank)
                    }
                })
                .drop_key()
            },
            // a new iteration is needed if at least one page's rank has changed
            |changed: &mut bool, (_x, old, new)| {
                *changed = *changed || (new - old).abs() / new > EPS
            },
            |state, changed| *state = *state || changed,
            |state| replace(state, false),
        );
    let result = result.collect_vec();
    dropme.for_each(|_| {});

    env.execute();

    if let Some(mut res) = result.get() {}
}
