"""
map.py
------
Choropleth map of Toronto neighbourhoods coloured by mean summer LST.
"""

from dash import dcc, html
import plotly.express as px
import geopandas as gpd
from pathlib import Path

DATA_PATH = Path("data/processed/toronto_final.geojson")


def build_map_figure(canopy_increase: float = 0.0):
    if not DATA_PATH.exists():
        return px.scatter(title="No data yet — run the pipeline first.")

    gdf = gpd.read_file(DATA_PATH)
    gdf["adjusted_lst"] = gdf["lst_mean"] - (canopy_increase * abs(gdf.get("canopy_coef", 0.3)))
    gdf["hover_text"] = (
        "<b>" + gdf["AREA_NAME"].astype(str) + "</b><br>" +
        "Mean LST: " + gdf["adjusted_lst"].round(1).astype(str) + "°C<br>" +
        "Canopy: " + gdf["canopy_pct"].round(1).astype(str) + "%<br>" +
        "Impervious: " + gdf["impervious_pct"].round(1).astype(str) + "%<br>" +
        "Trees needed (+2°C buffer): +" + gdf["trees_needed_pct"].round(1).astype(str) + "%"
    )

    fig = px.choropleth_mapbox(
        gdf,
        geojson=gdf.geometry.__geo_interface__,
        locations=gdf.index,
        color="adjusted_lst",
        color_continuous_scale="RdYlGn_r",
        mapbox_style="carto-positron",
        zoom=10,
        center={"lat": 43.7, "lon": -79.42},
        opacity=0.75,
        hover_name="AREA_NAME",
        custom_data=["hover_text"],
        labels={"adjusted_lst": "LST (°C)"}
    )
    fig.update_traces(hovertemplate="%{customdata[0]}<extra></extra>")
    fig.update_layout(margin={"r": 0, "t": 0, "l": 0, "b": 0}, height=520)
    return fig


def map_component():
    return html.Div([
        dcc.Graph(id="lst-map", figure=build_map_figure(), config={"scrollZoom": True})
    ])
