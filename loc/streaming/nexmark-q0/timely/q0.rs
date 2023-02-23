pub fn q1<S: Scope<Timestamp = usize>>(
    input: &NexmarkInput,
    _nt: NexmarkTimer,
    scope: &mut S,
) -> Stream<S, Bid> {
    input
}
