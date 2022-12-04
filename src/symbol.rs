//! Symbols is used to interact with specific trading symbols on the Binance spot market

use serde_json::{Value};
use rust_decimal::prelude::*;
use std::{cmp::Ordering};

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub enum Side {
    Base,
    Quote
}

/// Represets a trading symbol on the binance spot trading market.
pub struct Symbol {
    /// The base asset of the symbol
    pub base: String,
    /// The quote asset of the symbol
    pub quote: String,
    /// The current asking price of the symbol
    pub asking_price: String,
    /// The current amount of the base asset available for sale
    pub asking_qty: String,
    /// The highest bid price available for the base asset in the quote asset
    pub bid_price: String,
    /// The most of the base asset purchasable buy the bidder
    pub bid_qty: String
}

impl Symbol {
    pub fn new(base: &str, quote: &str) -> Self {

        Self {
            base: base.to_string(),
            quote: quote.to_string(),
            asking_price: "1.000000".to_string(),
            asking_qty: "-1.000000".to_string(),
            bid_price: "1.000000".to_string(),
            bid_qty: "-1.000000".to_string()
        }

    }

    pub fn calculate_trade(&self, owned_asset: Side, amount: &str) -> Result<String, &str> {
        let owned_amount: Decimal = Decimal::from_str(amount).unwrap();
        let res: Decimal;
        match owned_asset {
            Side::Base => {
                let bid_price: Decimal = Decimal::from_str(&self.bid_price).unwrap();
                res = owned_amount * bid_price;
                let bid_qty: Decimal = Decimal::from_str(&self.bid_qty).unwrap();
                let total_value: Decimal = bid_price * bid_qty;
                if res.cmp(&total_value) == Ordering::Greater {
                    return Err("Base_side Not enough qty to trade");
                }
            },
            Side::Quote => {
                res = owned_amount / Decimal::from_str(&self.asking_price).unwrap();
                let ask_qty: Decimal = Decimal::from_str(&self.asking_qty).unwrap();
                if res.cmp(&ask_qty) == Ordering::Greater {
                    return Err("Not enough qty to trade");
                }
            }
        }
        Ok(res.round_dp_with_strategy(8, RoundingStrategy::ToZero).to_string())
    }

    pub fn update(&mut self, json_str: &str) {
        let res: Value = serde_json::from_str(json_str).unwrap();

        self.asking_price = res["a"].to_string().replace('\"', "");
        self.asking_qty= res["A"].to_string().replace('\"', "");
        self.bid_price= res["b"].to_string().replace('\"', "");
        self.bid_qty= res["B"].to_string().replace('\"', "");
    }

    pub fn build_trade_json(&self, owned_asset: Side, owned_amount: &str, server_time_stamp: u64, api_secret: &str) -> String {
        let qty: String;
        let side: &str;
        let base = self.base.clone();
        let quote = self.quote.clone();

        match owned_asset {
            Side::Base => {
                side = "SELL";
                qty = format!("quantity={owned_amount}");
            },
            Side::Quote => {
                side = "BUY";
                qty = format!("quoteOrderQty={owned_amount}");
            }
        }

        let result = format!("symbol={base}{quote}&side={side}&type=MARKET&{qty}&recvWindow=5000&timestamp={server_time_stamp}");

        let mut signed_key = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).unwrap();
        signed_key.update(result.as_bytes());
        let signature = hex::encode(signed_key.finalize().into_bytes());

        let result = format!("symbol={base}{quote}&side={side}&type=MARKET&{qty}&recvWindow=5000&timestamp={server_time_stamp}&signature={signature}");

        result
    }

}

#[cfg(test)]
mod test {

    use crate::Binance;
    use std::env;
    use dotenv::dotenv;

    use super::*;

    #[test]
    fn test_new() {
        let s1 = Symbol::new("BTC", "USDT");

        assert_eq!(s1.base, "BTC");
        assert_eq!(s1.quote, "USDT");

        assert_eq!(s1.asking_price, "1.000000");
        assert_eq!(s1.asking_qty, "1.000000");
        assert_eq!(s1.bid_price, "1.000000");
        assert_eq!(s1.bid_qty, "1.000000");
    }

