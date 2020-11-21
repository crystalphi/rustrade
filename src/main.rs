pub mod exchange;
pub mod model;
pub mod plotter;
pub mod repository;
pub mod synchronizer;
pub mod utils;
// use clap::App;
use clap::App;
use exchange::Exchange;
use repository::Repository;
use synchronizer::Synchronizer;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Rustrade")
        .version("1.0")
        .about("Rustrade")
        .subcommand(App::new("sync").about("sync candles"))
        .subcommand(App::new("check").about("check insconsist"))
        .subcommand(App::new("fix").about("fix insconsist"))
        .subcommand(App::new("list").about("list last"))
        .get_matches();

    dotenv::dotenv().unwrap();

    let exchange = Exchange::new()?;
    let repo = Repository::new()?;

    let synchronizer = Synchronizer::new("BTCUSDT", &15, &repo, &exchange);

    if let Some(_sync) = matches.subcommand_matches("sync") {
        synchronizer.synchronize();
    }

    if let Some(_sync) = matches.subcommand_matches("check") {
        synchronizer.check_inconsist();
    }

    if let Some(_fix) = matches.subcommand_matches("fix") {
        synchronizer.delete_inconsist();
    }

    if let Some(_list) = matches.subcommand_matches("list") {
        repo.list_candles("BTCUSDT", &15, &10);
    }

    //assert_e!(row.0, 150);
    // https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/src/main.rs

    Ok(())
}
