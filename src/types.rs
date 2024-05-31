use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub type Quantity = usize;
pub type OrderId = i32;

#[derive(Debug, Clone, Copy)]
pub struct Price(pub f64);

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for Price {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert the f64 to a format that can be hashed
        let price_as_int = (self.0 * 100.0) as i64;
        price_as_int.hash(state);
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum OrderType {
    GTC,
    FOK,
    IOC,
    Market,
}