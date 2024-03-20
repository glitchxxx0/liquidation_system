use crate::order_book::{Order, OrderSide};
use std::collections::BTreeMap;

pub struct OrderBook {
    buy_orders: BTreeMap<f64, f64>,
    sell_orders: BTreeMap<f64, f64>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            OrderSide::Buy => self.add_buy_order(order),
            OrderSide::Sell => self.add_sell_order(order),
        }
    }

    fn add_buy_order(&mut self, order: Order) {
    }

    fn add_sell_order(&mut self, order: Order) {
    }

    pub fn execute_liquidation(&mut self, liquidation_order: Order) {
    }
}
