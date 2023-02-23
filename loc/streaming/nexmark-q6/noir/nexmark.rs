fn winning_bids(
    events: Stream<Event, impl Operator<Event> + 'static>,
) -> Stream<(Auction, Bid), impl Operator<(Auction, Bid)>> {
    events
        .filter(|e| matches!(e, Event::Auction(_) | Event::Bid(_)))
        .add_timestamps(timestamp_gen, {
            let mut count = 0;
            move |_, ts| watermark_gen(ts, &mut count, WATERMARK_INTERVAL)
        })
        .group_by(|e| match e {
            Event::Auction(a) => a.id,
            Event::Bid(b) => b.auction,
            _ => unreachable!(),
        })
        .window(TransactionWindow::new(|e| match e {
            Event::Auction(a) => TransactionCommand::CommitAfter(a.expires as i64),
            _ => TransactionCommand::None,
        }))
        .fold(
            (None, Vec::new()),
            |(auction, bids): &mut (Option<Auction>, Vec<Bid>), e| match e {
                Event::Auction(a) => {
                    let winner = bids
                        .drain(..)
                        .filter(|b| {
                            b.price >= a.reserve && (a.date_time..a.expires).contains(&b.date_time)
                        })
                        .max_by_key(|b| b.price);

                    *auction = Some(a);
                    bids.extend(winner);
                    bids.shrink_to(1);
                }
                Event::Bid(b) => {
                    if let Some(a) = auction {
                        if b.price >= a.reserve
                            && (a.date_time..a.expires).contains(&b.date_time)
                            && bids.first().map(|w| b.price > w.price).unwrap_or(true)
                        {
                            bids.truncate(0);
                            bids.push(b);
                        }
                    } else {
                        bids.push(b);
                    }
                }
                _ => unreachable!(),
            },
        )
        .drop_key()
        .filter_map(|(auction, mut bid)| Some((auction.unwrap(), bid.pop()?)))
}

fn query6(events: Stream<Event, impl Operator<Event> + 'static>) {
    winning_bids(events)
        // [PARTITION BY A.seller ROWS 10]
        .map(|(a, b)| (a.seller, b.price))
        .group_by(|(seller, _)| *seller)
        .window(CountWindow::sliding(10, 1))
        // AVG(Q.final)
        .fold((0, 0), |(sum, count), (_, price)| {
            *sum += price;
            *count += 1;
        })
        .map(|(_, (sum, count))| sum as f32 / count as f32)
        .unkey()
        .for_each(std::mem::drop)
}
