# solace

A wildfire spread simulator for the boreal forest, built on the Canadian
Forest Fire Behaviour Prediction (FBP) System — validated against the
2016 Fort McMurray fire.

## What this is

Most wildfire "simulators" built as portfolio projects are dashboards:
they visualize historical fire data or run a black-box ML model with no
grounding in fire physics. Solace does something different — it
implements the actual operational fire behaviour model used by Canadian
fire agencies, drives it with real weather and fuel data, and checks its
output against what actually happened on the ground during a real fire.

The goal is a simulator that, given a fuel map, a weather record, and an
ignition point, predicts how a fire's perimeter grows over time — and
gets it approximately right when checked against the real perimeter
progression of Fort McMurray 2016.

## Why FBP, not Rothermel

The Rothermel (1972) fire spread model is the standard in US fire
science, but it's calibrated to US fuel classifications. Fort McMurray
burned through boreal spruce, which the Canadian **FBP System** models
natively as fuel type C-2. FBP is empirically calibrated against
Canadian fires (rather than derived from lab-scale energy-balance
physics like Rothermel), which makes it the more scientifically
appropriate choice for a Canadian boreal fire — and it's what real
Canadian fire behaviour analysts actually use.

## Architecture

```
sim/          Rust — simulation core
  fwi/        Fire Weather Index System (FFMC, DMC, DC, ISI, BUI)
  fbp/        Fire Behaviour Prediction System (fuel types, ROS, elliptical growth)
  grid.rs     Spatial grid: fuel type, elevation, moisture per cell
  propagation.rs   Wavelet-based perimeter growth over the grid

pipeline/     Python — data acquisition & preprocessing
  fetch_era5.py            hourly weather for the burn window
  fetch_fuel_types.py      NRCan fuel type layer, clipped to AOI
  fetch_dem.py             elevation / slope
  fetch_cwfis_perimeters.py   actual fire perimeter history (ground truth)
  build_grid.py            merges the above into sim/'s input format

validation/   Python — scoring simulated vs. actual perimeters
  dice_score.py    Sørensen-Dice coefficient per timestep

viz/          Perimeter-growth-over-time animation
```

Python handles data acquisition and validation, where the geospatial
tooling (xarray, rasterio, geopandas) does the heavy lifting. Rust runs
the actual grid simulation, where performance matters once you're
propagating fire across a fine-grained grid over multiple weeks of
fire growth.

## Status

- [x] FWI System (FFMC, DMC, DC, ISI, BUI) — Van Wagner (1987)
- [x] FBP ROS for C-2, D-1, M-1/M-2 — validated against a published
      reference case (Wotton, Alexander & Taylor 2009): FFMC=92,
      wind=15 km/h, BUI=64 → ROS=17.16 m/min, matched to within rounding
- [ ] Elliptical fire growth (head/flank/back spread → 2D shape)
- [ ] Spatial grid + wavelet propagation
- [ ] Real data ingestion (ERA5, NRCan fuel types, DEM)
- [ ] Validation against CWFIS Fort McMurray 2016 perimeters

## Data sources (planned)

- **Weather:** ERA5 hourly reanalysis
- **Fuel types:** NRCan Canadian Forest Fire Behaviour Prediction System
  fuel type layer
- **Terrain:** SRTM / Canadian DEM
- **Ground truth:** CWFIS (Canadian Wildland Fire Information System)
  historical fire perimeters

## Running the tests

```
cd sim
cargo test
```