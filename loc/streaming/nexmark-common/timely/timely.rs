fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut h: ::fnv::FnvHasher = Default::default();
    t.hash(&mut h);
    h.finish()
}

fn main() {
    assert_eq!(queries.len(), 1);
    let q = &queries[0].to_owned();
    static FIRST: AtomicBool = AtomicBool::new(false);

    let start = Instant::now();
    let _timelines: Vec<_> = timely::execute_from_args(timely_args.into_iter(), move |worker| {

        let peers = worker.peers();
        let index = worker.index();

        if index == 0 {
            FIRST.store(true, std::sync::atomic::Ordering::Relaxed);
        }

        // Declare re-used input, control and probe handles.
        let mut input = InputHandle::new();
        let mut control_input = InputHandle::new();
        let mut probe = ProbeHandle::new();

        {
            let control = std::rc::Rc::new(timely::dataflow::operators::capture::event::link::EventLink::new());

            let bids = std::rc::Rc::new(timely::dataflow::operators::capture::event::link::EventLink::new());
            let auctions = std::rc::Rc::new(timely::dataflow::operators::capture::event::link::EventLink::new());
            let people = std::rc::Rc::new(timely::dataflow::operators::capture::event::link::EventLink::new());


            let closed_auctions = std::rc::Rc::new(timely::dataflow::operators::capture::event::link::EventLink::new());
            let closed_auctions_flex = std::rc::Rc::new(timely::dataflow::operators::capture::event::link::EventLink::new());

            let nexmark_input = NexmarkInput {
                control: &control,
                bids: &bids,
                auctions: &auctions,
                people: &people,
                closed_auctions: &closed_auctions,
                closed_auctions_flex: &closed_auctions_flex,
            };

            let nexmark_timer = NexmarkTimer {
                time_dilation
            };

            eprintln!("starting {index:02}[{peers:02}]");

            worker.dataflow(|scope: &mut ::timely::dataflow::scopes::Child<_, usize>| {
                use timely::dataflow::operators::generic::builder_rc::OperatorBuilder;
                let mut demux = OperatorBuilder::new("NEXMark demux".to_string(), scope.clone());

                let mut input = demux.new_input(&input.to_stream(scope), Pipeline);

                let (mut b_out, bids_stream) = demux.new_output();
                let (mut a_out, auctions_stream) = demux.new_output();
                let (mut p_out, people_stream) = demux.new_output();

                let mut demux_buffer = Vec::new();

                demux.build(move |_capability| {
                    move |_frontiers| {
                        let mut b_out = b_out.activate();
                        let mut a_out = a_out.activate();
                        let mut p_out = p_out.activate();

                        input.for_each(|time, data| {
                            data.swap(&mut demux_buffer);
                            let mut b_session = b_out.session(&time);
                            let mut a_session = a_out.session(&time);
                            let mut p_session = p_out.session(&time);

                            for datum in demux_buffer.drain(..) {
                                match datum {
                                    nexmark::event::Event::Bid(b) => { b_session.give(b) },
                                    nexmark::event::Event::Auction(a) => { a_session.give(a) },
                                    nexmark::event::Event::Person(p) => { p_session.give(p) },
                                }
                            }
                        });
                    }
                });

                bids_stream.capture_into(bids.clone());
                auctions_stream.capture_into(auctions.clone());
                people_stream.capture_into(people.clone());
                control_input.to_stream(scope).broadcast().capture_into(control.clone());
            });


            // Q0: Do nothing in particular.
            if queries.iter().any(|x| *x == "q0") {
                worker.dataflow(|scope| {
                    ::nexmark::queries::q0(&nexmark_input, nexmark_timer, scope).probe_with(&mut probe);
                });
            }

            // Q1: Convert bids to euros.
            if queries.iter().any(|x| *x == "q1") {
                worker.dataflow(|scope| {
                    ::nexmark::queries::q1(&nexmark_input, nexmark_timer, scope).probe_with(&mut probe);
                });
            }

            // Q2: Filter some auctions.
            if queries.iter().any(|x| *x == "q2") {
                worker.dataflow(|scope| {
                    ::nexmark::queries::q2(&nexmark_input, nexmark_timer, scope).probe_with(&mut probe);
                });
            }

            // Q3: Join some auctions.
            if queries.iter().any(|x| *x == "q3") {
                worker.dataflow(|scope| {
                    ::nexmark::queries::q3(&nexmark_input, nexmark_timer, scope).probe_with(&mut probe);
                });
            }

            // Intermission: Close some auctions.
            if queries.iter().any(|x| *x == "q4" || *x == "q6") {
                worker.dataflow(|scope| {
                    ::nexmark::queries::q4_q6_common(&nexmark_input, nexmark_timer, scope)
                        .capture_into(nexmark_input.closed_auctions.clone());
                });
            }

            if queries.iter().any(|x| *x == "q4") {
                worker.dataflow(|scope| {
                    ::nexmark::queries::q4(&nexmark_input, nexmark_timer, scope).probe_with(&mut probe);
                });
            }

            if queries.iter().any(|x| *x == "q5") {
                // 60s windows, ticking in 1s intervals
                // NEXMark default is 60 minutes, ticking in one minute intervals
                let window_slice_count = 5;
                let window_slide_ns = 2_000_000_000;
                worker.dataflow(|scope| {
                    ::nexmark::queries::q5(&nexmark_input, nexmark_timer, scope, window_slice_count, window_slide_ns).probe_with(&mut probe);
                });
            }

            if queries.iter().any(|x| *x == "q6") {
                worker.dataflow(|scope| {
                    ::nexmark::queries::q6(&nexmark_input, nexmark_timer, scope).probe_with(&mut probe);
                });
            }

            if queries.iter().any(|x| *x == "q7") {
                worker.dataflow(|scope| {
                    // Window ticks every 10 seconds.
                    // NEXMark default is different: ticks every 60s
                    let window_size_ns = 10_000_000_000;
                    ::nexmark::queries::q7(&nexmark_input, nexmark_timer, scope, window_size_ns).probe_with(&mut probe);
                });
            }

            if queries.iter().any(|x| *x == "q8") {
                worker.dataflow(|scope| {
                    ::nexmark::queries::q8(&nexmark_input, nexmark_timer, scope).probe_with(&mut probe);
                });
            }
        }

        let mut config1 = nexmark::config::Config::new();
        config1.insert("events-per-second", format!("{}", rate));
        config1.insert("first-event-number", format!("{}", index));
        let mut config = nexmark::config::NEXMarkConfig::new(&config1);

        let count = 1;
        input.advance_to(count);
        while probe.less_than(&count) { worker.step(); }

        let timer = ::std::time::Instant::now();

        // Establish a start of the computation.
        let elapsed_ns = timer.elapsed().to_nanos();
        config.base_time_ns = elapsed_ns as usize;

        assert!(worker.peers() < 256);

        let input_times = {
            let config = config.clone();
            move || nexmark::config::NexMarkInputTimes::new(config.clone(), duration_ns, time_dilation, peers)
        };

        let mut output_metric_collector =
            ::streaming_harness::output::default::hdrhist_timeline_collector(
                input_times(),
                0, 1_000_000_000, duration_ns - 1_000_000_000, duration_ns,
                250_000_000);

        let mut events_so_far = 0;

        let mut input_times_gen =
            ::streaming_harness::input::SyntheticInputTimeGenerator::new(input_times());

        let mut input = Some(input);

        let mut last_ns = 0;

        loop {
            let elapsed_ns = timer.elapsed().to_nanos();
            let wait_ns = last_ns;
            let target_ns = wait_ns + 100_000_000;
            last_ns = target_ns;

            output_metric_collector.acknowledge_while(
                elapsed_ns,
                |t| {
                    !probe.less_than(&(t as usize + count))
                });

            if input.is_none() {
                break;
            }

            if let Some(it) = input_times_gen.iter_until(target_ns) {
                let input = input.as_mut().unwrap();
                for _t in it {
                    input.send(
                        Event::create(
                            events_so_far,
                            &mut config));
                    events_so_far += worker.peers();
                }
                input.advance_to(target_ns as usize + count);
            } else {
                input.take().unwrap();
            }

            if input.is_some() {
                while probe.less_than(&(wait_ns as usize + count)) { worker.step(); }
            } else {
                while worker.step() { }
            }
        }

        output_metric_collector.into_inner()
    }).expect("unsuccessful execution").join().into_iter().map(|x| x.unwrap()).collect();

    if FIRST.load(std::sync::atomic::Ordering::Relaxed) {
        println!("{q}:elapsed:{:?}", start.elapsed());
    }
}
