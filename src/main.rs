pub mod exchange;
pub mod model;
pub mod plotter;
pub mod repository;
pub mod synchronizer;
pub mod utils;
use ifmt::iprintln;
use repository::Repository;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();

    let mut i: Decimal = 1u32.into();
    i += dec!(1);

    let repo = Repository::new()?;

    let last_id = repo.last_id();
    let last_close_time = repo.last_close_time("BTCUSDT");

    iprintln!("last id: {last_id} close_time: {last_close_time:?}");

    let candle_opt = repo.candle_by_id(last_id);

    iprintln!("{candle_opt:?}");

    //assert_eq!(row.0, 150);
    // https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/src/main.rs

    Ok(())
}
