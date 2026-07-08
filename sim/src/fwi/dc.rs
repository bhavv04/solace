//! Drought Code (DC).
//!
//! Represents average moisture of deep, compact organic layers
//! (~25 kg/m^2), reflecting seasonal drought over roughly 2 months of
//! weather history. Drives BUI along with DMC. Van Wagner (1987).

use super::{NoonWeather, DC_DAYLENGTH};

/// Compute today's DC from yesterday's DC, today's noon weather, and month.
pub fn calc_dc(dc_prev: f64, w: NoonWeather, month: u32) -> f64 {
    let lf = DC_DAYLENGTH[((month.clamp(1, 12)) - 1) as usize];

    // Rain phase.
    let dc_after_rain = if w.precip_mm > 2.8 {
        let rd = 0.83 * w.precip_mm - 1.27;
        let qo = 800.0 * (-dc_prev / 400.0).exp();
        let qr = qo + 3.937 * rd;
        let dr = 400.0 * (800.0 / qr).ln();
        dr.max(0.0)
    } else {
        dc_prev
    };

    // Drying phase.
    let v = (0.36 * (w.temp_c + 2.8) + lf).max(0.0);

    dc_after_rain + v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_summer_day_increases_dc() {
        let w = NoonWeather {
            temp_c: 25.0,
            rh_pct: 30.0,
            wind_kmh: 10.0,
            precip_mm: 0.0,
        };
        let result = calc_dc(100.0, w, 7);
        assert!(result > 100.0, "expected DC to rise, got {result}");
    }

    #[test]
    fn heavy_rain_lowers_dc() {
        let w = NoonWeather {
            temp_c: 15.0,
            rh_pct: 70.0,
            wind_kmh: 10.0,
            precip_mm: 30.0,
        };
        let result = calc_dc(300.0, w, 6);
        assert!(result < 300.0, "expected DC to fall after heavy rain, got {result}");
    }
}