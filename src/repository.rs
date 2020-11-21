use crate::{
    model::candle::Candle,
    utils::{datetime_to_str, str_to_datetime},
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
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

    pub fn last_close_time(&self, symbol: &str) -> Option<DateTime<Utc>> {
        let future = sqlx::query_as("SELECT MAX(close_time) FROM candle WHERE symbol = $1")
            .bind(symbol)
            .fetch_one(&self.pool);
        let result: (Option<String>,) = async_std::task::block_on(future).unwrap();
        result.0.map(|dt| str_to_datetime(&dt))
    }

    pub fn candle_by_id(&self, id: Decimal) -> Option<Candle> {
        let future =
            sqlx::query_as!(Candle, "SELECT * FROM candle WHERE id = $1", id).fetch_one(&self.pool);
        async_std::task::block_on(future).ok()
    }

    pub fn candles_by_time(
        &self,
        symbol: &str,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> Option<Vec<Candle>> {
        let start_time = datetime_to_str(start_time);
        let end_time = datetime_to_str(end_time);

        #[allow(clippy::suspicious_else_formatting)] 
        let future = sqlx::query_as!(
            Candle,
            "SELECT * FROM candle WHERE symbol = $1 AND (open_time BETWEEN $2 AND $3 OR close_time BETWEEN $2 AND $3)",
            symbol, start_time, end_time 
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
}

#[cfg(test)] 
pub mod tests {
    use chrono::Duration;
    use ifmt::iprintln;

    use super::*;

    #[test]
    fn candles_test() {
        dotenv::dotenv().unwrap();
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(1);
        let repo = Repository::new().unwrap();
        let candles = repo.candles_by_time("BTCUSDT", &start_time, &end_time).unwrap_or_default();
        println!("Found candles:");
        for candle in candles {
            iprintln!("{candle}");
        }

    }

}
