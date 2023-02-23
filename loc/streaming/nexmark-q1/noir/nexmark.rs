fn query1(events: Stream<Event, impl Operator<Event> + 'static>) {
    events
        .filter_map(filter_bid)
        .map(|mut b| {
            b.price = (b.price as f32 * 0.908) as usize;
            b
        })
        .for_each(std::mem::drop)
}
