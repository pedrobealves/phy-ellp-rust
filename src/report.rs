use std::collections::LinkedList;
use std::error::Error;
use rust_xlsxwriter::*;
use crate::car::Cart;
use crate::state::State;


pub struct Report {
}

impl Default for Report {
    fn default() -> Self {
            Report {
            }
    }
}

impl Report {
    pub fn create(&mut self, states: &LinkedList<State>, car: &Cart) -> Result<(),Box<dyn Error>> {


        Ok(())
    }
}
