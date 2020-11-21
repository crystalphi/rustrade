pub mod exchange;
pub mod model;
pub mod plotter;
pub mod repository;
pub mod synchronizer;
pub mod utils;
use exchange::Exchange;
use repository::Repository;
use synchronizer::Synchronizer;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();

    let exchange = Exchange::new()?;
    let repo = Repository::new()?;

    let synchronizer = Synchronizer::new(repo, exchange);

    synchronizer.synchronize("BTCUSDT");
    //assert_e!(row.0, 150);
    // https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/src/main.rs

    Ok(())
}
