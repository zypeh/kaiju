// (compile-time static) Collection of three calibration curves.
mod intcal20;

mod calibration;

use clap::Parser;

// 1950AD: 3000 = 3000BP = 1050BC

fn main() {
    println!("{:?}", intcal20::AGE_BP);
}
