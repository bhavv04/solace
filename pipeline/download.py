"""
download.py
-----------
Downloads MODIS MOD11A1 land surface temperature tiles
covering Toronto (h12v04 tile) for summer months (June-August).
"""

import earthaccess
import os
from pathlib import Path
from dotenv import load_dotenv
import calendar

load_dotenv()

RAW_MODIS_DIR = Path("data/raw/modis")
RAW_MODIS_DIR.mkdir(parents=True, exist_ok=True)

# Toronto falls in MODIS tile h12v04
TORONTO_TILE = "h12v04"
SUMMER_MONTHS = ["06", "07", "08"]
YEARS = list(range(2015, 2025))


def authenticate():
    os.environ["EARTHDATA_USERNAME"] = os.getenv("EARTHDATA_USERNAME")
    os.environ["EARTHDATA_PASSWORD"] = os.getenv("EARTHDATA_PASSWORD")
    earthaccess.login(strategy="environment")

def search_modis_granules(year: int, month: str):
    """Search for MOD11A1 granules for a given year and summer month."""
    last_day = calendar.monthrange(year, int(month))[1]
    results = earthaccess.search_data(
        short_name="MOD11A1",
        version="061",
        temporal=(f"{year}-{month}-01", f"{year}-{month}-{last_day}"),
        granule_name=f"*{TORONTO_TILE}*"
    )
    return results


def download_all():
    authenticate()
    for year in YEARS:
        for month in SUMMER_MONTHS:
            print(f"Searching {year}-{month}...")
            granules = search_modis_granules(year, month)
            if not granules:
                print(f"  No granules found for {year}-{month}")
                continue
            print(f"  Found {len(granules)} granules, downloading...")
            earthaccess.download(granules, local_path=str(RAW_MODIS_DIR / str(year)))


if __name__ == "__main__":
    download_all()
