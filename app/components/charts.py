"""
charts.py
---------
Scatter plot: canopy % vs mean LST per neighbourhood,
sized by area, coloured by impervious %.
"""

from dash import dcc, html
import plotly.express as px
import geopandas as gpd
from pathlib import Path

DATA_PATH = Path("data/processed/toronto_final.geojson")


def build_scatter_figure():
    if not DATA_PATH.exists():
        return px.scatter(title="No data yet — run the pipeline first.")

    gdf = gpd.read_file(DATA_PATH)
    df = gdf.drop(columns="geometry")

    fig = px.scatter(
        df,
        x="canopy_pct",
        y="lst_mean",
        color="impervious_pct",
        color_continuous_scale="OrRd",
        hover_name="AREA_NAME",
        trendline="ols",
        labels={
            "canopy_pct": "Tree Canopy (%)",
            "lst_mean": "Mean Summer LST (°C)",
            "impervious_pct": "Impervious Surface (%)"
        },
        title="Canopy cover vs. surface temperature by neighbourhood"
    )
    fig.update_layout(height=380)
    return fig


def scatter_component():
    return html.Div([
        dcc.Graph(id="canopy-scatter", figure=build_scatter_figure())
    ])
