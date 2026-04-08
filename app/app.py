"""
app.py
------
Entry point for the Solace Dash application.
"""

import dash
import dash_bootstrap_components as dbc
from app.layout import create_layout
from app import callbacks  # noqa: F401 - registers callbacks

app = dash.Dash(
    __name__,
    external_stylesheets=[dbc.themes.BOOTSTRAP],
    title="Solace — Toronto Urban Heat"
)

app.layout = create_layout()

if __name__ == "__main__":
    app.run(debug=True)
