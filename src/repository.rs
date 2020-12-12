use crate::{config::symbol_minutes::SymbolMinutes, model::candle::Candle, utils::str_to_datetime};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use ifmt::iprintln;
use rust_decimal::Decimal;
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
        let future = sqlx::query_as("SELECT MAX(close_time) FROM candle WHERE symbol = $1 AND minutes = $2")
            .bind(&symbol_minutes.symbol)
            .bind(&symbol_minutes.minutes)
            .fetch_one(&self.pool);
        let result: (Option<String>,) = async_std::task::block_on(future).unwrap();
        result.0.map(|dt| str_to_datetime(&dt))
    }

    pub fn candle_by_id(&self, id: Decimal) -> Option<Candle> {
        let future = sqlx::query_as!(Candle, "SELECT * FROM candle WHERE id = $1", id).fetch_one(&self.pool);
        async_std::task::block_on(future).ok()
    }

    pub fn candles_default(&self, symbol_minutes: &SymbolMinutes) -> Vec<Candle> {
        let start = Instant::now();
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(14);
        let result = self
            .candles_by_time(symbol_minutes, &start_time, &end_time)
            .unwrap_or_default();
        iprintln!("Read repository: {start.elapsed():?}");
        result
    }

    pub fn candles_by_time(
        &self,
        symbol_minutes: &SymbolMinutes,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> Option<Vec<Candle>> {
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

    pub fn add_candle(&self, candle: &Candle) -> anyhow::Result<Decimal> {
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
        let rec = async_std::task::block_on(future).unwrap();

        Ok(rec.id)
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
        iprintln!("Listing candles limit {limit}:");
        for candle in candles.iter() {
            iprintln!("{candle}");
        }
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::Duration;
    use ifmt::iprintln;

    use crate::utils::inconsistent_candles;

    use super::*;

    #[test]
    fn candles_test() {
        dotenv::dotenv().unwrap();
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(30);
        let repo = Repository::new().unwrap();
        let symbol_minutes = SymbolMinutes::new("BTCUSDT", &15);
        let candles = repo
            .candles_by_time(&symbol_minutes, &start_time, &end_time)
            .unwrap_or_default();

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
}
