pub mod analyzers;
pub mod exchange;
pub mod model;
pub mod plotter;
pub mod repository;
pub mod synchronizer;
pub mod utils;
// use clap::App;
use analyzers::macd_tac::MacdTac;
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
        .subcommand(App::new("analyzer").about("analyzer"))
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

    if let Some(_analyzer) = matches.subcommand_matches("analyzer") {
        let candles = repo.candles_default("BTCUSDT", &15);

        let candles_ref: Vec<_> = candles.iter().collect();
        let analyzer = MacdTac::new(candles_ref.as_slice());
        let tacs = analyzer.run();
        plotter::plot_tecals("BTCUSDT", &15, &tacs).unwrap();
    }

    //assert_e!(row.0, 150);
    // https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/src/main.rs

    Ok(())
}
