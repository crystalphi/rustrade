use super::symbol_minutes::SymbolMinutes;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub struct CandlesSelection {
    pub symbol_minutes: SymbolMinutes,
    #[serde(with = "my_date_format")]
    pub start_time: DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub end_time: DateTime<Utc>,
    pub heikin_ashi: bool,
}

impl CandlesSelection {
    pub fn last_n(symbol: &str, minutes: &u32, last: u32, now: DateTime<Utc>) -> Self {
        let end_time = now;
        let start_time = end_time - (Duration::minutes((minutes * last) as i64));

        Self {
            symbol_minutes: SymbolMinutes::new(symbol, minutes),
            start_time,
            end_time,
            heikin_ashi: true,
        }
    }

    pub fn new(symbol: &str, minutes: &u32, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
        Self {
            symbol_minutes: SymbolMinutes::new(symbol, minutes),
            start_time,
            end_time,
            heikin_ashi: true,
        }
    }
}

mod my_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let result = Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(result)
    }
}

mod my_date_format_opt {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let s = format!("{}", date.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        let result = match s {
            Some(s) => Some(Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?),
            None => None,
        };
        Ok(result)
    }
}
