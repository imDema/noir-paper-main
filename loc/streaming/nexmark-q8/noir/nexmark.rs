fn query8(events: Stream<Event, impl Operator<Event> + 'static>) {
    let window_descr = EventTimeWindow::tumbling(10 * SECOND_MILLIS);

    let mut routes = events
        .add_timestamps(timestamp_gen, {
            let mut count = 0;
            move |_, ts| watermark_gen(ts, &mut count, WATERMARK_INTERVAL)
        })
        .route()
        .add_route(|e| matches!(e, Event::Person(_)))
        .add_route(|e| matches!(e, Event::Auction(_)))
        .build()
        .into_iter();

    let person = routes
        .next()
        .unwrap()
        .map(unwrap_person)
        .map(|p| (p.id, p.name));
    let auction = routes
        .next()
        .unwrap()
        .map(unwrap_auction)
        .map(|a| (a.seller, a.reserve));

    person
        .group_by(|(id, _)| *id)
        .window_join(window_descr, auction.group_by(|(seller, _)| *seller))
        .drop_key()
        .map(|((id, name), (_, reserve))| (id, name, reserve))
        .for_each(std::mem::drop)
}
