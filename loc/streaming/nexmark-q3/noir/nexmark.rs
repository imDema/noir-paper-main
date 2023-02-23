fn query3(events: Stream<Event, impl Operator<Event> + 'static>) {
    let mut routes = events
        .route()
        .add_route(|e| matches!(e, Event::Person(_)))
        .add_route(|e| matches!(e, Event::Auction(_)))
        .build()
        .into_iter();
    let person = routes
        .next()
        .unwrap()
        .map(unwrap_person)
        .filter(|p| p.state == "or" || p.state == "id" || p.state == "ca");
    let auction = routes
        .next()
        .unwrap()
        .map(unwrap_auction)
        .filter(|a| a.category == 10);
    person
        .join(auction, |p| p.id, |a| a.seller)
        .drop_key()
        .map(|(p, a)| (p.name, p.city, p.state, a.id))
        .for_each(std::mem::drop)
}
