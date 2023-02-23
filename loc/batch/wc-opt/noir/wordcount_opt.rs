fn main() {
    let (config, args) = EnvironmentConfig::from_args();
    let mut env = StreamEnvironment::new(config);

    env.spawn_remote_workers();

    let result = env
        .stream_file(path)
        .batch_mode(BatchMode::fixed(1024))
        .fold_assoc(
            HashMap::<String, u64, BuildHasherDefault<wyhash::WyHash>>::default(),
            |acc, line| {
                let mut word = String::with_capacity(8);
                for c in line.chars() {
                    if c.is_ascii_alphabetic() {
                        word.push(c.to_ascii_lowercase());
                    } else if !word.is_empty() {
                        let key = std::mem::replace(&mut word, String::with_capacity(8));
                        *acc.entry(key).or_default() += 1;
                    }
                }
            },
            |a, mut b| {
                for (k, v) in b.drain() {
                    *a.entry(k).or_default() += v;
                }
            },
        )
        .collect_vec();

    env.execute();

    if let Some(_res) = result.get() {}
}
