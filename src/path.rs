use crate::symbol::{Symbol, Side};
use rust_decimal::prelude::*;

/// Represents a series of 3 trades that can be made to return from BUSD to BUSD
pub struct Path {
    trade_1: &Symbol,
    trade_2: &Symbol,
    trade_3: &Symbol
}

impl Path {

    fn new(trade_1: &Symbol, trade_2: &Symbol, trade_3: &Symbol,) -> Self {
        Seld {
            trade_1,
            trade_2,
            trade_3
        }
    }

    /// Checks if a given path is profitable    
    fn is_profitable(&self) -> bool {
        false
    }

}