fn query5(events: Stream<Event, impl Operator<Event> + 'static>) {
    let window_descr = EventTimeWindow::sliding(10 * SECOND_MILLIS, 2 * SECOND_MILLIS);
    let bid = events
        .add_timestamps(timestamp_gen, {
            let mut count = 0;
            move |_, ts| watermark_gen(ts, &mut count, WATERMARK_INTERVAL)
        })
        .filter_map(filter_bid);

    // count how bids in each auction, for every window
    let counts = bid
        .map(|b| b.auction)
        .group_by(|a| *a)
        .map(|_| ())
        .window(window_descr.clone())
        .count()
        .unkey();
    counts
        .window_all(window_descr)
        .max_by_key(|(_, v)| *v)
        .for_each(std::mem::drop)
}
