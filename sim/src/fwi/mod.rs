//! Canadian Forest Fire Weather Index (FWI) System.
//!
//! Implements the three fuel moisture codes (FFMC, DMC, DC) and the two
//! fire behaviour indices (ISI, BUI) derived from them, following
//! Van Wagner (1987), "Development and Structure of the Canadian Forest
//! Fire Weather Index System," Canadian Forestry Service Technical Report 35.
//!
//! Each moisture code is computed daily from the previous day's value plus
//! a noon weather observation (temperature, relative humidity, wind speed,
//! 24h precipitation). ISI and BUI are then derived from those codes.

pub mod bui;
pub mod dc;
pub mod dmc;
pub mod ffmc;
pub mod isi;

pub use bui::calc_bui;
pub use dc::calc_dc;
pub use dmc::calc_dmc;
pub use ffmc::calc_ffmc;
pub use isi::calc_isi;

/// A single noon weather observation used to drive the FWI System.
#[derive(Debug, Clone, Copy)]
pub struct NoonWeather {
    /// Dry-bulb temperature, degrees Celsius.
    pub temp_c: f64,
    /// Relative humidity, percent (0-100).
    pub rh_pct: f64,
    /// 10 m open wind speed, km/h.
    pub wind_kmh: f64,
    /// 24-hour accumulated precipitation ending at noon, mm.
    pub precip_mm: f64,
}

/// Rolling state of the three fuel moisture codes, carried day to day.
#[derive(Debug, Clone, Copy)]
pub struct FwiState {
    pub ffmc: f64,
    pub dmc: f64,
    pub dc: f64,
}

impl Default for FwiState {
    /// Standard spring startup values (Van Wagner 1987).
    fn default() -> Self {
        Self {
            ffmc: 85.0,
            dmc: 6.0,
            dc: 15.0,
        }
    }
}

impl FwiState {
    /// Advance the state by one day given a noon weather observation and
    /// the month (1-12, used for day-length factors in DMC/DC).
    pub fn step(&self, weather: NoonWeather, month: u32) -> FwiState {
        let ffmc = calc_ffmc(self.ffmc, weather);
        let dmc = calc_dmc(self.dmc, weather, month);
        let dc = calc_dc(self.dc, weather, month);
        FwiState { ffmc, dmc, dc }
    }

    pub fn isi(&self, wind_kmh: f64) -> f64 {
        calc_isi(self.ffmc, wind_kmh)
    }

    pub fn bui(&self) -> f64 {
        calc_bui(self.dmc, self.dc)
    }
}

/// Effective day-length factor Le for DMC, by month (Jan=index 0).
/// Values for ~45-50N; Van Wagner 1987 Table 3.
pub(crate) const DMC_DAYLENGTH: [f64; 12] = [
    6.5, 7.5, 9.0, 12.8, 13.9, 13.9, 12.4, 10.9, 9.4, 8.0, 7.0, 6.0,
];

/// Day-length adjustment factor Lf for DC, by month (Jan=index 0).
/// Van Wagner 1987 Table 4 (values for the northern hemisphere).
pub(crate) const DC_DAYLENGTH: [f64; 12] = [
    -1.6, -1.6, -1.6, 0.9, 3.8, 5.8, 6.4, 5.0, 2.4, 0.4, -1.6, -1.6,
];