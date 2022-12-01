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
use reqwest;
use std::collections::HashMap;
use symbol::Side;
use reqwest::blocking::Response;
use std::error::Error;

const BINANCE_WS_URL: &str = "wss://stream.binance.com:443/ws";

pub struct Binance {
    api_key: String,
    api_secret: String,
    pub connection: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    pub connection_began_at: time::Instant,
    pub symbols: Option<Vec<Symbol>>,
    pub server_time_stamp: u64,
}

impl Binance {

    /// Creates a new instance of Binance.
    ///
    /// ```
    /// let x: Binance = Binance::new("<api_key>");
    /// ```
    pub fn new(api_key: &str, api_secret: &str) -> Binance {
        Binance {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            connection: None,
            connection_began_at: time::Instant::now(),
            symbols: None,
            server_time_stamp: Binance::get_server_time_stamp().unwrap()
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

    pub fn set_server_time_stamp(&mut self) {
        self.server_time_stamp = Binance::get_server_time_stamp().unwrap();
    }

    pub fn get_server_time_stamp() -> Result<u64, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get("https://api.binance.com/api/v3/time")?
        .json::<HashMap<String, u64>>()?;
        Ok(resp["serverTime"])
    }

    pub fn execute(&mut self, symbol: Symbol, owned_asset: Side, owned_amount: &str, server_time_stamp: u64) -> Result<Response, impl Error> {
        
        let body: String = symbol.build_trade_json(owned_asset, &owned_amount, server_time_stamp, &self.api_secret);
        let client = reqwest::blocking::Client::new();
        let res = client.post("https://api.binance.com/api/v3/order/test")
        .body(body)
        .header("X-MBX-APIKEY", self.api_key.clone())
        .send();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dotenv::dotenv;
    use std::env;

    #[test]
    pub fn test_incoming_message() {
        let mut b1 = Binance::new("Bruh Moment", "Yete");

        b1.new_connection();
        fn print_msg(s: &str) {
            println!("{}", &s);
        }
        b1.handle_incoming(print_msg);
    }

    #[test]
    fn test_get_server_time_stamp() {
        let mut b1 = Binance::new("bruh", "moment");
        println!("{}", b1.server_time_stamp);
        b1.set_server_time_stamp();
        println!("{}", b1.server_time_stamp);
        b1.set_server_time_stamp();
        println!("{}", b1.server_time_stamp);
        b1.set_server_time_stamp();
        println!("{}", b1.server_time_stamp);
    
    }

    #[test]
    fn test_execute() {
        dotenv().ok();
        let api_key = env::var("BINANCE_API_KEY").unwrap();
        let api_secret = env::var("BINANCE_API_SECRET").unwrap();

        let mut b1 = Binance::new(&api_key, &api_secret);

        let json_string = r#"{"u":22277893334,"s":"ETHUSDT","b":"1268.53000000","B":"107.76630000","a":"1268.54000000","A":"3.89930000"}"#;
        let mut s1 = Symbol::new("ETH", "USDT");
        s1.update(json_string);

        let res = b1.execute(
            s1,
            Side::Base, 
            "100", 
            Binance::get_server_time_stamp().unwrap()
        );
        
        if res.is_err() {
            println!("BRUH MOMENT {}", res.unwrap_err());
        } else {
            println!("BRUH MOMENT {:#?}", res.unwrap());
        }

    }
}
