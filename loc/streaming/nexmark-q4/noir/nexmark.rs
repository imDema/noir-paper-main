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

fn query4(events: Stream<Event, impl Operator<Event> + 'static>) {
    winning_bids(events)
        .map(|(a, b)| (a.category, b.price))
        .group_by_avg(|(category, _)| *category, |(_, price)| *price as f64)
        .unkey()
        .for_each(std::mem::drop)
}
