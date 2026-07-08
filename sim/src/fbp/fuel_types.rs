//! FBP fuel type definitions and their ROS coefficients.
//!
//! `a`, `b`, `c` parameterize the Chapman-Richards spread curve
//! RSI = a * (1 - exp(-b * ISI))^c, and `q`/`bui_o` parameterize the
//! buildup effect multiplier. Values below are taken from the standard
//! FBP System fuel type table (ST-X-3 / fbp97 reference implementation).

/// Coefficients for the basic "curve" fuel types, where RSI is computed
/// directly from a, b, c with no blending against another fuel type.
#[derive(Debug, Clone, Copy)]
pub struct CurveParams {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub q: f64,
    pub bui_o: f64,
}

/// FBP fuel types relevant to Fort McMurray-style boreal fires.
///
/// M-1 and M-2 (boreal mixedwood) are not simple curves — they blend the
/// C-2 and D-1 spread rates by percent conifer (PC). M-2 additionally
/// dampens the deciduous (D-1) contribution to 20% of its normal value,
/// reflecting that green mixedwood carries fire less readily than
/// leafless mixedwood.
#[derive(Debug, Clone, Copy)]
pub enum FuelType {
    /// Boreal Spruce — the dominant fuel type for Fort McMurray 2016.
    C2,
    /// Leafless Aspen.
    D1,
    /// Boreal Mixedwood - Leafless. `percent_conifer` in [0, 100].
    M1 { percent_conifer: f64 },
    /// Boreal Mixedwood - Green. `percent_conifer` in [0, 100].
    M2 { percent_conifer: f64 },
}

impl FuelType {
    /// Curve coefficients for this fuel type, where applicable.
    /// Returns `None` for blended types (M1/M2), which are handled
    /// separately in `ros.rs`.
    pub fn curve_params(&self) -> Option<CurveParams> {
        match self {
            FuelType::C2 => Some(CurveParams {
                a: 110.0,
                b: 0.0282,
                c: 1.5,
                q: 0.70,
                bui_o: 64.0,
            }),
            FuelType::D1 => Some(CurveParams {
                a: 30.0,
                b: 0.0232,
                c: 1.6,
                q: 0.90,
                bui_o: 32.0,
            }),
            FuelType::M1 { .. } | FuelType::M2 { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c2_matches_reference_table() {
        let p = FuelType::C2.curve_params().unwrap();
        assert_eq!(p.a, 110.0);
        assert_eq!(p.b, 0.0282);
        assert_eq!(p.c, 1.5);
        assert_eq!(p.q, 0.70);
        assert_eq!(p.bui_o, 64.0);
    }
}