pub fn q4_q6_common<S: Scope<Timestamp=usize>>(input: &NexmarkInput, nt: NexmarkTimer, scope: &mut S) -> Stream<S, (Auction, Bid)>
{
    let bids = input.bids(scope);
    let auctions = input.auctions(scope);

    bids.binary_frontier(
        &auctions,
        Exchange::new(|b: &Bid| b.auction as u64),
        Exchange::new(|a: &Auction| a.id as u64),
        "Q4 Auction close",
        |_capability, _info| {
            let mut state: HashMap<_, (Option<_>, Vec<Bid>)> = std::collections::HashMap::new();
            let mut opens = std::collections::BinaryHeap::new();

            let mut capability: Option<Capability<usize>> = None;
            use std::collections::hash_map::Entry;
            use std::cmp::Reverse;

            fn is_valid_bid(bid: &Bid, auction: &Auction) -> bool {
                bid.price >= auction.reserve && auction.date_time <= bid.date_time && bid.date_time < auction.expires
            }

            move |input1, input2, output| {
                input1.for_each(|time, data| {
                    for bid in data.iter().cloned() {
                        let entry = state.entry(bid.auction).or_insert((None, Vec::new()));
                        if let Some(ref auction) = entry.0 {
                            debug_assert!(entry.1.len() <= 1);
                            if is_valid_bid(&bid, auction) {
                                if let Some(existing) = entry.1.get(0).cloned() {
                                    if existing.price < bid.price {
                                        entry.1[0] = bid;
                                    }
                                } else {
                                    entry.1.push(bid);
                                }
                            }
                        } else {
                            opens.push((Reverse(bid.date_time), bid.auction));
                            if capability.as_ref().map(|c| nt.to_nexmark_time(*c.time()) <= bid.date_time) != Some(true) {
                                capability = Some(time.delayed(&nt.from_nexmark_time(bid.date_time)));
                            }
                            entry.1.push(bid);
                        }
                    }
                });

                input2.for_each(|time, data| {
                    for auction in data.iter().cloned() {
                        if capability.as_ref().map(|c| nt.to_nexmark_time(*c.time()) <= auction.expires) != Some(true) {
                            capability = Some(time.delayed(&nt.from_nexmark_time(auction.expires)));
                        }
                        opens.push((Reverse(auction.expires), auction.id));
                        let mut entry = state.entry(auction.id).or_insert((None, Vec::new()));
                        debug_assert!(entry.0.is_none());
                        entry.1.retain(|bid| is_valid_bid(&bid, &auction));
                        if let Some(bid) = entry.1.iter().max_by_key(|bid| bid.price).cloned() {
                            entry.1.clear();
                            entry.1.push(bid);
                        }
                        entry.0 = Some(auction);
                    }
                });

                if let Some(ref capability) = capability {
                    let complete1 = input1.frontier.frontier().get(0).cloned().unwrap_or(usize::max_value());
                    let complete2 = input2.frontier.frontier().get(0).cloned().unwrap_or(usize::max_value());
                    let complete = std::cmp::min(complete1, complete2);

                    let mut session = output.session(capability);
                    while opens.peek().map(|x| complete == usize::max_value() || (x.0).0 < nt.to_nexmark_time(complete)) == Some(true) {
                        let (Reverse(time), auction) = opens.pop().unwrap();
                        let entry = state.entry(auction);
                        if let Entry::Occupied(mut entry) = entry {
                            let delete = {
                                let auction_bids = entry.get_mut();
                                if let Some(ref auction) = auction_bids.0 {
                                    if time == auction.expires {
                                        // Auction expired, clean up state
                                        if let Some(winner) = auction_bids.1.pop() {
                                            session.give((auction.clone(), winner));
                                        }
                                        true
                                    } else {
                                        false
                                    }
                                } else {
                                    auction_bids.1.retain(|bid| bid.date_time > time);
                                    auction_bids.1.is_empty()
                                }
                            };
                            if delete {
                                entry.remove_entry();
                            }
                        }
                    }
                }

                if let Some(head) = opens.peek() {
                    capability.as_mut().map(|c| c.downgrade(&nt.from_nexmark_time((head.0).0)));
                } else {
                    capability = None;
                }
            }
        }
    )
}

pub fn q6<S: Scope<Timestamp = usize>>(
    input: &NexmarkInput,
    _nt: NexmarkTimer,
    scope: &mut S,
) -> Stream<S, (usize, f32)> {
    input
        .closed_auctions(scope)
        .map(|(_a, b)| (b.bidder, b.price))
        .unary(
            Exchange::new(|x: &(usize, usize)| x.0 as u64),
            "Q6 Average",
            |_cap, _info| {
                // Store bidder -> [prices; 10]
                let mut state = std::collections::HashMap::new();

                move |input, output| {
                    input.for_each(|time, data| {
                        let mut session = output.session(&time);
                        for (bidder, price) in data.iter().cloned() {
                            let entry = state.entry(bidder).or_insert_with(VecDeque::new);
                            if entry.len() >= 10 {
                                entry.pop_back();
                            }
                            entry.push_front(price);
                            let sum: usize = entry.iter().sum();
                            session.give((bidder, sum as f32 / entry.len() as f32));
                        }
                    });
                }
            },
        )
}
