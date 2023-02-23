use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::digit1;
use nom::combinator::{map_res, map};
use nom::multi::{count, many1};
use nom::number::complete::float;
use nom::sequence::{delimited, pair};
use nom::IResult;
use xshell::Shell;

// 3|elapsed: 6.0191825s
// 1|elapsed: 6.036558924s
// 0|elapsed: 6.039664242s
// 2|elapsed: 6.056800357s
// 3|Query3: None
// 1|Query3: None
// 0|Query3: Some(603612)
// 2|Query3: None


struct Row {
    query: String,
    time: f32,
}

fn time_line(s: &str) -> IResult<&str, f32> {
    let (s, _) = tag("0|elapsed: ")(s)?;
    let (s, n) = float(s)?;
    let (s, _) = take_until("0|")(s)?;
    Ok((s, n))
}

fn query_line(s: &str) -> IResult<&str, String> {
    let (s, _) = tag("0|Query")(s)?;
    let (s, n) = digit1(s)?;
    let (s, _) = take_until("(")(s)?;
    let (s, _) = take_until(")")(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, n.to_owned()))
}

fn parse(s: &str) -> Result<Vec<Row>, eyre::Error> {
    let filtered = s.lines().filter(|l| l.starts_with("0|")).collect::<String>();
    // eprintln!("{filtered}");

    let mut times = Vec::new();
    let mut queries = Vec::new();

    let mut s = &filtered[..];

    loop {
        let mut quit = true;
        if let Ok((ss, time)) = time_line(s) {
            s = ss;
            times.push(time);
            quit = false;
        }
        if let Ok((ss, query)) = query_line(s) {
            s = ss;
            queries.push(query);
            quit = false;
        }
        if quit {
            break;
        }
    }

    eprintln!("{s}");
    assert_eq!(times.len(), queries.len());
    

    let r = queries.into_iter().zip(times).map(|(query, time)| Row { query, time }).collect();

    Ok(r)
}

fn main() -> eyre::Result<()> {
    env_logger::init();

    let dir = std::env::args()
        .nth(1)
        .expect("specify result dir as first arg!");
    let sh = Shell::new()?;

    for path in sh.read_dir(&dir)? {
        let text = sh.read_file(path)?;
        for Row { query, time } in parse(&text)?.into_iter() {
            println!("noir,32,q{query},{time}");
        }
    }

    Ok(())
}
