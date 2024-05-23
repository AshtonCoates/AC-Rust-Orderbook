
pub type Price = f64;
pub type Quantity = u32;
pub type OrderId = i32;

pub enum Side {
    Buy,
    Sell,
}

pub enum OrderType {
    GTC,
    FOK,
    IOC,
    Market,
}