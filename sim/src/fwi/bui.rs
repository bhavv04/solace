//! Buildup Index (BUI).
//!
//! Combines DMC and DC into a rating of total fuel available for
//! combustion. Feeds into the FBP System's ROS equations alongside ISI.
//! Van Wagner (1987).

/// Compute BUI from today's DMC and DC.
pub fn calc_bui(dmc: f64, dc: f64) -> f64 {
    if dmc <= 0.4 * dc {
        0.8 * dmc * dc / (dmc + 0.4 * dc)
    } else {
        dmc - (1.0 - 0.8 * dc / (dmc + 0.4 * dc)) * (0.92 + (0.0114 * dmc).powf(1.7))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bui_increases_with_dmc_and_dc() {
        let low = calc_bui(20.0, 100.0);
        let high = calc_bui(60.0, 300.0);
        assert!(high > low, "expected higher DMC/DC to raise BUI");
    }

    #[test]
    fn bui_is_nonnegative() {
        let result = calc_bui(1.0, 1.0);
        assert!(result >= 0.0);
    }
}