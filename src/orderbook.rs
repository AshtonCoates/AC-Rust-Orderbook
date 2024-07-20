use std::collections::{BinaryHeap, HashMap};

use crate::order::{Order, OrderQueue};
use crate::types::{Price, Quantity, OrderId, 
                   Side, OrderType};
use crate::trade::Trade;

pub struct OrderBook {
    pub(crate) buy_orders: HashMap<OrderId, Order>,
    pub(crate) sell_orders: HashMap<OrderId, Order>,

    pub(crate) bid_tree: BinaryHeap<Price>,
    pub(crate) ask_tree: BinaryHeap<Price>,

    pub(crate) bid_price_map: HashMap<Price, OrderQueue>,
    pub(crate) ask_price_map: HashMap<Price, OrderQueue>,

    pub buy_volume: Quantity,
    pub sell_volume: Quantity,

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

    pub fn place_order(&mut self, order: Order) -> bool {
        match order.side {
            Side::Buy => {
                self.buy_volume += order.quantity;
                match order.kind {
                    OrderType::GTC => {
                        if self.ask_tree.is_empty() {
                            self.add_order(order, false);
                            return false;
                        } else if self.get_ask().unwrap() <= &order.price {
                            let ask_price = self.get_ask().unwrap().clone();
                            let sell_order = self.ask_price_map.get_mut(&ask_price).unwrap().pop().unwrap();
                            let remaining_order = self.match_order(order, self.sell_orders.get(&sell_order).unwrap().clone());
                            if let Some(order) = remaining_order {
                                self.add_order(order, false);
                            }
                        } else {
                            self.add_order(order, false);
                            return false;
                        }
                    }
                    OrderType::FOK => {return false;}
                    OrderType::IOC => {return false;}
                    OrderType::Market => {return false;}
                }
            }
            Side::Sell => {
                self.sell_volume += order.quantity;
                match order.kind {
                    OrderType::GTC => {
                        if self.bid_tree.is_empty() {
                            self.add_order(order, false);
                            return false;
                        } else if self.get_bid().unwrap() >= &order.price {
                            let bid_price = self.get_bid().unwrap().clone();
                            let buy_order = self.bid_price_map.get_mut(&bid_price).unwrap().pop().unwrap();
                            let remaining_order = self.match_order(self.buy_orders.get(&buy_order).unwrap().clone(), order);
                            if let Some(order) = remaining_order {
                                self.add_order(order, false);
                            }
                        } else {
                            self.add_order(order, false);
                            return false;
                        }
                    }
                    OrderType::FOK => {return false;}
                    OrderType::IOC => {return false;}
                    OrderType::Market => {return false;}
                }
            }
        }
        self.clean_empty_bid();
        self.clean_empty_ask();
        return true;
    }

    fn match_order(&mut self, buy_order: Order, sell_order: Order) -> Option<Order> {
        let quantity = std::cmp::min(buy_order.quantity, sell_order.quantity);
        let price = buy_order.price;
        let trade = Trade {
            buy_order: buy_order.clone(),
            sell_order: sell_order.clone(),
            price,
            quantity,
        };
        self.trades.insert((buy_order.id, sell_order.id), trade);
        self.buy_volume -= quantity;
        self.sell_volume -= quantity;

        if buy_order.quantity == sell_order.quantity {
            self.buy_orders.remove(&buy_order.id);
            self.sell_orders.remove(&sell_order.id);
            Option::None
        } else if buy_order.quantity > sell_order.quantity {
            let remaining_quantity = buy_order.quantity - quantity;
            let remaining_order = Order {
                id: buy_order.id,
                kind: buy_order.kind,
                quantity: remaining_quantity,
                price: buy_order.price,
                side: buy_order.side,
            };
            Option::Some(remaining_order)
        } else {
            let remaining_quantity = sell_order.quantity - quantity;
            let remaining_order = Order {
                id: sell_order.id,
                kind: sell_order.kind,
                quantity: remaining_quantity,
                price: sell_order.price,
                side: sell_order.side,
            };
            Option::Some(remaining_order)
        }
    }

    // private function, place_order method is the public API
    fn add_order(&mut self, order: Order, test: bool) {
        match order.side {
            Side::Buy => {
                self.buy_orders.insert(order.id, order);
                self.bid_tree.push(order.price);
                self.bid_price_map.entry(order.price).or_insert(OrderQueue::new()).push(order.id);
                if test {
                    self.buy_volume += order.quantity; // volume is adjusted in place_order, but to test other functions we need to adjust it here
                }
            }
            Side::Sell => {
                self.sell_orders.insert(order.id, order);
                self.ask_tree.push(order.price);
                self.ask_price_map.entry(order.price).or_insert(OrderQueue::new()).push(order.id);
                if test {
                    self.sell_volume += order.quantity;
                }
            }
        }
    }

    pub fn cancel_order(&mut self, id: i32) -> bool {
        if let Some(order) = self.buy_orders.get(&id).or(self.sell_orders.get(&id)) {
            match order.side {
                Side::Buy => {
                    if let Some(order) = self.buy_orders.remove(&id) {
                        self.bid_price_map.get_mut(&order.price).unwrap().remove_order(id);
                        self.buy_volume -= order.quantity;
                        true
                    } else {
                        false
                    }
                }
                Side::Sell => {
                    if let Some(order) = self.sell_orders.remove(&id) {
                        self.ask_price_map.get_mut(&order.price).unwrap().remove_order(id);
                        self.sell_volume -= order.quantity;
                        true
                    } else {
                    false
                    }
                }
            }
        } else {
            return false;
        }
    }

