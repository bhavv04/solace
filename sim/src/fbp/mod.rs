//! Canadian Forest Fire Behaviour Prediction (FBP) System.
//!
//! Implements head fire rate of spread (ROS) following:
//! Forestry Canada Fire Danger Group. 1992. "Development and Structure of
//! the Canadian Forest Fire Behavior Prediction System." Inf. Rep. ST-X-3.
//!
//! Coefficients (a, b, c, q, BUIo) and the general spread-rate/BUI-effect
//! formulation are cross-checked against the NRCan reference implementation
//! (fbp97.c, B.M. Wotton et al.), which is the canonical software
//! implementation of ST-X-3 used operationally by CWFIS.
//!
//! Only the fuel types relevant to boreal fires (C-2, D-1, M-1, M-2) are
//! implemented initially. Adding more is a matter of adding entries to
//! `fuel_types.rs` — the ROS math itself is fuel-type-agnostic.

pub mod fuel_types;
pub mod ros;

pub use fuel_types::FuelType;
pub use ros::calc_ros;