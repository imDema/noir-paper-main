use regex::Regex;
use xshell::Shell;

// |q4                 |100,000,000        |17.61              |197.484            |3478.519           |28.75 K/s          |
struct Row {
    query: String,
    time: f32,
}

const RE: &str = r"\|(q\d+)[^|]+(?:\|[^|]+){2}\|((?:\d|\.)+)";

fn row(s: &str, re: &Regex) -> Option<Row> {
    let cap = re.captures(s)?;

    Some(Row { query: cap.get(1)?.as_str().to_string(), time: cap.get(2)?.as_str().parse().ok()? })
}

fn main() -> eyre::Result<()> {
    env_logger::init();

    let dir = std::env::args()
        .nth(1)
        .expect("specify result dir as first arg!");
    let sh = Shell::new()?;

    let re = Regex::new(RE).unwrap();
    for path in sh.read_dir(&dir)? {
        let text = sh.read_file(path)?;
        for Row { query, time } in text.lines().flat_map(|l| row(l, &re)) {
            println!("{query},{time}");
        }
    }

    Ok(())
}
