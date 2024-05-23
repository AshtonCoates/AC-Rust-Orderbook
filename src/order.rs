use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};

pub struct Order {
    id: OrderId,
    kind: OrderType,
    quantity: Quantity,
    price: Price,
    side: Side,
}