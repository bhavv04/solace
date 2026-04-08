"""
layout.py
---------
Top-level layout for the Solace app.
"""

from dash import html, dcc
import dash_bootstrap_components as dbc
from app.components.map import map_component
from app.components.charts import scatter_component
from app.components.controls import controls_component


def create_layout():
    return dbc.Container([
        dbc.Row([
            dbc.Col([
                html.H1("Solace", className="display-4"),
                html.P(
                    "Toronto urban heat island analysis — "
                    "where the city burns and what green can fix it.",
                    className="lead text-muted"
                ),
                html.Hr()
            ])
        ]),

        dbc.Row([
            dbc.Col([controls_component()], width=3),
            dbc.Col([map_component()], width=9),
        ], className="mb-4"),

        dbc.Row([
            dbc.Col([scatter_component()], width=12),
        ]),

        dcc.Store(id="coefficients-store"),
        dcc.Store(id="neighbourhood-data-store"),

    ], fluid=True, className="p-4")
