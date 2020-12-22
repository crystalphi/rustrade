pub mod analyzers;
pub mod application;
pub mod candles_range;
pub mod checker;
mod config;
mod exchange;
mod model;
mod repository;
mod strategy;
mod tac_plotters;
mod technicals;
mod utils;
use application::{app::Application, candles_provider::CandlesProvider};
use checker::Checker;
use config::{candles_selection::CandlesSelection, symbol_minutes::SymbolMinutes};
use exchange::Exchange;
use ifmt::iformat;
use log::{info, LevelFilter};
use repository::Repository;
use std::time::Instant;
use strategy::pivots_triangle::pivots_triangle;
use structopt::StructOpt;
use tac_plotters::plotter::plot_candles;
use technicals::{macd::macd_tac::MacdTac, pivots::PivotTac};
use utils::str_to_datetime;
#[derive(Debug, StructOpt)]
#[structopt(about = "Commands")]
enum Command {
    /// Check content
    Check {},
    /// Synchronize
    Sync {},
    /// Fix records
    Fix {},
    /// List  
    List {},
    /// Import from excange
    Import {},
    /// Plot graph
    Plot {},
    /// Triangle
    Triangle {},
    /// Interative stream
    Stream {},
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rustrade", about = "A Rust Bot Trade")]
struct Opt {
    /// Symbol (e.g. BTCUST)
    #[structopt(short = "y", long, default_value = "BTCUSDT")]
    symbol: String,
    /// Minutes (e.g. 15)
    #[structopt(short, long, default_value = "15")]
    minutes: u32,
    /// Start date time
    #[structopt(short, long, default_value = "2020-11-01 00:00:00")]
    start_time: String,
    /// End date time
    #[structopt(short, long, default_value = "2020-12-01 00:00:00")]
    end_time: String,
    #[structopt(subcommand)]
    command: Command,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    ri_lib_log_utils::setup_log(LevelFilter::Debug);

    dotenv::dotenv().unwrap();
    let exchange: Exchange = Exchange::new().unwrap();
    let repo: Repository = Repository::new().unwrap();
    let candles_selection = CandlesSelection::new(
        &opt.symbol,
        &opt.minutes,
        Some(&str_to_datetime(&opt.start_time)),
        Some(&str_to_datetime(&opt.end_time)),
    );
    let symbol_minutes = SymbolMinutes::new(&opt.symbol, &opt.minutes);
    let synchronizer = Checker::new(&symbol_minutes, &repo, &exchange);

    match opt.command {
        Command::Check {} => {
            synchronizer.check_inconsist(&repo, &candles_selection);
        }
        Command::Sync {} => {
            synchronizer.synchronize().unwrap();
        }
        Command::Fix {} => {
            synchronizer.delete_inconsist();
        }
        Command::List {} => {
            repo.list_candles(&opt.symbol, &opt.minutes, &10);
        }
        Command::Plot {} => plot(&repo, &exchange, &candles_selection),
        Command::Stream {} => {
            read_stream(Application::new(&repo, &exchange, &synchronizer, &candles_selection));
        }
        Command::Import {} => {}
        Command::Triangle {} => triangle(&repo, &exchange, &candles_selection),
    };
    info!("Exiting program");
    //assert_e!(row.0, 150);
    // https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/src/main.rs

    Ok(())
}

fn plot(repo: &Repository, exchange: &Exchange, candles_selection: &CandlesSelection) {
    let start = Instant::now();
    info!("Loading...");
    let mut candles_provider = CandlesProvider::new(repo, exchange);
    let selection = Application::selection_factory(candles_selection.clone());
    let candles = candles_provider.candles_selection(&selection).unwrap();
    let candles = candles.iter().collect::<Vec<_>>();
    let candles = candles.as_slice();
    info!("{}", iformat!("Loaded {start.elapsed():?}"));

    let start = Instant::now();
    info!("Tacing...");
    let macd_tac = MacdTac::new(candles);
    let pivots = PivotTac::new(candles).pivots();
    info!("{}", iformat!("Taced {start.elapsed():?}"));

    let start = Instant::now();
    info!("Plotting...");
    plot_candles(&candles_selection.symbol_minutes, &candles, &pivots, &macd_tac, "out/stock.png").unwrap();
    info!("{}", iformat!("Plotted {start.elapsed():?}"));
}

fn read_stream(mut app: Application) {
    app.run_stream();
}

fn triangle(repo: &Repository, exchange: &Exchange, candles_selection: &CandlesSelection) {
    let start = Instant::now();
    info!("Loading...");
    let mut candles_provider = CandlesProvider::new(repo, exchange);
    let selection = Application::selection_factory(candles_selection.clone());
    let candles = candles_provider.candles_selection(&selection).unwrap();
    let candles = candles.iter().collect::<Vec<_>>();
    let candles = candles.as_slice();
    info!("{}", iformat!("Loaded {start.elapsed():?}"));

    let pivots = PivotTac::new(candles).pivots();
    pivots_triangle(&pivots);
}
