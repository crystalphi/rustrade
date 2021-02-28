use super::{macd_trend::MacdTrend, trade_context_provider::TradeContextProvider, trader::Trader};
use crate::{
    application::{
        app::Application,
        candles_provider::{CandlesProvider, CandlesProviderBuffer},
        plot_selection::plot_selection,
    },
    config::candles_selection::CandlesSelection,
    tac_plotters::{indicator_plotter::PlotterIndicatorContext, trading_plotter::TradingPlotter},
    technicals::ind_provider::IndicatorProvider,
};
use ifmt::iformat;
use lockfree_object_pool::LinearObjectPool;
use log::info;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Clone)]
pub struct TraderFactory {
    candles_selection: CandlesSelection,
    candles_provider: CandlesProviderBuffer,
}

impl TraderFactory {
    pub fn new(candles_selection: CandlesSelection, candles_provider: CandlesProviderBuffer) -> Self {
        Self {
            candles_selection,
            candles_provider,
        }
    }

    pub fn create_trader(&self) -> Trader {
        //info!("*************************** create_trader {:?}...", thread::current().id());
        let mut candles_provider = self.candles_provider.clone();
        candles_provider.set_candles_selection(self.candles_selection.clone());
        let indicator_provider = IndicatorProvider::new();
        let trend_context_provider = TradeContextProvider::new(&self.candles_selection.symbol_minutes.symbol, indicator_provider, candles_provider);
        let mcad_trend = MacdTrend::new();
        Trader::new(trend_context_provider, Box::new(mcad_trend))
    }
}

pub fn run_trader_back_test(app: &mut Application) -> anyhow::Result<()> {
    let start = Instant::now();
    info!("Initializing backtest...");

    let trader_factory = TraderFactory::new(app.selection.candles_selection.clone(), app.candles_provider.clone());

    //let position = Position::new_from_usd(dec!(1000));
    //let trader_register = TraderRegister::new(position);

    app.candles_provider.set_candles_selection(app.selection.candles_selection.clone());
    let candles = app.candles_provider.candles()?;
    let msg = format!("Running back test... candles.len {}", candles.len());
    info!("{}", msg);

    //for i in 1..candles.len() {
    //let candles_ref = &candles[0..=i];
    //let c = candles_ref.last().unwrap();
    //  trader.check(&mut trader_register, /*candles_ref,*/ c.close_time, c.close).unwrap();
    //}

    let pool = LinearObjectPool::<Trader>::new(move || trader_factory.create_trader(), |_| ());
    //let pool = Arc::new(pool);

    //let pool = Pool::<Trader>::new(32, || trader_factory.create_trader());
    let pool_rayon = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap();
    let trades = pool_rayon.install(|| {
        candles
            .par_iter()
            .map(|c| {
                //let candles_ref = &candles[0..=i];
                //let c = candles_ref.last().unwrap();
                let mut trader = pool.pull();

                // let mut trader = pool.try_pull().unwrap();

                //info!("let mut trader {:?}...", thread::current().id());

                trader.check(/*candles_ref,*/ c.close_time, c.close).unwrap();
                trader.trades()
            })
            .flatten()
            .collect::<Vec<_>>()
    });

    // TODO generating position from trades

    //let trades = trader.trades();
    //

    let trading_plotter = TradingPlotter::new(&trades);

    let plotters = vec![Box::new(trading_plotter) as Box<dyn PlotterIndicatorContext>];

    plot_selection(app.selection.clone(), app.candles_provider.clone_provider(), plotters)?;

    info!("{}", iformat!("Finished backtest, elapsed: {start.elapsed():?}"));

    Ok(())
}
