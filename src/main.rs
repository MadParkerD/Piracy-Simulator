#[macro_use]                                                                 
extern crate serde_derive;                                                   

extern crate serde;                                                          
extern crate serde_json; 
mod econosim;
use econosim::start_econ::*;

fn main() {
    for i in bootstrap("test.txt".to_string()){
        println!("{:?}", i);
    }
}
