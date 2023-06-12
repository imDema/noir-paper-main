pub fn q1<S: Scope<Timestamp = usize>>(
    input: &NexmarkInput,
    _nt: NexmarkTimer,
    scope: &mut S,
) -> Stream<S, Bid> {
    input
        .bids(scope)
        .map_in_place(|b| b.price = (b.price as f32 * 0.908) as usize)
}
