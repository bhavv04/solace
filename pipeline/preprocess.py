"""
preprocess.py
-------------
Processes Landsat 8/9 LST GeoTIFFs exported from Google Earth Engine.
Clips to Toronto neighbourhood boundary and computes mean summer LST
across all years (2015-2024).
"""

import numpy as np
import rasterio
import rasterio.mask
import geopandas as gpd
from pathlib import Path

RAW_DIR = Path("data/raw/landsat")
PROCESSED_DIR = Path("data/processed")
BOUNDARY_PATH = Path("data/raw/boundaries/toronto_neighbourhoods.geojson")

PROCESSED_DIR.mkdir(parents=True, exist_ok=True)


def clip_to_toronto(tiff_path: Path, out_path: Path):
    """Clip LST raster to Toronto neighbourhood boundary."""
    boundary = gpd.read_file(BOUNDARY_PATH).to_crs("EPSG:4326")
    geoms = [geom.__geo_interface__ for geom in boundary.geometry]

    with rasterio.open(tiff_path) as src:
        clipped, transform = rasterio.mask.mask(
            src, geoms, crop=True, nodata=np.nan, filled=True
        )
        profile = src.profile.copy()
        profile.update(
            transform=transform,
            height=clipped.shape[1],
            width=clipped.shape[2],
            nodata=np.nan,
            dtype="float32"
        )
        clipped = clipped.astype(np.float32)
        with rasterio.open(out_path, "w", **profile) as dst:
            dst.write(clipped)


def compute_mean_lst():
    """Stack all clipped yearly tiles and compute mean LST per pixel."""
    tiffs = sorted(PROCESSED_DIR.glob("lst_clipped_*.tif"))
    if not tiffs:
        raise FileNotFoundError("No clipped tiles found.")

    print(f"  Stacking {len(tiffs)} yearly tiles...")
    arrays = []
    profile = None
    for t in tiffs:
        with rasterio.open(t) as src:
            arr = src.read(1).astype(np.float32)
            arr = np.where(arr == src.nodata, np.nan, arr)
            arrays.append(arr)
            if profile is None:
                profile = src.profile.copy()

    stack = np.stack(arrays, axis=0)
    mean_lst = np.nanmean(stack, axis=0)

    out_path = PROCESSED_DIR / "toronto_mean_summer_lst.tif"
    profile.update(dtype="float32", count=1, nodata=np.nan)
    with rasterio.open(out_path, "w", **profile) as dst:
        dst.write(mean_lst, 1)

    print(f"Mean summer LST saved to {out_path}")
    return out_path


if __name__ == "__main__":
    tiffs = sorted(RAW_DIR.glob("toronto_lst_*.tif"))
    print(f"Found {len(tiffs)} Landsat LST files...")

    for tiff_path in tiffs:
        year = tiff_path.stem.split("_")[-1]
        out_path = PROCESSED_DIR / f"lst_clipped_{year}.tif"

        if out_path.exists():
            print(f"  Skipping {year} (already processed)")
            continue

        print(f"  Clipping {year}...")
        try:
            clip_to_toronto(tiff_path, out_path)
        except Exception as e:
            print(f"  ERROR on {year}: {e}")
            continue

    print("\nComputing mean summer LST across all years...")
    compute_mean_lst()
    print("Done.")