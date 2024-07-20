use ac_rust_orderbook::types::{Price, Side, OrderType};
use ac_rust_orderbook::order::Order;
use ac_rust_orderbook::orderbook::OrderBook;

#[test]
fn place_matching_gtc_order() {
    let mut orderbook = OrderBook::new();

    let order1 = Order::new(1, OrderType::GTC, 100, Price(30.0), Side::Buy);
    let order2 = Order::new(2, OrderType::GTC, 100, Price(29.0), Side::Buy);
    let order3 = Order::new(3, OrderType::GTC, 100, Price(28.0), Side::Buy);
    let order4 = Order::new(4, OrderType::GTC, 100, Price(27.0), Side::Buy);
    let order5 = Order::new(5, OrderType::GTC, 100, Price(26.0), Side::Buy);
    orderbook.place_order(order1);
    orderbook.place_order(order2);
    orderbook.place_order(order3);
    orderbook.place_order(order4);
    orderbook.place_order(order5);

    let order6 = Order::new(1, OrderType::GTC, 100, Price(31.0), Side::Sell);
    let order7 = Order::new(2, OrderType::GTC, 100, Price(32.0), Side::Sell);
    let order8 = Order::new(3, OrderType::GTC, 100, Price(33.0), Side::Sell);
    let order9 = Order::new(4, OrderType::GTC, 100, Price(34.0), Side::Sell);
    orderbook.place_order(order6);
    orderbook.place_order(order7);
    orderbook.place_order(order8);
    orderbook.place_order(order9);

    assert_eq!(orderbook.buy_volume, 500);
    assert_eq!(orderbook.sell_volume, 400);
    
    let order10 = Order::new(5, OrderType::GTC, 50, Price(29.0), Side::Sell);
    orderbook.place_order(order10);

    assert_eq!(orderbook.buy_volume, 450);
    assert_eq!(orderbook.get_bid().unwrap(), &Price(30.0));
    assert_eq!(orderbook.trades.len(), 1);
    assert_eq!(orderbook.trades.get(&(1, 10)).unwrap().quantity, 50);
    assert_eq!(orderbook.trades.get(&(1, 10)).unwrap().price, Price(30.0));
}