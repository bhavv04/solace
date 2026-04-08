"""
cooling_model.py
----------------
Optional Random Forest model for non-linear LST prediction.
Used to compare against OLS and validate cooling estimates.
"""

import pandas as pd
import geopandas as gpd
import numpy as np
from sklearn.ensemble import RandomForestRegressor
from sklearn.model_selection import cross_val_score
from sklearn.preprocessing import StandardScaler
import joblib
from pathlib import Path

PROCESSED_DIR = Path("data/processed")
MODELS_DIR = Path("models")

FEATURES = ["canopy_pct", "impervious_pct"]
TARGET = "lst_mean"


def load_data() -> pd.DataFrame:
    gdf = gpd.read_file(PROCESSED_DIR / "toronto_neighbourhoods_stats.geojson")
    return pd.DataFrame(gdf.drop(columns="geometry")).dropna(subset=FEATURES + [TARGET])


def train(df: pd.DataFrame):
    X = df[FEATURES]
    y = df[TARGET]

    rf = RandomForestRegressor(n_estimators=200, random_state=42, n_jobs=-1)
    scores = cross_val_score(rf, X, y, cv=5, scoring="r2")
    print(f"CV R² scores: {scores}")
    print(f"Mean R²: {scores.mean():.3f} ± {scores.std():.3f}")

    rf.fit(X, y)
    joblib.dump(rf, MODELS_DIR / "random_forest.pkl")
    print("Random Forest model saved.")
    return rf


def feature_importance(rf: RandomForestRegressor):
    for name, imp in zip(FEATURES, rf.feature_importances_):
        print(f"  {name}: {imp:.3f}")


if __name__ == "__main__":
    df = load_data()
    rf = train(df)
    feature_importance(rf)
