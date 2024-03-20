use crate::order_book::{Order, OrderBook, OrderSide, OrderType};
use crate::utils::math;

pub struct Trader {
    initial_margin: f64,
    maintenance_margin: f64,
    account_balance: f64,
    position: Option<Position>,
}

struct Position {
    size: f64,
    entry_price: f64,
}

impl Trader {
    pub fn new(initial_margin: f64, maintenance_margin: f64) -> Self {
        Trader {
            initial_margin,
            maintenance_margin,
            account_balance: 0.0,
            position: None,
        }
    }

    pub fn open_position(&mut self, size: f64, entry_price: f64) {
        let margin_required = size * entry_price * self.initial_margin;
        self.account_balance -= margin_required;
        self.position = Some(Position {
            size,
            entry_price,
        });
    }

    pub fn check_liquidation(&self, current_price: f64) -> Option<LiquidationDetails> {
        if let Some(position) = &self.position {
            let unrealized_pnl = position.size * (current_price - position.entry_price);
            let margin_level = (self.account_balance + unrealized_pnl) / (position.size * current_price);

            if margin_level < self.maintenance_margin {
                let liquidation_size = position.size;
                let liquidation_value = liquidation_size * current_price;
                return Some(LiquidationDetails {
                    size: liquidation_size,
                    value: liquidation_value,
                });
            }
        }
        None
    }
}
#[derive(Debug)]
pub struct LiquidationDetails {
    pub size: f64,
    pub value: f64,
}
