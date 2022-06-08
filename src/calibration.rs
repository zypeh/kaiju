use crate::intcal20::{AGE_BP, CALIBRATION_BP, SIGMA_OF_THE_YEAR};

// 1950AD: 3000 = 3000BP = 1050BC
type YearBP = u32;
type YearInBCOrAD = i32;
type YearRange = i32;

pub fn bp_to_bcad (bp: YearBP) -> YearInBCOrAD { 1950 - (bp as i32) }

/// Struct for uncalibrated radiocarbon date, input for `run_calibration`
#[derive(Debug, PartialEq, Eq)]
pub struct UncalibratedRadioCarbonDate {
    /// C_14 age in years BP
    c14_age: YearBP,
    /// sigma in years, also served as standard deviation
    c14_range: YearRange,
}

/// Just a slice to const static Int20 curve
pub struct CalibratedCurveInBP<'a> {
    bp: &'a[YearBP],
    uncalibrated_bp: &'a[YearBP],
    sigma_bp: &'a[YearRange],
}

/// `pdf` stands for probability density function
pub struct CalibratedPDF {
    pdf: Vec<YearInBCOrAD>,
    pdf_density: Vec<f64>,
}

// Calibrate using standard deviation method
// Calibeate using the IntCal20 curve
fn calibration_carbon_date_Bchron(a: UncalibratedRadioCarbonDate) -> CalibratedPDF {
    CalibratedPDF { pdf: vec![], pdf_density: vec![] }
}

fn calibrated_curve_segment (uncal: UncalibratedRadioCarbonDate) -> CalibratedCurveInBP<'static> {
    let mean = uncal.c14_age;
    let std  = uncal.c14_range;

    let start = 6 * (std as u32) + mean;
    let stop  = mean - 6 * (std as u32);

    let start_idx = &AGE_BP.iter().position(|&x| x >= start).unwrap_or(0);
    let stop_idx  = &AGE_BP.iter().rev().position(|&x| x <= stop).unwrap_or(0);

    let to_idx = stop_idx - start_idx;

    CalibratedCurveInBP {
        bp: &CALIBRATION_BP[(start as usize) .. to_idx],
        uncalibrated_bp: &AGE_BP[(start as usize) .. to_idx],
        sigma_bp: &SIGMA_OF_THE_YEAR[(start as usize) .. to_idx]
    }
}
