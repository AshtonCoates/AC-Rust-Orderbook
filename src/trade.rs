use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use crate::order::Order;
pub struct Trade {
    pub buy_order: Order,
    pub sell_order: Order,
    pub price: Price,
    pub quantity: Quantity,
}