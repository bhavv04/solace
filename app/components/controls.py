"""
controls.py
-----------
Sidebar controls: layer toggle and the what-if canopy slider.
"""

from dash import html, dcc
import dash_bootstrap_components as dbc


def controls_component():
    return dbc.Card([
        dbc.CardBody([
            html.H5("Controls", className="card-title"),
            html.Hr(),

            html.Label("Layer", className="fw-bold"),
            dcc.RadioItems(
                id="layer-toggle",
                options=[
                    {"label": " Heat (LST)", "value": "lst"},
                    {"label": " Land Cover", "value": "landcover"},
                ],
                value="lst",
                className="mb-3"
            ),

            html.Hr(),

            html.Label("What-if: add canopy cover", className="fw-bold"),
            html.P(
                "Drag to simulate adding tree cover city-wide "
                "and see projected temperature change.",
                className="text-muted small"
            ),
            dcc.Slider(
                id="canopy-slider",
                min=0, max=30, step=1, value=0,
                marks={0: "0%", 10: "+10%", 20: "+20%", 30: "+30%"},
                tooltip={"placement": "bottom", "always_visible": True}
            ),
            html.Div(id="canopy-impact-text", className="mt-2 text-success small"),

            html.Hr(),

            html.Label("Colour scale", className="fw-bold"),
            dcc.RadioItems(
                id="colorscale-toggle",
                options=[
                    {"label": " Heat (Red-Green)", "value": "RdYlGn_r"},
                    {"label": " Sequential (Inferno)", "value": "inferno"},
                ],
                value="RdYlGn_r",
                className="mb-3"
            ),
        ])
    ], className="h-100")
