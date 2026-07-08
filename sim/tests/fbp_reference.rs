//! Validation against published FBP System reference cases.
//!
//! Reference case source: Wotton, B.M., Alexander, M.E., Taylor, S.W.
//! 2009. "Updates and revisions to the 1992 Canadian Forest Fire
//! Behavior Prediction System." NRCan Inf. Rep. GLC-X-10.
//!
//! The report's worked example: a fire in FBP fuel type C-2 (boreal
//! spruce) on level terrain with FFMC=92, wind speed=15 km/h, and
//! BUI=64 gives LB=1.98, ROS=17.16 m/min, FROS=4.88 m/min, and
//! BROS=2.16 m/min. Only ROS is checked here; FROS/BROS/LB will be
//! added once the elliptical growth model (fbp/ellipse.rs) exists.

// NOTE: these `use` paths assume `fwi` and `fbp` are declared `pub mod`
// in main.rs, or that the crate exposes a lib target re-exporting them.
// If this test doesn't compile because the modules aren't visible from
// an integration test, that's a project-structure fix (typically:
// promote fwi/fbp into a src/lib.rs and have main.rs depend on the
// library), not a formula problem — flag it and we'll restructure.
use solace_sim::fbp::{calc_ros, FuelType};
use solace_sim::fwi::isi::calc_isi;

#[test]
fn c2_matches_wotton_alexander_taylor_2009_reference_case() {
    let ffmc = 92.0;
    let wind_kmh = 15.0;
    let bui = 64.0;

    let isi = calc_isi(ffmc, wind_kmh);
    let ros = calc_ros(FuelType::C2, isi, bui);

    let expected_ros = 17.16;
    let tolerance = 0.05; // published value is itself rounded to 2 dp

    assert!(
        (ros - expected_ros).abs() < tolerance,
        "expected ROS ~= {expected_ros} m/min (Wotton et al. 2009 reference case), got {ros:.4}"
    );
}