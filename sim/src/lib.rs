//! Solace — wildfire spread simulation core.
//!
//! Library root exposing the FWI (Fire Weather Index) and FBP (Fire
//! Behaviour Prediction) modules so they're usable both from the `solace-sim`
//! binary and from integration tests in `tests/`.

pub mod fbp;
pub mod fwi;