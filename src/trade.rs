use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use crate::order::Order;
pub struct Trade {
    buy_order: Order,
    sell_order: Order,
    price: Price,
    quantity: Quantity,
}