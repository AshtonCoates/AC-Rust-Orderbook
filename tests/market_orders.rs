use ac_rust_orderbook::types::{Price, Side, OrderType};
use ac_rust_orderbook::order::{self, Order};
use ac_rust_orderbook::orderbook::{OrderBook};

#[test]
fn buyside_market_order() {
    let mut orderbook: OrderBook = OrderBook::new();
    let buy_order: Order = Order {
        id: 1,
        kind: OrderType::Market,
        quantity: 100,
        price: Price(100.0),
        side: Side::Buy,
    };
    let sell_order: Order = Order {
        id: 2,
        kind: OrderType::GTC,
        quantity: 100,
        price: Price(100.0),
        side: Side::Sell,
    };
    orderbook.place_order(sell_order);

    assert_eq!(0, orderbook.trades.len());
    assert_eq!(100, orderbook.sell_volume);

    orderbook.place_order(buy_order);

    assert_eq!(1, orderbook.trades.len());
    assert_eq!(0, orderbook.buy_volume);
}

#[test]
fn sellside_market_order() {
    let mut orderbook: OrderBook = OrderBook::new();
    let buy_order: Order = Order {
        id: 1,
        kind: OrderType::GTC,
        quantity: 100,
        price: Price(100.0),
        side: Side::Buy,
    };
    let sell_order: Order = Order {
        id: 2,
        kind: OrderType::Market,
        quantity: 100,
        price: Price(0.0),
        side: Side::Sell,
    };
    assert!(orderbook.place_order(buy_order));
    assert!(orderbook.place_order(sell_order));

    assert_eq!(1, orderbook.trades.len());
    assert_eq!(0, orderbook.sell_volume);
    assert_eq!(0, orderbook.buy_volume);
    assert_eq!(100, orderbook.trades.get(&(1, 2)).unwrap().quantity);
    assert_eq!(Price(100.0), orderbook.trades.get(&(1, 2)).unwrap().price);
}