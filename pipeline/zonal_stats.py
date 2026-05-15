"""
zonal_stats.py
--------------
Computes per-neighbourhood zonal statistics:
  - mean summer LST
  - tree canopy fraction
  - impervious surface fraction
Then merges into a single GeoDataFrame for modeling.
"""

import geopandas as gpd
import pandas as pd
import numpy as np
from rasterstats import zonal_stats
from pathlib import Path

PROCESSED_DIR = Path("data/processed")
BOUNDARY_PATH = Path("data/raw/boundaries/toronto_neighbourhoods.geojson")
LANDCOVER_PATH = Path("data/raw/landcover/toronto_landcover.tif")

# NLCD class codes relevant to Toronto analysis
TREE_CANOPY_CLASSES = [10]
IMPERVIOUS_CLASSES  = [50]


def compute_lst_stats(neighbourhoods: gpd.GeoDataFrame) -> pd.DataFrame:
    lst_path = PROCESSED_DIR / "toronto_mean_summer_lst.tif"
    stats = zonal_stats(
        neighbourhoods,
        str(lst_path),
        stats=["mean", "min", "max", "std"],
        nodata=np.nan
    )
    df = pd.DataFrame(stats)
    df.columns = ["lst_mean", "lst_min", "lst_max", "lst_std"]
    return df


def compute_landcover_fractions(neighbourhoods: gpd.GeoDataFrame) -> pd.DataFrame:
    """Compute tree canopy % and impervious % per neighbourhood."""
    stats = zonal_stats(
        neighbourhoods,
        str(LANDCOVER_PATH),
        categorical=True,
        nodata=0
    )
    records = []
    for s in stats:
        total = sum(s.values()) if s else 1
        tree = sum(s.get(c, 0) for c in TREE_CANOPY_CLASSES) / total * 100
        impervious = sum(s.get(c, 0) for c in IMPERVIOUS_CLASSES) / total * 100
        records.append({"canopy_pct": tree, "impervious_pct": impervious})
    return pd.DataFrame(records)


def build_neighbourhood_dataset() -> gpd.GeoDataFrame:
    neighbourhoods = gpd.read_file(BOUNDARY_PATH).to_crs("EPSG:4326")

    print("Computing LST zonal stats...")
    lst_df = compute_lst_stats(neighbourhoods)

    print("Computing land cover fractions...")
    lc_df = compute_landcover_fractions(neighbourhoods)

    result = pd.concat([neighbourhoods.reset_index(drop=True), lst_df, lc_df], axis=1)
    result = result.dropna(subset=["lst_mean"])

    out_path = PROCESSED_DIR / "toronto_neighbourhoods_stats.geojson"
    result.to_file(out_path, driver="GeoJSON")
    print(f"Neighbourhood dataset saved to {out_path}")
    return result


if __name__ == "__main__":
    build_neighbourhood_dataset()
