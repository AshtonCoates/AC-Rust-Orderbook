use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use std::collections::VecDeque;
use std::cmp::Ordering;

/// Order is a struct that represents an order.
/// An order has an id, a type, a quantity, a price, and a side (buy or sell).
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Order {
    pub id: OrderId,
    pub kind: OrderType,
    pub quantity: Quantity,
    pub price: Price,
    pub side: Side,
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.price == other.price {
            Some(self.id.cmp(&other.id))
        } else {
            Some(self.get_heap_val().cmp(&other.get_heap_val()))
        }
    }
}

impl Order {
    //! Create a new order
    pub fn new(id: OrderId, kind: OrderType, quantity: Quantity, price: Price, side: Side) -> Order {
        Order {
            id: id,
            kind: kind,
            quantity: quantity,
            price: price,
            side: side,
        }
    }

    pub fn get_heap_val(&self) -> Price {
        if self.side == Side::Buy {
            self.price
        } else {
            Price(-self.price.0)
        }
    }
}

/// OrderQueue is a queue of orders with the same price.
/// It is used to store orders with the same price in the order book.
/// The order queue contains a double ended queue, but only implements methods of a queue.
/// The order queue is ordered by the time the orders were added to the queue.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct OrderQueue(pub(crate) VecDeque<OrderId>);

impl OrderQueue {

    pub(crate) fn new() -> OrderQueue {
        OrderQueue(VecDeque::new())
    }

    pub(crate) fn push(&mut self, id: OrderId) {
        self.0.push_back(id);
    }

    pub(crate) fn pop(&mut self) -> Option<OrderId> {
        self.0.pop_front()
    }

    pub(crate) fn peek(&self) -> Option<&OrderId> {
        self.0.front()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn remove_order(&mut self, order_id: OrderId) {
        self.0.retain(|&id| id != order_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Price, Quantity, OrderId, 
                       Side, OrderType};

    #[test]
    fn create_order() {
        let order: Order = Order {
            id: 1,
            kind: OrderType::GTC,
            quantity: 100,
            price: Price(100.0),
            side: Side::Buy,
        };
        assert_eq!(1, order.id);
    }

    #[test]
    fn create_orderqueue() {
        let _orderqueue_1: OrderQueue = OrderQueue::new();
    }


    #[test]
    fn orderqueue_push() {
        let mut orderqueue: OrderQueue = OrderQueue::new();
        orderqueue.push(1);
        assert_eq!(1, orderqueue.len());
    }

    #[test]
    fn orderqueue_pop() {
        let mut orderqueue: OrderQueue = OrderQueue::new();
        orderqueue.push(1);
        let popped_order: Option<OrderId> = orderqueue.pop();
        assert_eq!(None, orderqueue.pop());
        assert_eq!(1, popped_order.unwrap());
    }

    #[test]
    fn order_eq() {
        let order1: Order = Order {
            id: 1,
            kind: OrderType::GTC,
            quantity: 100,
            price: Price(100.0),
            side: Side::Buy,
        };
        let order2: Order = Order {
            id: 1,
            kind: OrderType::GTC,
            quantity: 100,
            price: Price(100.0),
            side: Side::Buy,
        };
        assert_eq!(order1, order2);
    }
}