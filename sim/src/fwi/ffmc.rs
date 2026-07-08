//! Fine Fuel Moisture Code (FFMC).
//!
//! Represents moisture content of litter and fine cured fuels
//! (~0.25 kg/m^2 dry weight). Drives ISI. Van Wagner (1987).

use super::NoonWeather;

/// Compute today's FFMC from yesterday's FFMC and today's noon weather.
pub fn calc_ffmc(ffmc_prev: f64, w: NoonWeather) -> f64 {
    // Convert yesterday's FFMC to yesterday's fine fuel moisture content (%).
    let mo = 147.2 * (101.0 - ffmc_prev) / (59.5 + ffmc_prev);

    // Rain phase: only rainfall above a 0.5 mm canopy-interception threshold
    // actually wets the fuel.
    let mo = if w.precip_mm > 0.5 {
        let rf = w.precip_mm - 0.5;
        let mut mr = mo
            + 42.5 * rf * (-100.0 / (251.0 - mo)).exp() * (1.0 - (-6.93 / rf).exp());
        if mo > 150.0 {
            mr += 0.0015 * (mo - 150.0).powi(2) * rf.sqrt();
        }
        mr.min(250.0)
    } else {
        mo
    };

    // Equilibrium moisture content for drying (Ed) and wetting (Ew),
    // both functions of RH and temperature.
    let ed = 0.942 * w.rh_pct.powf(0.679)
        + 11.0 * ((w.rh_pct - 100.0) / 10.0).exp()
        + 0.18 * (21.1 - w.temp_c) * (1.0 - (-0.115 * w.rh_pct).exp());
    let ew = 0.618 * w.rh_pct.powf(0.753)
        + 10.0 * ((w.rh_pct - 100.0) / 10.0).exp()
        + 0.18 * (21.1 - w.temp_c) * (1.0 - (-0.115 * w.rh_pct).exp());

    let m = if mo < ed && mo < ew {
        // Wetting phase.
        let k1 = 0.424 * (1.0 - ((100.0 - w.rh_pct) / 100.0).powf(1.7))
            + 0.0694 * w.wind_kmh.sqrt() * (1.0 - ((100.0 - w.rh_pct) / 100.0).powi(8));
        let kw = k1 * 0.581 * (0.0365 * w.temp_c).exp();
        ew - (ew - mo) * 10f64.powf(-kw)
    } else if mo > ed {
        // Drying phase.
        let ko = 0.424 * (1.0 - (w.rh_pct / 100.0).powf(1.7))
            + 0.0694 * w.wind_kmh.sqrt() * (1.0 - (w.rh_pct / 100.0).powi(8));
        let kd = ko * 0.581 * (0.0365 * w.temp_c).exp();
        ed + (mo - ed) * 10f64.powf(-kd)
    } else {
        // Between Ew and Ed: no change.
        mo
    };

    // Convert moisture content back to the FFMC code and clamp to [0, 101].
    let ffmc = 59.5 * (250.0 - m) / (147.2 + m);
    ffmc.clamp(0.0, 101.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_rain_dries_from_default_startup() {
        // Warm, dry, breezy day from the standard startup FFMC of 85 should
        // dry the fuel further, i.e. push FFMC up.
        let w = NoonWeather {
            temp_c: 20.0,
            rh_pct: 40.0,
            wind_kmh: 15.0,
            precip_mm: 0.0,
        };
        let result = calc_ffmc(85.0, w);
        assert!(result > 85.0, "expected drying to raise FFMC, got {result}");
    }

    #[test]
    fn heavy_rain_lowers_ffmc() {
        let w = NoonWeather {
            temp_c: 15.0,
            rh_pct: 80.0,
            wind_kmh: 10.0,
            precip_mm: 20.0,
        };
        let result = calc_ffmc(90.0, w);
        assert!(result < 90.0, "expected rain to lower FFMC, got {result}");
    }
}