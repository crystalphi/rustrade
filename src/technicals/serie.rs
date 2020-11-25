pub struct Serie<'a> {
    pub date_time: &'a str,
    pub value: f64,
}

impl<'a> Serie<'a> {
    pub fn new(date_time: &'a str, value: f64) -> Self {
        Serie { date_time, value }
    }
}
