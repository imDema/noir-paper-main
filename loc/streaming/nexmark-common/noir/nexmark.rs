const WATERMARK_INTERVAL: usize = 1 << 20;
const BATCH_SIZE: usize = 16 << 10;
const SECOND_MILLIS: i64 = 1_000;

fn timestamp_gen(e: &Event) -> Timestamp {
    e.timestamp() as i64
}

fn watermark_gen(ts: &Timestamp, count: &mut usize, interval: usize) -> Option<Timestamp> {
    *count = (*count + 1) % interval;
    if *count == 0 {
        Some(*ts)
    } else {
        None
    }
}

fn events(env: &mut StreamEnvironment, tot: usize) -> Stream<Event, impl Operator<Event>> {
    env.stream_par_iter(move |i, n| {
        let conf = NexmarkConfig {
            num_event_generators: n as usize,
            first_rate: 10_000_000,
            next_rate: 10_000_000,
            ..Default::default()
        };
        nexmark::EventGenerator::new(conf)
            .with_offset(i)
            .with_step(n)
            .take(tot / n as usize + (i < tot as u64 % n) as usize)
    })
    .batch_mode(BatchMode::fixed(BATCH_SIZE))
}

fn unwrap_auction(e: Event) -> Auction {
    match e {
        Event::Auction(x) => x,
        _ => panic!("tried to unwrap wrong event type!"),
    }
}
fn unwrap_person(e: Event) -> Person {
    match e {
        Event::Person(x) => x,
        _ => panic!("tried to unwrap wrong event type!"),
    }
}
fn filter_bid(e: Event) -> Option<Bid> {
    match e {
        Event::Bid(x) => Some(x),
        _ => None,
    }
}

fn main() {
    let (config, args) = EnvironmentConfig::from_args();
    let mut env = StreamEnvironment::new(config);
    env.spawn_remote_workers();

    match q {
        "0" => query0(events(&mut env, n)),
        "1" => query1(events(&mut env, n)),
        "2" => query2(events(&mut env, n)),
        "3" => query3(events(&mut env, n)),
        "4" => query4(events(&mut env, n)),
        "5" => query5(events(&mut env, n)),
        "6" => query6(events(&mut env, n)),
        "7" => query7(events(&mut env, n)),
        "8" => query8(events(&mut env, n)),

        _ => panic!("Invalid query! {q}"),
    }

    let start = Instant::now();
    env.execute();
    println!("q{q}:elapsed:{:?}", start.elapsed());
}
