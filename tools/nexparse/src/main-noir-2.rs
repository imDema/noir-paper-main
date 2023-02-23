use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use eyre::ContextCompat;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::opt;
use nom::number::complete::float;
use nom::IResult;
use serde::Deserialize;
use serde_json::Value;
use xshell::Shell;

// 3|q0:elapsed:6.0191825s
// 1|q1:elapsed:6.036558924s
// 0|q2:elapsed:6.039664242s
// 2|q3:elapsed:6.056800357s


struct Row {
    query: String,
    time: f32,
}

fn time_line<'a>(s: &'a str, suffix: &str) -> IResult<&'a str, Row> {
    let (s, _) = opt(tag("0|"))(s)?;
    tag("q")(s)?;
    let (s, q) = take_until(":")(s)?;
    let (s, _) = tag(":elapsed:")(s)?;
    let (s, n) = float(s)?;
    let (s, _) = tag("s")(s)?;
    let r = Row { query: q.to_string(), time: n };
    Ok((s, r))
}


fn parse(s: &str, suffix: &str) -> Result<Vec<Row>, eyre::Error> {
    let r = s.lines()
        .filter_map(|l| time_line(l, suffix).ok().map(|r| r.1))
        .collect();

    Ok(r)
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value(""))]
    suffix: String,
    #[clap(subcommand)]
    subcommand: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Json {path: PathBuf},
    Output {path: PathBuf},
}

fn main() -> eyre::Result<()> {
    env_logger::init();

    let args = Args::parse();
    let sh = Shell::new()?;

    match args.subcommand {
        Cmd::Json { path } => json(&sh, &path, &args.suffix),
        Cmd::Output { path } => output(&sh, &path, &args.suffix),
    }
}

fn output(sh: &Shell, path: &Path, suffix: &str) -> eyre::Result<()> {
    for path in sh.read_dir(&path)? {
        let text = sh.read_file(path)?;
        for Row { query, time } in parse(&text, "")?.into_iter() {
            println!("noir,32,{query},{time}");
        }
    }

    Ok(())
}

#[derive(Deserialize)]
struct JsonResult {
    result: RunResult,
    system: String,
    num_hosts: usize,
}

#[derive(Deserialize)]
struct RunResult {
    stdout: String,
}

fn json(sh: &Shell, path: &Path, suffix: &str) -> eyre::Result<()> {

    let re = regex::Regex::new(&format!(r"q\d+{suffix}")).unwrap();

    for path in sh.read_dir(&path)? {
        if path.is_dir() {
            json(sh, &path, suffix)?;
            continue;
        }
        if !re.is_match(&format!("{}", path.display())) {
            // eprintln!("skipping {}", path.display());
            continue;
        }
        eprintln!("{}", path.display());

        let text = sh.read_file(path)?;
        let res: JsonResult = serde_json::from_str(&text)?;

        for Row { query, time } in parse(&res.result.stdout, suffix)?.into_iter() {
            println!("{},{},{query},{time}", res.system, res.num_hosts * 8);
        }
    }

Ok(())
}