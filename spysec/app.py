import dash
from dash import html
import dash_bootstrap_components as dbc


app = dash.Dash(
    __name__,
    title="SpySEC",
    external_stylesheets=[dbc.themes.BOOTSTRAP, dbc.icons.FONT_AWESOME],
    use_pages=True,
    suppress_callback_exceptions=True
)

server = app.server

app.layout = html.Div(
    [
        html.A("SpySEC", href="/", className="h1 text-decoration-none"),
        html.Hr(),
        dash.page_container
    ],
    className="m-2",
)


if __name__ == "__main__":
    app.run(debug=True)