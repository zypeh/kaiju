use core::fmt;
use std::f64::consts::PI;
use zipWith::IntoZipWith;

use crate::intcal20::{AGE_BP, CALIBRATION_BP, SIGMA_OF_THE_YEAR};

type YearBP = u32;
type YearRange = i32;

#[derive(Debug)]
struct YearInBCOrAD(i32);

impl fmt::Display for YearInBCOrAD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_negative() {
            write!(f, "BC {}", self.0.abs())
        } else {
            write!(f, "AD {}", self.0)
        }
    }
}

fn bp_to_bcad (bp: YearBP) -> YearInBCOrAD { YearInBCOrAD(1950 - (bp as i32)) }

/// Struct for uncalibrated radiocarbon date, input for `run_calibration`
#[derive(Debug, PartialEq, Eq)]
pub struct UncalibratedRadioCarbonDate {
    /// C_14 age in years BP
    pub c14_age: YearBP,
    /// sigma in years, also served as standard deviation
    pub c14_range: YearRange,
}

/// Just a slice to const static Int20 curve
#[derive(Debug)]
pub struct CalibratedCurveInBP<'a> {
    bp: &'a[YearBP],
    uncalibrated_bp: &'a[YearBP],
    sigma_bp: &'a[YearRange],
}

/// `pdf` stands for probability density function
#[derive(Debug)]
pub struct CalibratedPDF {
    pdf: Vec<i32>,
    pdf_density: Vec<f64>,
}

// Calibrate using standard deviation method
// Calibeate using the IntCal20 curve
pub fn carbon_date_density_bchron(uncal: UncalibratedRadioCarbonDate) -> CalibratedPDF {
    let age = bp_to_bcad(uncal.c14_age);
    let age_sd2 = uncal.c14_range.pow(2);

    let cal_curve = calibrated_curve_segment(uncal);

    // I just wondered why is the number so huge. I need to convert to year BC or AD
    let bp_in_bcad: Vec<i32> = cal_curve.bp.iter().map(|&x| bp_to_bcad(x).0).collect();
    let ulcal_bp_in_bcad: Vec<i32> = cal_curve.bp.iter().map(|&x| bp_to_bcad(x).0).collect();

    let densities: Vec<f64> = ulcal_bp_in_bcad.zip_with(&bp_in_bcad, |mu, &cal| {
        let det = ((age.0 - mu) as f64) / ((age_sd2 as f64) + (cal as f64).powi(2)).sqrt();
        dnorm(0.0, 1.0, det as f64)
    }).collect();

    normalise_calibrated_pdf(CalibratedPDF { pdf: bp_in_bcad, pdf_density: densities })
}

fn normalise_calibrated_pdf(pdf: CalibratedPDF) -> CalibratedPDF {
    let sum_of_densities: f64 = pdf.pdf_density.iter().sum();
    let normalised_densities: Vec<f64> = pdf.pdf_density.iter().map(|&d| d / sum_of_densities).collect();
    CalibratedPDF { pdf: pdf.pdf, pdf_density: normalised_densities }
}

/// This will grab the segment from the Int20 data
fn calibrated_curve_segment (uncal: UncalibratedRadioCarbonDate) -> CalibratedCurveInBP<'static> {
    let mean = uncal.c14_age;
    let std  = uncal.c14_range;

    let start = 6 * (std as u32) + mean;
    let stop  = mean - 6 * (std as u32);

    let start_idx = AGE_BP.iter().position(|&x| x <= start).unwrap_or(0);
    let stop_idx  = (AGE_BP.len() - 1) - AGE_BP.iter().rev().position(|&x| x >= stop).unwrap_or(0);

    CalibratedCurveInBP {
        bp: &CALIBRATION_BP[start_idx .. stop_idx],
        uncalibrated_bp: &AGE_BP[start_idx .. stop_idx],
        sigma_bp: &SIGMA_OF_THE_YEAR[start_idx .. stop_idx]
    }
}

fn dnorm(mu: f64, sigma: f64, x: f64) -> f64 {
    let a = (2.0 * PI * sigma.powi(2)).sqrt().recip();
    let b = (-(x - mu).powi(2) / (2.0 * sigma.powi(2))).exp();
    a * b
}

#[cfg(test)]
mod calibration {
    use super::*;

    #[test]
    fn density_of_normal_dist() -> () {
        assert_eq!(dnorm(1.0, 1.0, 1.0) as f32, 0.3989423);
    }

    #[test]
    fn bp_conversion() -> () {
        assert_eq!(format!("{}", bp_to_bcad(3000)), "BC 1050");
    }

    #[test]
    fn get_curve() -> () {
        println!("{:?}", carbon_date_density_bchron(UncalibratedRadioCarbonDate {
            c14_age: 3000,
            c14_range: 30
        }))
    }
}