    pub fn get_bid(&self) -> Option<&Price> {
        self.bid_tree.peek()
    }

    pub fn get_ask(&self) -> Option<&Price> {
        self.ask_tree.peek()
    }

    fn clean_empty_bid(&mut self) {
        while let Some(price) = self.get_bid() {
            if let Some(price) = self.bid_price_map.get(price) {
                if price.is_empty() {
                    self.bid_tree.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn clean_empty_ask(&mut self) {
        while let Some(price) = self.get_ask() {
            if let Some(price) = self.ask_price_map.get(price) {
                if price.is_empty() {
                    self.ask_tree.pop();
                } else {
                    break;
                }
            } else {
                break;
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
        orderbook.add_order(order1, true);
        orderbook.add_order(order2, true);

        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().0.len(), 2);
        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &1);
        assert_eq!(orderbook.bid_price_map.len(), 1);
    }

    #[test]
    fn add_buy_order_diff_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(120.0), Side::Buy);
        orderbook.add_order(order1, true);
        orderbook.add_order(order2, true);

        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().0.len(), 1);
        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &1);
        assert_eq!(orderbook.bid_price_map.len(), 2);
    }

    #[test]
    fn add_sell_order_same_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Sell);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(100.0), Side::Sell);
        orderbook.add_order(order1, true);
        orderbook.add_order(order2, true);

        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().0.len(), 2);
        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &1);
        assert_eq!(orderbook.ask_price_map.len(), 1);
    }

    #[test]
    fn add_sell_order_diff_price() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Sell);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(120.0), Side::Sell);
        orderbook.add_order(order1, true);
        orderbook.add_order(order2, true);

        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().0.len(), 1);
        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &1);
        assert_eq!(orderbook.ask_price_map.len(), 2);
    }

    #[test]
    fn cancel_buy_order() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(100.0), Side::Buy);
        orderbook.add_order(order1, true);
        orderbook.add_order(order2, true);

        orderbook.cancel_order(1);

        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().0.len(), 1);
        assert_eq!(orderbook.bid_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &2);
        assert_eq!(orderbook.bid_price_map.len(), 1);
    }

    #[test]
    fn cancel_sell_order() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Sell);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(100.0), Side::Sell);
        orderbook.add_order(order1, true);
        orderbook.add_order(order2, true);

        orderbook.cancel_order(1);

        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().0.len(), 1);
        assert_eq!(orderbook.ask_price_map.get(&Price(100.0)).unwrap().peek().unwrap(), &2);
        assert_eq!(orderbook.ask_price_map.len(), 1);
    }

    #[test]
    fn clean_empty_bid() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(120.0), Side::Buy);
        let order3 = Order::new(3, OrderType::GTC, 200, Price(90.0), Side::Buy);
        orderbook.add_order(order1, true);
        orderbook.add_order(order2, true);
        orderbook.add_order(order3, true);

        orderbook.cancel_order(1);

        assert_eq!(orderbook.bid_tree.len(), 3);
        orderbook.clean_empty_bid();
        assert_eq!(orderbook.bid_tree.len(), 3);
        let _ = orderbook.bid_tree.pop();
        orderbook.clean_empty_bid();
        assert_eq!(orderbook.bid_tree.len(), 1);
    }
    #[test]
    fn match_order_test() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(100.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(100.0), Side::Sell);
        orderbook.add_order(order1, true);
        orderbook.add_order(order2, true);
        orderbook.match_order(order1, order2);
        assert_eq!(orderbook.trades.len(), 1);
        assert_eq!(orderbook.trades.get(&(1, 2)).unwrap().quantity, 100);
        assert_eq!(orderbook.buy_volume, 0);
        assert_eq!(orderbook.sell_volume, 50);
        assert_eq!(orderbook.bid_tree.len(), 1); // we have not cleaned the empty bid
        assert_eq!(orderbook.ask_tree.len(), 1);

        let order3 = Order::new(3, OrderType::GTC, 50, Price(100.0), Side::Buy);
        orderbook.add_order(order3, true);
        orderbook.match_order(order3, order2);
        assert_eq!(orderbook.trades.len(), 2);
        assert_eq!(orderbook.trades.get(&(3, 2)).unwrap().quantity, 50);
        assert_eq!(orderbook.buy_volume, 0);
        assert_eq!(orderbook.sell_volume, 0);
        assert_eq!(orderbook.bid_tree.len(), 2); // we have not cleaned the empty bids
        assert_eq!(orderbook.ask_tree.len(), 1); // we have not cleaned the empty ask
    }

    #[test]
    fn place_gtc_order_and_match() {
        let mut orderbook = OrderBook::new();
        let order1 = Order::new(1, OrderType::GTC, 100, Price(110.0), Side::Buy);
        let order2 = Order::new(2, OrderType::GTC, 150, Price(100.0), Side::Sell);
        
        let result = orderbook.place_order(order1);
        assert_eq!(result, false);
        let result = orderbook.place_order(order2);
        assert_eq!(result, true);

        assert_eq!(orderbook.trades.len(), 1);
        assert_eq!(orderbook.trades.get(&(1, 2)).unwrap().quantity, 100);
        assert_eq!(orderbook.trades.get(&(1 ,2)).unwrap().price, Price(110.0));
        assert_eq!(orderbook.buy_volume, 0);
        assert_eq!(orderbook.sell_volume, 50); 
        assert_eq!(orderbook.bid_tree.len(), 0);
        assert_eq!(orderbook.ask_tree.len(), 0);
    }
}