fn main() {
    let (config, args) = EnvironmentConfig::from_args();

    let mut env = StreamEnvironment::new(config);

    env.spawn_remote_workers();

    let edges_source = CsvSource::<(u64, u64)>::new(path_edges)
        .delimiter(b',')
        .has_headers(false);

    let mut edges = env.stream(edges_source).split(2);

    let (state, result) = edges.pop().unwrap().iterate(
        num_iterations,
        // (old, new) count of paths in the transitive closure
        (0, 0, 0),
        move |s, _| {
            let mut paths = s.split(2);
            paths
                .pop()
                .unwrap()
                .join(edges.pop().unwrap(), |(_z, x)| *x, |(x, _y)| *x)
                // if there are a path z -> x and an edge x -> y, then generate the path z -> y
                .map(|(_, ((z, _), (_, y)))| (z, y))
                .drop_key()
                // concatenate the paths already present in the transitive closure
                .merge(paths.pop().unwrap())
                // delete duplicated paths
                .group_by_reduce(|(x, y)| (*x, *y), |_, _| {})
                .drop_key()
                .filter(|(x, y)| x != y)
        },
        |count: &mut u64, _| *count += 1,
        |(_old, new, _iter), count| *new += count,
        |(old, new, iter)| {
            *iter += 1;
            let condition = old != new;
            *old = *new;
            *new = 0;
            condition
        },
    );

    // we are interested in the stream output
    let result = result.collect_vec();
    state.for_each(|(_, _, iter)| eprintln!("Iterations: {iter}"));

    env.execute();

    if let Some(mut res) = result.get() {}
}
