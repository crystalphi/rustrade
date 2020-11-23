use rust_decimal::Decimal;

pub struct Serie<'a> {
    pub date_time: &'a str,
    pub value: &'a Decimal,
}
