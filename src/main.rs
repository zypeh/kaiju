// (compile-time static) Collection of three calibration curves.
mod intcal20;

use clap::Parser;

// 1950AD: 3000 = 3000BP = 1050BC
type YearBP = u32;
type YearInBCorAD = u32;
type YearRange = i32;


fn main() {
    println!("{:?}", intcal20::AGE_BP);
}
