# Solace

Urban heat island analysis for Toronto — mapping where the city burns and modeling how much tree cover is needed to cool it down.

Built with NASA MODIS land surface temperature data, NLCD land cover, and Toronto neighbourhood boundaries.

## What it does

- Maps mean summer land surface temperature (LST) across all 158 Toronto neighbourhoods (2015–2024)
- Correlates LST with tree canopy % and impervious surface % per neighbourhood
- Models how many percentage points of added canopy cover are needed to drop temperature by 2°C
- Interactive what-if slider: simulate adding city-wide tree cover and see projected cooling in real time

## Data sources

| Dataset | Source | Notes |
|---|---|---|
| MOD11A1 v6.1 | NASA Earthdata | Daily 1km LST, tile h12v04 |
| NLCD 2021 | USGS | Land cover classification |
| Neighbourhood boundaries | City of Toronto Open Data | 158 neighbourhoods |

## Setup

```bash
git clone https://github.com/bhavv04/solace.git
cd solace
python -m venv venv && source venv/bin/activate
pip install -r requirements.txt
cp .env.example .env  # add your NASA Earthdata credentials
```

## Run the pipeline

```bash
python -m pipeline.download       # download MODIS tiles (~2-3 GB)
python -m pipeline.preprocess     # convert HDF4 → GeoTIFF, clip to Toronto
python -m pipeline.zonal_stats    # compute per-neighbourhood stats
python -m models.regression       # fit OLS, compute cooling coefficients
```

## Run the app

```bash
python -m app.app
```

Open http://localhost:8050

## Key finding

> Neighbourhoods with under 10% tree canopy averaged X°C hotter than the city median in summer. Adding 15% canopy cover city-wide is projected to reduce peak LST by ~2°C.

*(Update with real numbers after running the pipeline)*

## Stack

Python · rasterio · geopandas · rasterstats · scikit-learn · Plotly Dash · earthaccess
