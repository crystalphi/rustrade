use crate::{config::symbol_minutes::SymbolMinutes, model::candle::Candle};
use anyhow::{bail, Result};
use chrono::{DateTime, Duration, Utc};
use ifmt::iformat;
use log::{error, info};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use rust_decimal_macros::dec;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, time::Instant};
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new() -> Result<Repository> {
        let e = env::var("DATABASE_URL")?;
        let future = PgPoolOptions::new().max_connections(5).connect(&e);
        let pool = async_std::task::block_on(future)?;
        Ok(Repository { pool })
    }

    pub fn last_id(&self) -> Decimal {
        let future = sqlx::query_as("SELECT MAX(id) FROM candle").fetch_one(&self.pool);
        let result: (Option<Decimal>,) = async_std::task::block_on(future).unwrap();
        result.0.unwrap_or_default()
    }

    pub fn last_close_time(&self, symbol_minutes: &SymbolMinutes) -> Option<DateTime<Utc>> {
        let future = sqlx::query!(
            "SELECT MAX(close_time) as close_time FROM candle WHERE symbol = $1 AND minutes = $2",
            &symbol_minutes.symbol,
            Decimal::from_u32(symbol_minutes.minutes)
        )
        .fetch_one(&self.pool);
        let result = async_std::task::block_on(future).unwrap();
        result.close_time
    }

    pub fn ranges_symbol_minutes(&self, symbol_minutes: &SymbolMinutes) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
        let future = sqlx::query!(
            "SELECT MIN(close_time) as min_close_time, MAX(close_time) as max_close_time FROM candle WHERE symbol = $1 AND minutes = $2",
            &symbol_minutes.symbol,
            Decimal::from_u32(symbol_minutes.minutes)
        )
        .fetch_one(&self.pool);
        let result = async_std::task::block_on(future).unwrap();
        (result.min_close_time, result.max_close_time)
    }

    pub fn candle_by_id(&self, id: Decimal) -> Option<Candle> {
        let future = sqlx::query_as!(Candle, "SELECT * FROM candle WHERE id = $1", id).fetch_one(&self.pool);
        async_std::task::block_on(future).ok()
    }

    pub fn candles_default(&self, symbol_minutes: &SymbolMinutes) -> Vec<Candle> {
        let start = Instant::now();
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(14);
        let result = self.candles_by_time(symbol_minutes, &start_time, &end_time).unwrap_or_default();
        info!("{}", iformat!("Read repository: {start.elapsed():?}"));
        result
    }

    pub fn symbols_minutes(&self) -> Vec<(SymbolMinutes, i64)> {
        let mut result = Vec::new();

        let future = sqlx::query_as(
            r#"
                SELECT symbol, minutes, count(*) as qtd FROM candle                 
                GROUP BY symbol, minutes
                "#,
        )
        .fetch_all(&self.pool);

        let rows: Vec<(String, Decimal, i64)> = async_std::task::block_on(future).unwrap();
        for row in rows {
            let symbol_minutes = SymbolMinutes::new(&row.0, &row.1.to_u32().unwrap());
            result.push((symbol_minutes, row.2));
        }
        result
    }

    pub fn candles_by_time(&self, symbol_minutes: &SymbolMinutes, start_time: &DateTime<Utc>, end_time: &DateTime<Utc>) -> Option<Vec<Candle>> {
        let minutes = Decimal::from(symbol_minutes.minutes);

        #[allow(clippy::suspicious_else_formatting)]
        let future = sqlx::query_as!(
            Candle,
            r#"
                SELECT * FROM candle 
                WHERE symbol = $1 AND minutes = $2 AND (open_time BETWEEN $3 AND $4 OR close_time BETWEEN $3 AND $4)
                ORDER BY open_time
            "#,
            symbol_minutes.symbol,
            minutes,
            start_time,
            end_time
        )
        .fetch_all(&self.pool);
        async_std::task::block_on(future).ok()
    }

    pub fn last_candles(&self, symbol: &str, minutes: &u32, limit: &i64) -> Option<Vec<Candle>> {
        let minutes = Decimal::from(*minutes);

        #[allow(clippy::suspicious_else_formatting)]
        let future = sqlx::query_as!(
            Candle,
            r#"
                SELECT * FROM candle 
                WHERE symbol = $1 AND minutes = $2
                ORDER BY open_time DESC
                FETCH FIRST $3 ROWS ONLY
            "#,
            symbol,
            minutes,
            limit
        )
        .fetch_all(&self.pool);
        async_std::task::block_on(future).ok()
    }

    pub fn insert_candles(&self, candles: &mut [Candle]) -> anyhow::Result<()> {
        let mut candle_id = self.last_id();
        let one = dec!(1);
        candles.iter_mut().for_each(|c| {
            c.id = {
                candle_id += one;
                candle_id
            }
        });

        let candles_errors = candles
            .iter()
            .map(|c| (c, self.insert_candle(c)))
            .filter(|cr| cr.1.is_err())
            .collect::<Vec<_>>();
        if !candles_errors.is_empty() {
            let c = candles_errors.get(0).unwrap().0;
            error!("{}", iformat!("Candles add error: {candles_errors.len()}"));
            error!("{}", iformat!("First candle: {c}"));
            bail!("Candles add error");
        }

        Ok(())
    }

    pub fn insert_candle(&self, candle: &Candle) -> anyhow::Result<Decimal> {
        let future = sqlx::query!(
            r#"
                INSERT INTO candle ( 
                    id,
                    symbol,
                    minutes,
                    open_time,
                    close_time,
                    open,
                    high,
                    low,
                    close,
                    volume )
                VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10 )
                RETURNING id
            "#,
            candle.id,
            candle.symbol,
            candle.minutes,
            candle.open_time,
            candle.close_time,
            candle.open,
            candle.high,
            candle.low,
            candle.close,
            candle.volume
        )
        .fetch_one(&self.pool);
        let rec = async_std::task::block_on(future)?;

        Ok(rec.id)
    }

    pub fn delete_all_candles(&self) -> anyhow::Result<()> {
        let future = sqlx::query!("DELETE FROM candle").execute(&self.pool);
        async_std::task::block_on(future)?;
        Ok(())
    }

    pub fn delete_candle(&self, id: &Decimal) {
        let future = sqlx::query!("DELETE FROM candle WHERE id = $1", id).execute(&self.pool);
        async_std::task::block_on(future).unwrap();
    }

    pub fn delete_last_candle(&self, symbol_minutes: &SymbolMinutes) {
        let future = sqlx::query!(
            r#"DELETE FROM candle WHERE id = 
            (SELECT id FROM candle WHERE symbol = $1 AND minutes = $2 
                ORDER BY close_time DESC FETCH FIRST 1 ROWS ONLY
            )"#,
            symbol_minutes.symbol,
            symbol_minutes.minutes as i64,
        )
        .execute(&self.pool);
        async_std::task::block_on(future).unwrap();
    }

    pub fn list_candles(&self, symbol: &str, minutes: &u32, limit: &i64) {
        let candles = self.last_candles(symbol, minutes, limit).unwrap_or_default();
        info!("{}", iformat!("Listing candles limit {limit}:"));
        for candle in candles.iter() {
            info!("{}", iformat!("{candle}"));
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::candles_utils::inconsistent_candles;
    use chrono::Duration;
    use ifmt::iprintln;

    #[test]
    fn candles_test() {
        dotenv::dotenv().unwrap();
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(30);
        let repo = Repository::new().unwrap();
        let symbol_minutes = SymbolMinutes::new("BTCUSDT", &15);
        let candles = repo.candles_by_time(&symbol_minutes, &start_time, &end_time).unwrap_or_default();

        println!("Found candles:");
        for candle in candles.iter() {
            iprintln!("{candle}");
        }

        let candles_ref: Vec<_> = candles.iter().collect();

        println!("Inconsist candles:");
        let inconsist = inconsistent_candles(candles_ref.as_slice(), &Duration::minutes(15));
        for candle in inconsist.iter() {
            iprintln!("{candle}");
        }
    }

    #[test]
    fn symbols_minutes_test() {
        dotenv::dotenv().unwrap();
        let repo = Repository::new().unwrap();
        let symbols_minutes = repo.symbols_minutes();

        iprintln!("symbols_minutes.len: {symbols_minutes.len()}");
        for (symbol_minutes, count) in symbols_minutes {
            let last_close_time = repo.last_close_time(&symbol_minutes);
            iprintln!("{symbol_minutes:?} {count}  {last_close_time:?}");
            let range = repo.ranges_symbol_minutes(&symbol_minutes);
            iprintln!("{symbol_minutes:?} {count}  {range.0:?} - {range.1:?}");
        }
    }

    #[test]
    fn add_candles_test() {}
}
