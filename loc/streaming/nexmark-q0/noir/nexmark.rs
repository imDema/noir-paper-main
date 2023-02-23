fn query0(events: Stream<Event, impl Operator<Event> + 'static>) {
    events.for_each(std::mem::drop)
}
