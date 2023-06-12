pub fn q5<S: Scope<Timestamp = usize>>(
    input: &NexmarkInput,
    nt: NexmarkTimer,
    scope: &mut S,
    window_slice_count: usize,
    window_slide_ns: usize,
) -> Stream<S, usize> {
    input
        .bids(scope)
        .map(move |b| {
            (
                b.auction,
                Date::new(((*b.date_time / window_slide_ns) + 1) * window_slide_ns),
            )
        })
        // TODO: Could pre-aggregate pre-exchange, if there was reason to do so.
        .unary_frontier(
            Exchange::new(|b: &(usize, _)| b.0 as u64),
            "Q5 Accumulate",
            |capability, _info| {
                let mut cap_set = CapabilitySet::new();
                cap_set.insert(capability);

                let mut additions = HashMap::new();
                let mut deletions = HashMap::new();
                let mut accumulations = HashMap::new();

                let mut bids_buffer = vec![];

                move |input, output| {
                    input.for_each(|_time, data| {
                        data.swap(&mut bids_buffer);

                        for (auction, a_time) in bids_buffer.drain(..) {
                            additions
                                .entry(nt.from_nexmark_time(a_time))
                                .or_insert_with(Vec::new)
                                .push(auction);
                        }
                    });

                    // Extract and order times we can now process.
                    let mut times = {
                        let add_times = additions
                            .keys()
                            .filter(|t| !input.frontier.less_equal(t))
                            .cloned();
                        let del_times = deletions
                            .keys()
                            .filter(|t| !input.frontier.less_equal(t))
                            .cloned();
                        add_times.chain(del_times).collect::<Vec<_>>()
                    };
                    times.sort();
                    times.dedup();

                    for time in times.drain(..) {
                        if let Some(additions) = additions.remove(&time) {
                            for &auction in additions.iter() {
                                *accumulations.entry(auction).or_insert(0) += 1;
                            }
                            let new_time = time + (window_slice_count * window_slide_ns);
                            deletions.insert(new_time, additions);
                        }
                        if let Some(deletions) = deletions.remove(&time) {
                            for auction in deletions.into_iter() {
                                use std::collections::hash_map::Entry;
                                match accumulations.entry(auction) {
                                    Entry::Occupied(mut entry) => {
                                        *entry.get_mut() -= 1;
                                        if *entry.get_mut() == 0 {
                                            entry.remove();
                                        }
                                    }
                                    _ => panic!("entry has to exist"),
                                }
                            }
                        }
                        let time = cap_set.delayed(&time);
                        if let Some((count, auction)) =
                            accumulations.iter().map(|(&a, &c)| (c, a)).max()
                        {
                            output.session(&time).give((auction, count));
                        }
                    }
                    cap_set.downgrade(&input.frontier.frontier());
                }
            },
        )
        .unary_frontier(
            Exchange::new(|_| 0),
            "Q5 Final Accumulate",
            |capability, _info| {
                let mut cap_set = CapabilitySet::new();
                cap_set.insert(capability);

                let mut acc = HashMap::new();

                let mut in_buffer = vec![];

                move |input, output| {
                    input.for_each(|time, data| {
                        data.swap(&mut in_buffer);

                        for (auction, count) in in_buffer.drain(..) {
                            let (max_id, max_count) =
                                acc.entry(*time).or_insert_with(|| (auction, count));

                            if count > *max_count {
                                *max_id = auction;
                                *max_count = count;
                            }
                        }
                    });

                    let mut times = acc
                        .keys()
                        .filter(|t| !input.frontier.less_equal(t))
                        .cloned()
                        .collect::<Vec<_>>();
                    times.sort();
                    times.dedup();

                    for time in times.drain(..) {
                        let time = cap_set.delayed(&time);
                        if let Some((auction, _count)) = acc.remove(&time) {
                            output.session(&time).give(auction);
                        }
                    }
                    cap_set.downgrade(&input.frontier.frontier());
                }
            },
        )
}
