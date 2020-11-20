pub mod model;
pub mod repository;

use ifmt::iprintln;
use repository::Repository;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();

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
