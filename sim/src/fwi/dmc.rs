//! Duff Moisture Code (DMC).
//!
//! Represents average moisture of loosely compacted organic layers
//! (~7 cm deep, ~5 kg/m^2). Drives BUI along with DC. Van Wagner (1987).

use super::{NoonWeather, DMC_DAYLENGTH};

/// Compute today's DMC from yesterday's DMC, today's noon weather, and month.
pub fn calc_dmc(dmc_prev: f64, w: NoonWeather, month: u32) -> f64 {
    let le = DMC_DAYLENGTH[((month.clamp(1, 12)) - 1) as usize];

    // Rain phase.
    let dmc_after_rain = if w.precip_mm > 1.5 {
        let re = 0.92 * w.precip_mm - 1.27;
        let mo = 20.0 + (5.6348 - dmc_prev / 43.43).exp();

        let b = if dmc_prev <= 33.0 {
            100.0 / (0.5 + 0.3 * dmc_prev)
        } else if dmc_prev <= 65.0 {
            14.0 - 1.3 * dmc_prev.ln()
        } else {
            6.2 * dmc_prev.ln() - 17.2
        };

        let mr = mo + 1000.0 * re / (48.77 + b * re);
        let pr = 244.72 - 43.43 * (mr - 20.0).ln();
        pr.max(0.0)
    } else {
        dmc_prev
    };

    // Drying phase (log drying rate), applied every day regardless of rain.
    let k = if w.temp_c > -1.1 {
        1.894 * (w.temp_c + 1.1) * (100.0 - w.rh_pct) * le * 1e-4
    } else {
        0.0
    };

    (dmc_after_rain + k).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_warm_day_increases_dmc() {
        let w = NoonWeather {
            temp_c: 22.0,
            rh_pct: 35.0,
            wind_kmh: 10.0,
            precip_mm: 0.0,
        };
        let result = calc_dmc(20.0, w, 7); // July
        assert!(result > 20.0, "expected DMC to rise, got {result}");
    }

    #[test]
    fn freezing_temp_gives_no_drying() {
        let w = NoonWeather {
            temp_c: -5.0,
            rh_pct: 60.0,
            wind_kmh: 10.0,
            precip_mm: 0.0,
        };
        let result = calc_dmc(20.0, w, 1);
        assert_eq!(result, 20.0);
    }
}