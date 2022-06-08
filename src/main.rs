// (compile-time static) Collection of three calibration curves.
mod intcal20;
mod calibration;

use calibration::{UncalibratedRadioCarbonDate, carbon_date_density_bchron};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    year: u32,

    /// Standard deviation
    #[clap(short, long="sd")]
    sd: i32,
}

fn main() {
    let args = Args::parse();

    println!("Running the calibration using curve IntCal20...");
    println!("Sample is BP {}±{}", args.year, args.sd);

    let calibrated_result = calibration::carbon_date_density_bchron(UncalibratedRadioCarbonDate {
        c14_age: args.year,
        c14_range: args.sd
    });

    println!("{:?}", calibrated_result);
}
