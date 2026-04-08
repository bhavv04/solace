# Solace

> Mapping where Toronto burns — and modeling the green infrastructure needed to cool it down.

Solace is an urban heat island analysis pipeline and interactive dashboard built on a decade of NASA satellite data. It quantifies mean summer land surface temperature (LST) across all 158 Toronto neighbourhoods, correlates heat with land cover composition, and produces a neighbourhood-level model of how much additional tree canopy is required to reduce peak temperatures by 2°C.

---

## Overview

Urban heat islands disproportionately affect dense, low-canopy neighbourhoods — often the same communities with the least green infrastructure. Solace makes that disparity legible and actionable by combining remote sensing data with regression modeling and an interactive what-if interface.

**Pipeline stages:**
1. Download MOD11A1 daily LST tiles from NASA Earthdata (summers 2015–2024)
2. Convert HDF4 → GeoTIFF, apply scale factor, convert Kelvin → Celsius
3. Reproject and clip to Toronto boundary
4. Compute mean summer LST and land cover fractions per neighbourhood (zonal statistics)
5. Fit OLS regression to extract cooling coefficient (°C per 1% canopy increase)
6. Serve results via interactive Plotly Dash application

---

## Data Sources

| Dataset | Source | Resolution | Notes |
|---|---|---|---|
| MOD11A1 v6.1 | NASA Earthdata | 1 km daily | Land surface temperature, tile h12v04 |
| NLCD 2021 | USGS | 30 m | Land cover classification |
| Neighbourhood profiles | City of Toronto Open Data | Polygon | 158 neighbourhoods |

---

## Key Finding

> *Update after running the full pipeline.*
>
> Neighbourhoods with under 10% tree canopy averaged **X°C** hotter than the city median in summer.
> The OLS model estimates that adding **X percentage points** of canopy cover reduces peak LST by 2°C —
> a threshold that varies significantly by neighbourhood based on existing impervious surface fraction.

---

## Setup

**Prerequisites:** Python 3.10+, GDAL installed system-wide (`winget install OSGeo.GDAL` on Windows, `brew install gdal` on macOS)

```bash
git clone https://github.com/bhavv04/solace.git
cd solace

# Windows
python -m venv venv
venv\Scripts\Activate.ps1

# macOS / Linux
python -m venv venv && source venv/bin/activate

pip install -r requirements.txt
copy .env.example .env   # add NASA Earthdata credentials
```

Register for a free NASA Earthdata account at `urs.earthdata.nasa.gov` and add your credentials to `.env`.

---

## Running the Pipeline

```bash
# 1. Download MODIS LST tiles (~2–3 GB)
python pipeline/download.py

# 2. Convert, reproject, clip, compute mean summer LST
python pipeline/preprocess.py

# 3. Zonal statistics per neighbourhood
python pipeline/zonal_stats.py

# 4. Fit regression model, compute cooling coefficients
python models/regression.py
```

---

## Running the App

```bash
python app/app.py
```

Open `http://localhost:8050`

The dashboard includes a choropleth map of neighbourhood LST, a canopy vs. temperature scatter plot, and a what-if slider to simulate city-wide tree cover increases and their projected cooling effect.

---

## Project Structure

```
solace/
├── pipeline/
│   ├── download.py        # NASA Earthdata acquisition
│   ├── preprocess.py      # HDF4 → GeoTIFF, reproject, clip, mean LST
│   └── zonal_stats.py     # Per-neighbourhood LST + land cover stats
├── models/
│   ├── regression.py      # OLS model + cooling coefficient
│   └── cooling_model.py   # Optional Random Forest model
├── app/
│   ├── app.py             # Dash entry point
│   ├── layout.py          # App layout
│   ├── callbacks.py       # Interactivity
│   └── components/        # Map, charts, controls
├── data/
│   ├── raw/               # Downloaded source data (not tracked in git)
│   └── processed/         # Pipeline outputs (not tracked in git)
├── notebooks/             # EDA and development notebooks
├── requirements.txt
└── .env.example
```

---

## Roadmap

**Pipeline**
- [x] Set up project structure and dependencies
- [x] NASA Earthdata authentication via `earthaccess`
- [x] Download MOD11A1 LST tiles for Toronto (2015–2024)
- [ ] Preprocess HDF4 → GeoTIFF, reproject, clip to Toronto boundary
- [ ] Compute mean summer LST per pixel (2015–2024 average)
- [ ] Zonal statistics per neighbourhood (LST + land cover fractions)

**Modeling**
- [ ] OLS regression: LST ~ canopy_pct + impervious_pct
- [ ] Extract cooling coefficient (°C per 1% canopy increase)
- [ ] Compute trees-needed estimate per neighbourhood
- [ ] Random Forest model for non-linear comparison
- [ ] Cross-validate and report R² for both models

**Dashboard**
- [ ] Choropleth map of neighbourhood LST
- [ ] Land cover toggle (heat vs. canopy layer)
- [ ] Canopy vs. LST scatter plot with OLS trendline
- [ ] What-if slider: simulate city-wide canopy increase
- [ ] Hover tooltips with neighbourhood-level stats

**Deployment & Documentation**
- [ ] Deploy Dash app to Render (free tier)
- [ ] Fill in Key Finding section with real model results
- [ ] Add screenshots to README
- [ ] Write methodology notes in `notebooks/`

**Stretch Goals**
- [ ] Time-lapse animation of LST change 2015–2024
- [ ] Equity layer: overlay neighbourhood income data
- [ ] Expand to other Canadian cities (Vancouver, Montreal)
- [ ] Export neighbourhood cooling report as PDF

---
