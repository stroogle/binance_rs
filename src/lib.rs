//! A Rust library build to interact with the Binance API
pub struct Binance {
    api_key: String
}

impl Binance {

    /// Creates a new instance of Binance.
    ///
    /// ```
    /// let x: Binance = Binance::new("<api_key>");
    /// ```
    pub fn new(api_key: &str) -> Binance {
        Binance {
            api_key: api_key.to_string()
        }
    }

    /// Creates a new connection and attaches it to instance
    pub fn new_connection() {}

    /// Listens for connection data stream
    /// 
    /// ```
    /// let x: Binance = Binance::new("<api_key>");
    /// let add = |a, b| => a + b;
    /// x.connect(add);
    /// ```
    pub fn connect(&self, callback: fn()) {
        callback();
    }

    /// Responds to ping message
    pub fn send_pong() {

    }

    /// Resets connection
    pub fn reset_connection() {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
