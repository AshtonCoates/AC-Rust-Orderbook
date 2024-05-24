use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use std::collections::VecDeque;
use std::cmp::Ordering;

/// Order is a struct that represents an order.
/// An order has an id, a type, a quantity, a price, and a side (buy or sell).
#[derive(Debug)]
pub struct Order {
    pub id: OrderId,
    pub kind: OrderType,
    pub quantity: Quantity,
    pub price: Price,
    pub side: Side,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.kind == other.kind && self.quantity == other.quantity && self.price == other.price && self.side == other.side
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.price == other.price {
            Some(self.id.cmp(&other.id))
        } else {
            Some(self.price.cmp(&other.price))
        }
    }
}

/// OrderQueue is a queue of orders with the same price.
/// It is used to store orders with the same price in the order book.
/// The order queue contains a double ended queue, but only implements methods of a queue.
/// The order queue is ordered by the time the orders were added to the queue.
#[derive(Debug)]
pub(crate) struct OrderQueue(VecDeque<Order>);

impl OrderQueue {
    pub(crate) fn new() -> OrderQueue {
        OrderQueue(VecDeque::new())
    }
    pub(crate) fn push(&mut self, order: Order) {
        self.0.push_back(order);
    }
    pub(crate) fn pop(&mut self) -> Option<Order> {
        self.0.pop_front()
    }
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl PartialEq for OrderQueue {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_empty() && other.0.is_empty() {
            return true;
        }
        let self_price = &self.0.front().unwrap().price;
        let other_price = &other.0.front().unwrap().price;
        self_price == other_price
    }

}

impl PartialOrd for OrderQueue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.is_empty() && other.0.is_empty() {
            return Some(Ordering::Equal);
        }
        let self_price = &self.0.front().unwrap().price;
        let other_price = &other.0.front().unwrap().price;
        self_price.partial_cmp(&other_price)
    }
}

impl Eq for OrderQueue {}

impl Ord for OrderQueue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => Ordering::Less,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::order::{Order, OrderQueue};
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
        let _orderqueue: OrderQueue = OrderQueue::new();
    }

    #[test]
    fn orderqueue_eq() {
        let orderqueue1: OrderQueue = OrderQueue::new();
        let orderqueue2: OrderQueue = OrderQueue::new();
        assert_eq!(orderqueue1, orderqueue2);
    }

    #[test]
    fn orderqueue_partial_cmp() {
        let orderqueue1: OrderQueue = OrderQueue::new();
        let orderqueue2: OrderQueue = OrderQueue::new();
        assert_eq!(orderqueue1.partial_cmp(&orderqueue2), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn orderqueue_cmp() {
        let orderqueue1: OrderQueue = OrderQueue::new();
        let orderqueue2: OrderQueue = OrderQueue::new();
        assert_eq!(orderqueue1.cmp(&orderqueue2), std::cmp::Ordering::Equal);
    }

    #[test]
    fn orderqueue_push() {
        let mut orderqueue: OrderQueue = OrderQueue::new();
        let order: Order = Order {
            id: 1,
            kind: OrderType::GTC,
            quantity: 100,
            price: Price(100.0),
            side: Side::Buy,
        };
        orderqueue.push(order);
        assert_eq!(1, orderqueue.0.len());
    }

    #[test]
    fn orderqueue_pop() {
        let mut orderqueue: OrderQueue = OrderQueue::new();
        let order: Order = Order {
            id: 1,
            kind: OrderType::GTC,
            quantity: 100,
            price: Price(100.0),
            side: Side::Buy,
        };
        orderqueue.push(order);
        let popped_order = orderqueue.pop();
        assert_eq!(None, orderqueue.pop());
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