    #[test]
    fn test_calculate_trade_base() {
        let json_string = r#"{"u":22277893334,"s":"ETHUSDT","b":"1268.53000000","B":"107.76630000","a":"1268.54000000","A":"3.89930000"}"#;
        let mut s1 = Symbol::new("ETH", "USDT");
        s1.update(json_string);

        assert_eq!(s1.calculate_trade(Side::Base, "100").unwrap(), "126853.00000000");
        assert_eq!(s1.calculate_trade(Side::Base, "10").unwrap(), "12685.30000000");
        assert_eq!(s1.calculate_trade(Side::Base, "1").unwrap(), "1268.53000000");
        assert_eq!(s1.calculate_trade(Side::Base, "107.76630000").unwrap(), "136704.78453900");
    
        assert_eq!(s1.calculate_trade(Side::Base, "107.76640000"), Err("Base_side Not enough qty to trade"));

        let s2 = Symbol::new("ETH", "USDT");

        assert_eq!(s2.calculate_trade(Side::Base, "1"), Err("Base_side Not enough qty to trade"));        
        assert_eq!(s2.calculate_trade(Side::Quote, "100"), Err("Not enough qty to trade"));        
    }
     #[test]
    fn test_calculate_trade_quote() {
        let json_string = r#"{"u":22277893334,"s":"ETHUSDT","b":"1268.53000000","B":"107.76630000","a":"1268.54000000","A":"3.89930000"}"#;
        let mut s1 = Symbol::new("ETH", "USDT");
        s1.update(json_string);

        assert_eq!(s1.calculate_trade(Side::Quote, "100").unwrap(), "0.07883078");
        assert_eq!(s1.calculate_trade(Side::Quote, "10").unwrap(), "0.00788307");
        assert_eq!(s1.calculate_trade(Side::Quote, "1").unwrap(), "0.00078830");
        assert_eq!(s1.calculate_trade(Side::Quote, "4946.418022").unwrap(), "3.8993");
    
        assert_eq!(s1.calculate_trade(Side::Quote, "4946.418023"), Err("Not enough qty to trade"));
    }

    #[test]
    fn test_update() {
        let json_string = r#"{"u":22277893334,"s":"ETHUSDT","b":"1268.53000000","B":"107.76630000","a":"1268.54000000","A":"3.89930000"}"#;
        let mut s1 = Symbol::new("ETH", "USDT");
        s1.update(json_string);

        assert_eq!(s1.base, "ETH");
        assert_eq!(s1.quote, "USDT");

        assert_eq!(s1.bid_price, "1268.53000000");
        assert_eq!(s1.bid_qty, "107.76630000");
        assert_eq!(s1.asking_price, "1268.54000000");
        assert_eq!(s1.asking_qty, "3.89930000");
    }

    #[test]
    fn test_build_trade_json() {
        dotenv().ok();
        let json_string = r#"{"u":22277893334,"s":"ETHUSDT","b":"1268.53000000","B":"107.76630000","a":"1268.54000000","A":"3.89930000"}"#;
        let mut s1 = Symbol::new("ETH", "USDT");
        s1.update(json_string);
        
        println!("BRUH MOMENT {}", s1.build_trade_json(Side::Base, "100", Binance::get_server_time_stamp().unwrap(), &env::var("BINANCE_API_SECRET").unwrap()));

        let json_string = r#"{"u":22277893334,"s":"ETHUSDT","b":"1268.53000000","B":"107.76630000","a":"1268.54000000","A":"3.89930000"}"#;
        let mut s1 = Symbol::new("ETH", "USDT");
        s1.update(json_string);
        
        println!("BRUH MOMENT {}", s1.build_trade_json(Side::Quote, "100", Binance::get_server_time_stamp().unwrap(), ""));
    }
}