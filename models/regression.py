"""
regression.py
-------------
Fits OLS regression: LST ~ canopy_pct + impervious_pct
Extracts the cooling coefficient (degrees C per 1% canopy increase).
"""

import pandas as pd
import geopandas as gpd
import statsmodels.formula.api as smf
from pathlib import Path
import json

PROCESSED_DIR = Path("data/processed")
MODELS_DIR = Path("models")


def load_data() -> gpd.GeoDataFrame:
    return gpd.read_file(PROCESSED_DIR / "toronto_neighbourhoods_stats.geojson")


def fit_ols(gdf: gpd.GeoDataFrame):
    df = pd.DataFrame(gdf.drop(columns="geometry"))
    model = smf.ols("lst_mean ~ canopy_pct + impervious_pct", data=df).fit()
    print(model.summary())
    return model


def compute_trees_needed(gdf: gpd.GeoDataFrame, model, target_drop: float = 2.0) -> gpd.GeoDataFrame:
    """
    For each neighbourhood, compute how many percentage points of
    additional tree cover are needed to drop LST by target_drop degrees C.
    Uses the canopy_pct coefficient from the OLS model.
    """
    canopy_coef = model.params["canopy_pct"]  # negative value (more canopy = lower LST)
    # pct_needed = target_drop / |canopy_coef|
    pct_needed = target_drop / abs(canopy_coef)
    gdf = gdf.copy()
    gdf["trees_needed_pct"] = pct_needed
    gdf["projected_lst"] = gdf["lst_mean"] - target_drop
    gdf["canopy_gap"] = (gdf["canopy_pct"] + pct_needed).clip(upper=100) - gdf["canopy_pct"]
    return gdf


def save_coefficients(model, out_path: Path):
    coeffs = {
        "intercept": model.params["Intercept"],
        "canopy_coef": model.params["canopy_pct"],
        "impervious_coef": model.params["impervious_pct"],
        "r_squared": model.rsquared
    }
    with open(out_path, "w") as f:
        json.dump(coeffs, f, indent=2)
    print(f"Coefficients saved to {out_path}")


if __name__ == "__main__":
    gdf = load_data()
    model = fit_ols(gdf)
    gdf = compute_trees_needed(gdf, model)
    save_coefficients(model, MODELS_DIR / "ols_coefficients.json")
    gdf.to_file(PROCESSED_DIR / "toronto_final.geojson", driver="GeoJSON")
    print("Final dataset with cooling estimates saved.")
