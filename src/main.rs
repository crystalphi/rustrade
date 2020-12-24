//#![feature(fold_first)]
#![feature(iterator_fold_self)]

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
use application::{app::Application, streamer::Streamer};
use checker::Checker;
use config::{candles_selection::CandlesSelection, selection::Selection, symbol_minutes::SymbolMinutes};
use exchange::Exchange;
use log::{info, LevelFilter};
use repository::Repository;
use std::collections::HashMap;
use structopt::StructOpt;
use technicals::{ema_tac::EmaTac, macd::macd_tac::MacdTac, technical::Technical};
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
    for tac in vec![MacdTac::definition(), EmaTac::definition()] {
        tacs.insert(tac.name.clone(), tac);
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

    let mut app = Application::new(&repo, &exchange, &checker, selection);

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
        Command::Plot {} => app.plot_selection().unwrap(),
        Command::Stream {} => {
            let mut streamer = Streamer::new(&mut app);
            streamer.run();
        }
        Command::Import {} => {}
        Command::Triangle {} => {
            app.plot_triangles().unwrap();
        }
    };
    info!("Exiting program");
    //assert_e!(row.0, 150);
    // https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/src/main.rs

    Ok(())
}
