use std::cmp::Ordering;

pub type Quantity = u32;
pub type OrderId = i32;

#[derive(Debug)]
pub struct Price(pub f64);

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for Price {}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => Ordering::Less,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderType {
    GTC,
    FOK,
    IOC,
    Market,
}