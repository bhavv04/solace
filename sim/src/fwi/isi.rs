//! Initial Spread Index (ISI).
//!
//! Combines FFMC (fine fuel moisture) and wind speed into a rating that
//! correlates with fire spread rate. Feeds directly into the FBP System's
//! ROS equations. Van Wagner (1987).

/// Compute ISI from today's FFMC and the wind speed (km/h) at the time of
/// interest (this need not be the same noon wind used to derive FFMC —
/// ISI can be recomputed hourly against fresh wind observations while
/// FFMC itself only updates once a day).
pub fn calc_isi(ffmc: f64, wind_kmh: f64) -> f64 {
    let m = 147.2 * (101.0 - ffmc) / (59.5 + ffmc);
    let f_wind = (0.05039 * wind_kmh).exp();
    let f_f = 91.9 * (-0.1386 * m).exp() * (1.0 + m.powf(5.31) / 4.93e7);
    0.208 * f_wind * f_f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_wind_increases_isi() {
        let low = calc_isi(90.0, 5.0);
        let high = calc_isi(90.0, 25.0);
        assert!(high > low, "expected higher wind to raise ISI");
    }

    #[test]
    fn higher_ffmc_increases_isi() {
        let low = calc_isi(70.0, 15.0);
        let high = calc_isi(95.0, 15.0);
        assert!(high > low, "expected higher FFMC (drier fuel) to raise ISI");
    }
}