//! Rate of spread (ROS) calculation for the FBP System.
//!
//! Two-stage calculation, following ST-X-3 / fbp97 reference:
//! 1. RSI ("initial spread rate") from ISI alone, via the fuel type's
//!    Chapman-Richards curve: RSI = a * (1 - exp(-b * ISI))^c
//! 2. ROS = BE * RSI, where BE ("buildup effect") scales RSI by how far
//!    today's BUI sits from the fuel type's reference buildup index BUIo:
//!    BE = exp(50 * ln(q) * (1/BUI - 1/BUIo))
//!
//! M-1/M-2 (boreal mixedwood) don't have their own a/b/c curve — instead
//! RSI is a blend of the C-2 and D-1 RSI values weighted by percent
//! conifer, following FBP System equations 27/28. Note the BUI effect is
//! applied once, after blending — not separately per component.

use super::fuel_types::{CurveParams, FuelType};

/// RSI from ISI alone, via the fuel type's spread curve.
fn calc_rsi(p: CurveParams, isi: f64) -> f64 {
    p.a * (1.0 - (-p.b * isi).exp()).powf(p.c)
}

/// Buildup effect multiplier: how today's BUI compares to the fuel type's
/// reference buildup index, BUIo. BE = 1 when BUI == BUIo.
fn calc_be(p: CurveParams, bui: f64) -> f64 {
    if bui > 0.0 && p.bui_o > 0.0 {
        (50.0 * p.q.ln() * (1.0 / bui - 1.0 / p.bui_o)).exp()
    } else {
        1.0
    }
}

/// Head fire rate of spread (m/min) for a given fuel type, ISI, and BUI.
///
/// `isi` and `bui` are the Initial Spread Index and Buildup Index for the
/// cell/timestep in question (from `fwi::FwiState`).
pub fn calc_ros(fuel: FuelType, isi: f64, bui: f64) -> f64 {
    let rsi = match fuel {
        FuelType::C2 | FuelType::D1 => {
            let p = fuel.curve_params().expect("curve params exist for C2/D1");
            calc_rsi(p, isi)
        }
        FuelType::M1 { percent_conifer } => {
            let pc = percent_conifer / 100.0;
            let c2 = calc_rsi(FuelType::C2.curve_params().unwrap(), isi);
            let d1 = calc_rsi(FuelType::D1.curve_params().unwrap(), isi);
            pc * c2 + (1.0 - pc) * d1
        }
        FuelType::M2 { percent_conifer } => {
            let pc = percent_conifer / 100.0;
            let c2 = calc_rsi(FuelType::C2.curve_params().unwrap(), isi);
            let d1 = calc_rsi(FuelType::D1.curve_params().unwrap(), isi);
            // D-1 contribution damped to 20% — green mixedwood carries
            // fire less readily than leafless mixedwood (FBP eqn 28).
            pc * c2 + 0.2 * (1.0 - pc) * d1
        }
    };

    // BUI effect: for blended M1/M2 fuels, apply using the dominant
    // conifer fraction's own reference curve (C-2), since the blend is a
    // weighted RSI, not a weighted BE.
    let be_params = match fuel {
        FuelType::C2 | FuelType::M1 { .. } | FuelType::M2 { .. } => {
            FuelType::C2.curve_params().unwrap()
        }
        FuelType::D1 => FuelType::D1.curve_params().unwrap(),
    };
    let be = calc_be(be_params, bui);

    let ros = be * rsi;
    ros.max(0.000001) // FBP reference clamps ROS to a small positive floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ros_increases_with_isi() {
        let low = calc_ros(FuelType::C2, 5.0, 60.0);
        let high = calc_ros(FuelType::C2, 20.0, 60.0);
        assert!(high > low, "expected higher ISI to raise ROS");
    }

    #[test]
    fn ros_at_reference_bui_uses_be_near_one() {
        // At BUI == BUIo (64 for C2), buildup effect should be ~1.0,
        // so ROS should closely match raw RSI.
        let isi = 10.0;
        let ros_at_buio = calc_ros(FuelType::C2, isi, 64.0);
        let p = FuelType::C2.curve_params().unwrap();
        let rsi = p.a * (1.0 - (-p.b * isi).exp()).powf(p.c);
        assert!(
            (ros_at_buio - rsi).abs() < 0.01,
            "expected ROS ~= RSI at BUI=BUIo, got ROS={ros_at_buio}, RSI={rsi}"
        );
    }

    #[test]
    fn higher_bui_increases_ros() {
        let low_bui = calc_ros(FuelType::C2, 10.0, 20.0);
        let high_bui = calc_ros(FuelType::C2, 10.0, 100.0);
        assert!(high_bui > low_bui, "expected higher BUI to raise ROS");
    }

    #[test]
    fn m1_blend_is_between_c2_and_d1() {
        let isi = 10.0;
        let bui = 60.0;
        let c2 = calc_ros(FuelType::C2, isi, bui);
        let d1 = calc_ros(FuelType::D1, isi, bui);
        let m1_50 = calc_ros(FuelType::M1 { percent_conifer: 50.0 }, isi, bui);
        let (lo, hi) = if c2 < d1 { (c2, d1) } else { (d1, c2) };
        assert!(
            m1_50 >= lo - 0.01 && m1_50 <= hi + 0.01,
            "expected 50% mixedwood ROS between C2 ({c2}) and D1 ({d1}), got {m1_50}"
        );
    }
}