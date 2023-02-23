
fn main() {
    let (config, args) = EnvironmentConfig::from_args();
    
    let mut env = StreamEnvironment::new(config);

    env.spawn_remote_workers();

    let source = FileSource::new(path);
    let tokenizer = Tokenizer::new();
    let result = env
        .stream(source)
        .batch_mode(BatchMode::fixed(1024))
        .flat_map(move |line| tokenizer.tokenize(line))
        .group_by_count(|word: &String| word.clone())
        .collect_vec();
    env.execute();

    if let Some(_res) = result.get() {
    }
}

#[derive(Clone)]
struct Tokenizer(Regex)

impl Tokenizer {
    fn new() -> Self {
        Self(Regex::new(r"[A-Za-z]+").unwrap())
    }
    fn tokenize(&self, value: String) -> Vec<String> {
        self.0
            .find_iter(&value)
            .map(|t| t.as_str().to_lowercase())
            .collect()
    }
}
