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
use application::{app::Application, candles_provider::CandlesProvider, streamer::Streamer};
use checker::Checker;
use config::{candles_selection::CandlesSelection, selection::Selection, symbol_minutes::SymbolMinutes};
use exchange::Exchange;
use ifmt::iformat;
use log::{info, LevelFilter};
use repository::Repository;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
use structopt::StructOpt;
use tac_plotters::plotter::plot_candles;
use technicals::{macd::macd_tac::MacdTac, pivots::PivotTac, technical::Technical};
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

pub fn selection_factory(candles_selection: CandlesSelection) -> Selection {
    let mut tacs = HashMap::new();
    for tac in vec![MacdTac::definition()] {
        tacs.insert(tac.name, tac);
    }
    Selection {
        tacs,
        candles_selection,
        image_name: "out/stock.png".to_string(),
    }
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

    let selection = selection_factory(candles_selection.clone());

    let symbol_minutes = SymbolMinutes::new(&opt.symbol, &opt.minutes);
    let checker = Checker::new(&symbol_minutes, &repo, &exchange);

    let mut app = Application::new(&repo, &exchange, &checker, selection.clone());

    match opt.command {
        Command::Check {} => {
            checker.check_inconsist(&repo, &candles_selection);
        }
        Command::Sync {} => {
            checker.synchronize().unwrap();
        }
        Command::Fix {} => {
            checker.delete_inconsist();
        }
        Command::List {} => {
            repo.list_candles(&opt.symbol, &opt.minutes, &10);
        }
        Command::Plot {} => plot(&repo, &exchange, &selection),
        Command::Stream {} => {
            let mut streamer = Streamer::new(&mut app);
            streamer.run();
        }
        Command::Import {} => {}
        Command::Triangle {} => {
            app.plot_triangles();
        }
    };
    info!("Exiting program");
    //assert_e!(row.0, 150);
    // https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/src/main.rs

    Ok(())
}

fn plot(repo: &Repository, exchange: &Exchange, selection: &Selection) {
    let start = Instant::now();
    info!("Loading...");
    let mut candles_provider = CandlesProvider::new(repo, exchange);
    let candles = candles_provider.candles_selection(selection).unwrap();
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
    plot_candles(&selection, &candles, &pivots, &macd_tac, "out/stock.png").unwrap();
    info!("{}", iformat!("Plotted {start.elapsed():?}"));
}
