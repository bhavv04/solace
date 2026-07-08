use solace_sim::fbp::{self, FuelType};
use solace_sim::fwi::{FwiState, NoonWeather};

fn main() {
    // Quick smoke test: a warm, dry, breezy summer day in C-2 boreal spruce.
    let weather = NoonWeather {
        temp_c: 25.0,
        rh_pct: 30.0,
        wind_kmh: 20.0,
        precip_mm: 0.0,
    };

    let state = FwiState::default().step(weather, 7); // July
    let isi = state.isi(weather.wind_kmh);
    let bui = state.bui();
    let ros = fbp::calc_ros(FuelType::C2, isi, bui);

    println!("FWI state: {state:?}");
    println!("ISI: {isi:.2}, BUI: {bui:.2}");
    println!("C-2 head fire ROS: {ros:.2} m/min");
}