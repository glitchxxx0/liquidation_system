#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Market,
    Limit(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub side: OrderSide,
    pub order_type: OrderType,
    pub size: f64,
}
