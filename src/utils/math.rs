pub fn calculate_margin_level(account_balance: f64, unrealized_pnl: f64, position_value: f64) -> f64 {
    (account_balance + unrealized_pnl) / position_value
}
