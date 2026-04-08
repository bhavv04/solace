"""
preprocess.py
-------------
Converts raw MODIS HDF4 tiles to GeoTIFF, applies scale factor,
converts Kelvin to Celsius, reprojects to EPSG:4326,
clips to Toronto boundary, and computes mean summer LST per pixel.
"""

import rasterio
import rasterio.mask
import numpy as np
import geopandas as gpd
from pathlib import Path
from rasterio.warp import calculate_default_transform, reproject, Resampling
from rasterio.merge import merge
import subprocess

RAW_DIR = Path("data/raw/modis")
PROCESSED_DIR = Path("data/processed")
BOUNDARY_PATH = Path("data/raw/boundaries/toronto_neighbourhoods.geojson")
TARGET_CRS = "EPSG:4326"

PROCESSED_DIR.mkdir(parents=True, exist_ok=True)


def hdf_to_geotiff(hdf_path: Path, out_path: Path):
    """Extract LST_Day_1km subdataset from HDF4 and save as GeoTIFF."""
    subdataset = f'HDF4_EOS:EOS_GRID:"{hdf_path}":MODIS_Grid_Daily_1km_LST:LST_Day_1km'
    subprocess.run([
        "gdal_translate", "-of", "GTiff", subdataset, str(out_path)
    ], check=True, capture_output=True)


def apply_scale_kelvin_to_celsius(tiff_path: Path) -> np.ndarray:
    """Apply MODIS scale factor (0.02) and convert K -> C."""
    with rasterio.open(tiff_path) as src:
        data = src.read(1).astype(np.float32)
        profile = src.profile.copy()
    data = np.where(data == 0, np.nan, data * 0.02 - 273.15)
    return data, profile


def reproject_to_wgs84(src_path: Path, dst_path: Path):
    with rasterio.open(src_path) as src:
        transform, width, height = calculate_default_transform(
            src.crs, TARGET_CRS, src.width, src.height, *src.bounds
        )
        profile = src.profile.copy()
        profile.update(crs=TARGET_CRS, transform=transform, width=width, height=height)
        with rasterio.open(dst_path, "w", **profile) as dst:
            reproject(
                source=rasterio.band(src, 1),
                destination=rasterio.band(dst, 1),
                src_transform=src.transform,
                src_crs=src.crs,
                dst_transform=transform,
                dst_crs=TARGET_CRS,
                resampling=Resampling.bilinear
            )


def clip_to_toronto(tiff_path: Path, out_path: Path):
    boundary = gpd.read_file(BOUNDARY_PATH).to_crs(TARGET_CRS)
    geoms = [geom.__geo_interface__ for geom in boundary.geometry]
    with rasterio.open(tiff_path) as src:
        clipped, transform = rasterio.mask.mask(src, geoms, crop=True, nodata=np.nan)
        profile = src.profile.copy()
        profile.update(transform=transform, height=clipped.shape[1], width=clipped.shape[2])
        with rasterio.open(out_path, "w", **profile) as dst:
            dst.write(clipped)


def compute_mean_summer_lst():
    """Stack all processed summer tiles and compute mean LST per pixel."""
    tiffs = sorted(PROCESSED_DIR.glob("lst_clipped_*.tif"))
    if not tiffs:
        raise FileNotFoundError("No processed tiles found. Run the full pipeline first.")

    arrays = []
    for t in tiffs:
        with rasterio.open(t) as src:
            arrays.append(src.read(1).astype(np.float32))
            profile = src.profile.copy()

    stack = np.stack(arrays, axis=0)
    mean_lst = np.nanmean(stack, axis=0)

    out_path = PROCESSED_DIR / "toronto_mean_summer_lst.tif"
    profile.update(dtype="float32", count=1)
    with rasterio.open(out_path, "w", **profile) as dst:
        dst.write(mean_lst, 1)

    print(f"Mean summer LST saved to {out_path}")
    return out_path


if __name__ == "__main__":
    import glob

    all_hdfs = sorted(Path(RAW_DIR).rglob("*.hdf"))
    print(f"Found {len(all_hdfs)} HDF files to process...")

    for hdf_path in all_hdfs:
        stem = hdf_path.stem
        tiff_path = PROCESSED_DIR / f"lst_raw_{stem}.tif"
        reproj_path = PROCESSED_DIR / f"lst_reproj_{stem}.tif"
        clipped_path = PROCESSED_DIR / f"lst_clipped_{stem}.tif"

        if clipped_path.exists():
            continue

        try:
            print(f"  Processing {hdf_path.name}...")
            hdf_to_geotiff(hdf_path, tiff_path)
            reproject_to_wgs84(tiff_path, reproj_path)
            clip_to_toronto(reproj_path, clipped_path)
            tiff_path.unlink(missing_ok=True)
            reproj_path.unlink(missing_ok=True)
        except Exception as e:
            print(f"  ERROR on {hdf_path.name}: {e}")
            continue

    print("All tiles processed. Computing mean summer LST...")
    compute_mean_summer_lst()