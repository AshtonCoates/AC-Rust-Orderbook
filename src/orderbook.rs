use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

use crate::order::{Order, OrderQueue};
use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use crate::trade::Trade;

pub struct OrderBook {
    pub(crate) buy_orders: HashMap<OrderId, Order>,
    pub(crate) sell_orders: HashMap<OrderId, Order>,

    pub(crate) bid_tree: BinaryHeap<Price>,
    pub(crate) ask_tree: BinaryHeap<Reverse<Price>>,

    bid_price_map: HashMap<Price, OrderQueue>,
    ask_price_map: HashMap<Price, OrderQueue>,

    buy_volume: Quantity,
    sell_volume: Quantity,

    pub trades: HashMap<(OrderId, OrderId), Trade>, // (buy_order_id, sell_order_id) -> Trade
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new(),

            bid_tree: BinaryHeap::new(),
            ask_tree: BinaryHeap::new(),

            bid_price_map: HashMap::new(),
            ask_price_map: HashMap::new(),

            buy_volume: 0,
            sell_volume: 0,

            trades: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            Side::Buy => {
                self.buy_orders.insert(order.id, order);
                self.bid_tree.push(order.price);
                self.bid_price_map.entry(order.price).or_insert(OrderQueue::new()).push(order.id);
                self.buy_volume += order.quantity;
            }
            Side::Sell => {
                self.sell_orders.insert(order.id, order);
                self.ask_tree.push(Reverse(order.price));
                self.ask_price_map.entry(order.price).or_insert(OrderQueue::new()).push(order.id);
                self.sell_volume += order.quantity;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Price, Quantity, OrderId, 
                       Side, OrderType};

    #[test]
    fn add_buy_order_same_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(100.0), Side::Buy);
        orderbook.add_order(order1);
        orderbook.add_order(order2);

        assert_eq!(orderbook.buy_volume, 250);
        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().0.len(), 2);
        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &1);
        assert_eq!(orderbook.bid_price_map.len(), 1);
    }

    #[test]
    fn add_buy_order_diff_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(120.0), Side::Buy);
        orderbook.add_order(order1);
        orderbook.add_order(order2);

        assert_eq!(orderbook.buy_volume, 250);
        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().0.len(), 1);
        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &1);
        assert_eq!(orderbook.bid_price_map.len(), 2);
    }

    #[test]
    fn add_sell_order_same_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Sell);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(100.0), Side::Sell);
        orderbook.add_order(order1);
        orderbook.add_order(order2);

        assert_eq!(orderbook.sell_volume, 250);
        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().0.len(), 2);
        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &1);
        assert_eq!(orderbook.ask_price_map.len(), 1);
    }

    #[test]
    fn add_sell_order_diff_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Sell);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(120.0), Side::Sell);
        orderbook.add_order(order1);
        orderbook.add_order(order2);

        assert_eq!(orderbook.sell_volume, 250);
        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().0.len(), 1);
        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &1);
        assert_eq!(orderbook.ask_price_map.len(), 2);
    }
}