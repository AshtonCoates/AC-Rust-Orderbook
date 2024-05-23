use crate::order::Order;
use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use crate::trade::Trade;

pub struct OrderBook {
    buy_orders: Vec<Order>,
    sell_orders: Vec<Order>,
    ask: Price, // lowest ask
    bid: Price, // highest bid
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
            ask: 0.0,
            bid: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::orderbook::OrderBook;

    #[test]
    fn create_orderbook() {
        OrderBook::new();
        assert_eq!(1, 1);
    }
}