fn query7(events: Stream<Event, impl Operator<Event> + 'static>) {
    let bid = events
        .add_timestamps(timestamp_gen, {
            let mut count = 0;
            move |_, ts| watermark_gen(ts, &mut count, WATERMARK_INTERVAL)
        })
        .filter_map(filter_bid);
    let window_descr = EventTimeWindow::tumbling(10 * SECOND_MILLIS);
    bid.map(|b| (b.auction, b.price, b.bidder))
        .key_by(|_| ())
        .window(window_descr.clone())
        .max_by_key(|(_, price, _)| *price)
        .drop_key()
        .window_all(window_descr)
        .max_by_key(|(_, price, _)| *price)
        .for_each(std::mem::drop)
}
