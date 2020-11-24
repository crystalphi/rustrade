pub mod analyzers;
pub mod config;
pub mod exchange;
pub mod model;
pub mod repository;
pub mod synchronizer;
pub mod tac_plotters;
pub mod technicals;
pub mod utils;
// use clap::App;
use clap::App;
use exchange::Exchange;
use repository::Repository;
use synchronizer::Synchronizer;
use tac_plotters::plotter::plot_candles;
use technicals::{macd::macd_tac::MacdTac, pivots::PivotTac};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Rustrade")
        .version("1.0")
        .about("Rustrade")
        .subcommand(App::new("sync").about("sync candles"))
        .subcommand(App::new("check").about("check insconsist"))
        .subcommand(App::new("fix").about("fix insconsist"))
        .subcommand(App::new("list").about("list last"))
        .subcommand(App::new("plot").about("plot"))
        .subcommand(App::new("stream").about("stream"))
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

    if let Some(_plot) = matches.subcommand_matches("plot") {
        let candles = repo.candles_default("BTCUSDT", &15);

        let candles_ref: Vec<_> = candles.iter().collect();
        let macd_tac = MacdTac::new(candles_ref.as_slice());

        let pivots = PivotTac::new(candles_ref.as_slice()).pivots();

        plot_candles("BTCUSDT", &15, &tacs, &pivots).unwrap();
    }

    if let Some(_stream) = matches.subcommand_matches("stream") {
        read_stream();
    }

    //assert_e!(row.0, 150);
    // https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/src/main.rs

    Ok(())
}

fn read_stream() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", line);
}
