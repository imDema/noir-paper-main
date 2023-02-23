const DAYS_BEFORE: [u16; 13] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

type Week = (u16, u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Accident {
    #[serde(rename = "DATE")]
    date: String,
    #[serde(rename = "BOROUGH")]
    borough: String,
    #[serde(rename = "NUMBER OF PERSONS KILLED")]
    killed: u32,
    #[serde(rename = "CONTRIBUTING FACTOR VEHICLE 1")]
    factor1: String,
    #[serde(rename = "CONTRIBUTING FACTOR VEHICLE 2")]
    factor2: String,
    #[serde(rename = "CONTRIBUTING FACTOR VEHICLE 3")]
    factor3: String,
    #[serde(rename = "CONTRIBUTING FACTOR VEHICLE 4")]
    factor4: String,
    #[serde(rename = "CONTRIBUTING FACTOR VEHICLE 5")]
    factor5: String,
}

impl Accident {
    fn week(&self) -> Week {
        let mut day = u16::from_str(&self.date[3..5]).unwrap();
        let month = u16::from_str(&self.date[0..2]).unwrap();
        let year = u16::from_str(&self.date[6..10]).unwrap();
        day += DAYS_BEFORE[month as usize];
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) && month >= 3 {
            day += 1;
        }
        (year, day / 7)
    }
}

fn query1_with_source(
    source: Stream<Accident, impl Operator<Accident> + 'static>,
) -> StreamOutput<Vec<(Week, u32)>> {
    source
        // map to the week with 1 if it was lethal, 0 otherwise
        .map(|a| (a.week(), (a.killed > 0) as u32))
        .group_by_sum(|(week, _killed)| *week, |(_week, killed)| killed)
        .collect_vec()
}

fn query1<P: Into<PathBuf>>(
    env: &mut StreamEnvironment,
    path: P,
) -> StreamOutput<Vec<(Week, u32)>> {
    let source = CsvSource::<Accident>::new(path).has_headers(true);
    let source = env.stream(source).batch_mode(BatchMode::fixed(1000));
    query1_with_source(source)
}

#[allow(clippy::print_literal)]
fn print_query1(res: Vec<(Week, u32)>) {}

fn query2_with_source(
    source: Stream<Accident, impl Operator<Accident> + 'static>,
) -> StreamOutput<Vec<(String, i32, u32)>> {
    source
        // extract (factor, num accidents, num kills)
        .flat_map(|a| {
            vec![
                (a.factor1, 1, (a.killed > 0) as u32),
                (a.factor2, 1, (a.killed > 0) as u32),
                (a.factor3, 1, (a.killed > 0) as u32),
                (a.factor4, 1, (a.killed > 0) as u32),
                (a.factor5, 1, (a.killed > 0) as u32),
            ]
            .into_iter()
            .unique()
            .collect_vec()
        })
        // ignore empty factors
        .filter(|(f, _, _)| !f.is_empty())
        // group by factors
        .group_by_fold(
            |(f, _, _)| f.clone(),
            (0, 0),
            |(a1, k1), (_f, a2, k2)| {
                *a1 += a2;
                *k1 += k2
            },
            |(a1, k1), (a2, k2)| {
                *a1 += a2;
                *k1 += k2
            },
        )
        .unkey()
        .map(|(f, (a, k))| (f, a, k))
        .collect_vec()
}

fn query2<P: Into<PathBuf>>(
    env: &mut StreamEnvironment,
    path: P,
) -> StreamOutput<Vec<(String, i32, u32)>> {
    let source = CsvSource::<Accident>::new(path).has_headers(true);
    let source = env.stream(source).batch_mode(BatchMode::fixed(1000));
    query2_with_source(source)
}

#[allow(clippy::print_literal)]
fn print_query2(res: Vec<(String, i32, u32)>) {}

#[allow(clippy::type_complexity)]
fn query3_with_source(
    source: Stream<Accident, impl Operator<Accident> + 'static>,
) -> (
    StreamOutput<Vec<(String, Week, i32, u32)>>,
    StreamOutput<Vec<((String, u16), (i32, u32, f64))>>,
) {
    let mut splitted = source
        .batch_mode(BatchMode::fixed(1000))
        // extract borough, week, num accidents, num lethal
        .map(|a| (a.borough.clone(), a.week(), 1, (a.killed > 0) as u32))
        // group by (borough, week) and sum accidents and lethal
        .group_by_fold(
            |(borough, week, _, _)| (borough.clone(), *week),
            (0, 0),
            |(a1, k1), (_, _, a2, k2)| {
                *a1 += a2;
                *k1 += k2;
            },
            |(a1, k1), (a2, k2)| {
                *a1 += a2;
                *k1 += k2
            },
        )
        .unkey()
        .map(|((borough, week), (accidents, killed))| (borough, week, accidents, killed))
        .split(2);

    let res = splitted.pop().unwrap().collect_vec();

    let avg_per_week = splitted
        .pop()
        .unwrap()
        // map into (borough, week number, accidents, lethal accidents)
        .map(|(b, (_, w), a, k)| (b, w, a, k))
        // group by (borough, week number)
        .group_by_fold(
            |(b, w, _, _)| (b.clone(), *w),
            (0, 0),
            |(a1, k1), (_, _, a2, k2)| {
                *a1 += a2;
                *k1 += k2;
            },
            |(a1, k1), (a2, k2)| {
                *a1 += a2;
                *k1 += k2
            },
        )
        .unkey()
        // compute the average
        .map(|((b, w), (a, k))| ((b, w), (a, k, 100.0 * (k as f64) / (a as f64))))
        .collect_vec();

    (res, avg_per_week)
}

#[allow(clippy::type_complexity)]
fn query3<P: Into<PathBuf>>(
    env: &mut StreamEnvironment,
    path: P,
) -> (
    StreamOutput<Vec<(String, Week, i32, u32)>>,
    StreamOutput<Vec<((String, u16), (i32, u32, f64))>>,
) {
    let source = CsvSource::<Accident>::new(path).has_headers(true);
    let source = env.stream(source).batch_mode(BatchMode::fixed(1000));
    query3_with_source(source)
}

#[allow(clippy::print_literal, clippy::type_complexity)]
fn print_query3(
    res: Vec<(String, Week, i32, u32)>,
    avg_per_week: Vec<((String, u16), (i32, u32, f64))>,
) {
}

fn main() {
    let (config, args) = EnvironmentConfig::from_args();

    let mut env = StreamEnvironment::new(config);
    env.spawn_remote_workers();

    let (query1, query2, query3) = if share_source {
        let source = CsvSource::<Accident>::new(path).has_headers(true);
        let mut splits = env
            .stream(source)
            .batch_mode(BatchMode::fixed(1000))
            .split(3)
            .into_iter();
        (
            query1_with_source(splits.next().unwrap()),
            query2_with_source(splits.next().unwrap()),
            query3_with_source(splits.next().unwrap()),
        )
    } else {
        (
            query1(&mut env, path),
            query2(&mut env, path),
            query3(&mut env, path),
        )
    };

    env.execute();

    // print only in debug mode
    if let Some(query1) = query1.get() {
    }
    if let Some(query2) = query2.get() {
    }
    if let Some(res) = query3.0.get() {
        if let Some(avg_per_week) = query3.1.get() {
        }
    }
}
