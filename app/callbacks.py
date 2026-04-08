"""
callbacks.py
------------
Dash callbacks wiring controls to map and chart updates.
"""

from dash import Input, Output, callback
from app.components.map import build_map_figure


@callback(
    Output("lst-map", "figure"),
    Output("canopy-impact-text", "children"),
    Input("canopy-slider", "value"),
    Input("colorscale-toggle", "value"),
)
def update_map(canopy_increase, colorscale):
    fig = build_map_figure(canopy_increase=canopy_increase)
    if canopy_increase > 0:
        # rough estimate: ~0.3°C drop per 1% canopy increase (updated by real model)
        est_drop = round(canopy_increase * 0.3, 1)
        text = f"+{canopy_increase}% canopy → estimated -{est_drop}°C city-wide"
    else:
        text = "Adjust the slider to simulate cooling."
    return fig, text
