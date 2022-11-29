pub struct Binance {
    api_key: String
}

impl Binance {

    /// Creates a new instance of Binance.
    ///
    /// ```
    /// let x: Binance = Binance::new("bruh".to_string());
    /// ```
    pub fn new(api_key: &str) -> Binance {
        Binance {
            api_key: api_key.to_string()
        }
    }
}