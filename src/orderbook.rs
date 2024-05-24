use crate::order::{Order, OrderQueue};
use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use crate::trade::Trade;

pub struct OrderBook {
    pub buy_orders: OrderQueue,
    pub sell_orders: OrderQueue,
    pub ask: Price, // lowest ask
    pub bid: Price, // highest bid
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: OrderQueue::new(),
            sell_orders: OrderQueue::new(),
            ask: Price(0.0),
            bid: Price(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::orderbook::OrderBook;
}