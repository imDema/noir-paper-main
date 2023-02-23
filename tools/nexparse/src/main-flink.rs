use nom::bytes::complete::{tag, take_until, take_while1};
use nom::combinator::map_res;
use nom::multi::count;
use nom::sequence::delimited;
use nom::IResult;
use xshell::Shell;

// |q4                 |100,000,000        |17.61              |197.484            |3478.519           |28.75 K/s          |
struct Row {
    query: String,
    time: f32,
}

fn field(s: &str) -> IResult<&str, &str> {
    delimited(
        tag("|"),
        take_while1(|c: char| c.is_ascii_graphic()),
        take_until("|"),
    )(s)
}

fn row(s: &str) -> IResult<&str, Row> {
    let (s, query) = field(s)?;
    let (_, _) = tag("q")(query)?; // Filter starts with q
    let (s, _) = count(field, 2)(s)?;
    let (s, time) = map_res(field, |s| s.parse())(s)?;

    Ok((
        s,
        Row {
            query: query.to_owned(),
            time,
        },
    ))
}

fn main() -> eyre::Result<()> {
    env_logger::init();

    let dir = std::env::args()
        .nth(1)
        .expect("specify result dir as first arg!");
    let sh = Shell::new()?;

    for path in sh.read_dir(&dir)? {
        let text = sh.read_file(path)?;
        for Row { query, time } in text.lines().flat_map(|l| row(l).ok().map(|r| r.1)) {
            println!("flink,32,{query},{time}");
        }
    }

    Ok(())
}
