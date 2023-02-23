pub fn q2<S: Scope<Timestamp = usize>>(
    input: &NexmarkInput,
    _nt: NexmarkTimer,
    scope: &mut S,
) -> Stream<S, (usize, usize)> {
    let auction_skip = 123;
    input
        .bids(scope)
        .filter(move |b| b.auction % auction_skip == 0)
        .map(|b| (b.auction, b.price))
}
