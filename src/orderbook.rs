use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

use crate::order::{Order, OrderQueue};
use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use crate::trade::Trade;

pub struct OrderBook {
    pub(crate) buy_orders: HashMap<OrderId, Order>,
    pub(crate) sell_orders: HashMap<OrderId, Order>,

    pub(crate) bid_tree: BinaryHeap<OrderQueue>,
    pub(crate) ask_tree: BinaryHeap<Reverse<OrderQueue>>,

    bid_prices: HashSet<Price>,
    ask_prices: HashSet<Price>,

    pub trades: HashMap<(OrderId, OrderId), Trade>, // (buy_order_id, sell_order_id) -> Trade
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new(),

            bid_tree: BinaryHeap::new(),
            ask_tree: BinaryHeap::new(),

            bid_prices: HashSet::new(),
            ask_prices: HashSet::new(),

            trades: HashMap::new(),
        }
    }

    pub fn buy_volume(&self) -> Quantity {
        self.buy_orders.len()
    }
    
    pub fn sell_volume(&self) -> Quantity {
        self.sell_orders.len()
    }

    pub fn get_bid(&self) -> Option<&Price> {
        let bid = self.bid_tree.peek();
        match bid {
            Some(order_queue) => Some(&order_queue.1),
            None => None,
        }
    }

    pub fn get_ask(&self) -> Option<&Price> {
        let ask = self.ask_tree.peek();
        match ask {
            Some(order_queue) => Some(&order_queue.0.1),
            None => None,
        }
    }

    pub(crate) fn add_order(&mut self, order: Order) {
        let order_price = order.price.clone();
        let order_id = order.id;
        match order.side {
            Side::Buy => {
                if self.buy_orders.contains_key(&order_id) {
                    return;
                } else if self.bid_prices.contains(&order.price) {
                    for order_queue in self.bid_tree.iter_mut() {
                        if order_queue.1 == order.price {
                            order_queue.push(order_id);
                            break;
                        }
                    }
                }
                self.buy_orders.insert(order_id, order);
                if self.bid_prices.insert(order_price.clone()) {
                    let mut order_queue: OrderQueue = OrderQueue::new(order_price);
                    order_queue.push(order_id);
                    self.bid_tree.push(order_queue);
                    
                }
            }

            Side::Sell => {
                self.sell_orders.insert(order_id.clone(), order);
                if self.ask_prices.insert(order_price.clone()) {
                    let order_queue: OrderQueue = OrderQueue::new(order_price);
                    self.ask_tree.push(Reverse(order_queue));
                    
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orderbook_init() {
        let orderbook = OrderBook::new();
        assert_eq!(orderbook.buy_orders.len(), 0);
        assert_eq!(orderbook.sell_orders.len(), 0);
    }

    #[test]
    fn add_one_buy_order() {
        let mut orderbook = OrderBook::new();
        let order = Order::new(1, OrderType::GTC, 100, Price(50.0), Side::Buy);
        orderbook.add_order(order);
        assert!(orderbook.bid_prices.contains(&Price(50.0)));
        assert_eq!(orderbook.buy_volume(), 1);
        assert_eq!(orderbook.buy_orders.len(), 1)
    }

    #[test]
    fn add_one_sell_order() {
        let mut orderbook = OrderBook::new();
        let order = Order::new(1, OrderType::GTC, 100, Price(50.0), Side::Sell);
        orderbook.add_order(order);
        assert!(orderbook.ask_prices.contains(&Price(50.0)));
        assert_eq!(orderbook.sell_volume(), 1);
        assert_eq!(orderbook.sell_orders.len(), 1)
    }

    #[test]
    fn add_buy_orders_equal_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(50.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 100, Price(50.0), Side::Buy);
        orderbook.add_order(order1);
        orderbook.add_order(order2);
    }

    #[test]
    fn add_buy_orders_different_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(50.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 100, Price(60.0), Side::Buy);
        orderbook.add_order(order1);
        orderbook.add_order(order2);
    }

}