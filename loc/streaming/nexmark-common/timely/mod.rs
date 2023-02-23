pub struct NexmarkInput<'a> {
    pub control: &'a Rc<EventLink<usize, Control>>,
    pub bids: &'a Rc<EventLink<usize, Bid>>,
    pub auctions: &'a Rc<EventLink<usize, Auction>>,
    pub people: &'a Rc<EventLink<usize, Person>>,
    pub closed_auctions: &'a Rc<EventLink<usize, (Auction, Bid)>>,
    pub closed_auctions_flex: &'a Rc<EventLink<usize, (Auction, Bid)>>,
}

impl<'a> NexmarkInput<'a> {
    pub fn control<S: Scope<Timestamp=usize>>(&self, scope: &mut S) -> Stream<S, Control> {
        Some(self.control.clone()).replay_into(scope)
    }

    pub fn bids<S: Scope<Timestamp=usize>>(&self, scope: &mut S) -> Stream<S, Bid> {
        Some(self.bids.clone()).replay_into(scope)
    }

    pub fn auctions<S: Scope<Timestamp=usize>>(&self, scope: &mut S) -> Stream<S, Auction> {
        Some(self.auctions.clone()).replay_into(scope)
    }

    pub fn people<S: Scope<Timestamp=usize>>(&self, scope: &mut S) -> Stream<S, Person> {
        Some(self.people.clone()).replay_into(scope)
    }

    pub fn closed_auctions<S: Scope<Timestamp=usize>>(&self, scope: &mut S) -> Stream<S, (Auction, Bid)> {
        Some(self.closed_auctions.clone()).replay_into(scope)
    }

    pub fn closed_auctions_flex<S: Scope<Timestamp=usize>>(&self, scope: &mut S) -> Stream<S, (Auction, Bid)> {
        Some(self.closed_auctions_flex.clone()).replay_into(scope)
    }
}
