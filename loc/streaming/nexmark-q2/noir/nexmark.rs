fn query2(events: Stream<Event, impl Operator<Event> + 'static>) {
    events
        .filter_map(filter_bid)
        .filter(|b| b.auction % 123 == 0)
        .for_each(std::mem::drop)
}
