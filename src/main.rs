mod liquidation;
mod order_book;
mod utils;

fn main(){
    let initial_margin = 0.1;
    let maintenance_margin = 0.05;
    let position_size = 1000.0;
    let entry_price = 50000.0;
    let liquidation_price = 49000.0;

    let mut trader = liquidation::Trader::new(initial_margin, maintenance_margin);
    trader.open_position(position_size, entry_price);

    if let Some(liquidation_details)=trader.check_liquidation(liquidation_price){
        println!("Liquidation triggered: {:?}", liquidation_details);
    }
}
