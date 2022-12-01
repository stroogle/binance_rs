//! A Rust library build to interact with the Binance API

// Modules
mod symbol;

// Uses
use tungstenite::{WebSocket, Message};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use url::Url;
use tungstenite::{connect};
use std::time;
use symbol::Symbol;

const BINANCE_WS_URL: &str = "wss://stream.binance.com:443/ws";

pub struct Binance {
    _api_key: String,
    pub connection: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    pub connection_began_at: time::Instant,
    pub symbols: Option<Vec<Symbol>>
}

impl Binance {

    /// Creates a new instance of Binance.
    ///
    /// ```
    /// let x: Binance = Binance::new("<api_key>");
    /// ```
    pub fn new(api_key: &str) -> Binance {
        Binance {
            _api_key: api_key.to_string(),
            connection: None,
            connection_began_at: time::Instant::now(),
            symbols: None
        }
    }

    /// Creates a new connection and attaches it to instance
    pub fn new_connection(&mut self) {
        let (socket, _response) = connect(
            Url::parse(BINANCE_WS_URL).unwrap()
        ).expect("Couldn't connect to socket");
        self.connection_began_at = time::Instant::now();
        self.connection = Some(socket);
        self.follow_ticker("ETHBUSD");
    }

    /// Not a completed function yet.
    /// Used to subscribe to tickers on the socket connection.
    pub fn follow_ticker(&mut self, _ticker: &str) {
        if self.connection.is_none() {
            return;
        }
        self.connection.as_mut().unwrap().write_message(Message::Text(r#"{
            "method": "SUBSCRIBE",
            "params":
            [
                "ethusdt@bookTicker",
                "bnbeth@bookTicker",
                "etceth@bookTicker",
                "zeceth@bookTicker"
            ],
            "id": 1
        }"#.into())).expect("Uh oh!");
    }

    /// Handles incoming data stream.
    /// Callback function only called on incoming ticker data.
    /// connection resets handled automatically in here.
    /// 
    /// ```
    /// let x: Binance = Binance::new("<api_key>");
    /// let add = |a, b| => a + b;
    /// x.connect(add);
    /// ```
    pub fn handle_incoming(&mut self, callback: fn(s: &str)) {
        loop {
            let msg: Message = self.connection.as_mut().unwrap().read_message().expect("Error reading message");
            let msg_as_string = msg.to_string();

            // Handle ping
            // Handle incoming ticker data
            callback(&msg_as_string);

            // Reset the connection if it has been running for 12 hours
            if self.connection_began_at.elapsed().as_secs() > (60 * 60 * 12) {
                self.reset_connection();
            }
        }
    }

    /// Resets connection
    pub fn reset_connection(&mut self) {
        println!("Connection being reset.");
        self.new_connection();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_incoming_message() {
        let mut b1 = Binance::new("Bruh Moment");

        b1.new_connection();
        fn print_msg(s: &str) {
            println!("{}", &s);
        }
        b1.handle_incoming(print_msg);
    }
}
