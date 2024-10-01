from typing import List
import dash
from dash import dcc, html, Input, Output, State
import dash_bootstrap_components as dbc
import pandas as pd
import plotly.express as px
import plotly.graph_objects as go
import yfinance as yf

from data import db
from models import Form4


dash.register_page(__name__, path="/")

def create_graph_table(hist: pd.DataFrame, forms: List[Form4]):
    forms_data = []
    for form in forms:
        forms_data.append({
            "Date": form.formDate,
            "Price": form.averageSharePrice,
            "Title": f"{form.owner} {form.get_direction()} ${form.monetaryValue:,.2f}",
            "Symbol": form.symbol 
        })

    transactions_df = pd.DataFrame(forms_data)

    graph = html.Div()
    if hist is not None and not hist.empty:
        transactions_df.where(transactions_df["Symbol"] == hist.iloc[0]["Symbol"], inplace=True)

        # Create a Plotly figure for stock price and transaction points
        fig = go.Figure()

        # Add stock price line
        fig.add_trace(go.Scatter(
            x=hist.index, 
            y=hist['Close'], 
            mode='lines', 
            name='Stock Price ($)'
        ))

        # Add Form 4 transaction points
        if not transactions_df.empty:
            fig.add_trace(go.Scatter(
                x=transactions_df['Date'], 
                y=transactions_df['Price'],
                text=transactions_df['Title'],
                mode='markers', 
                marker=dict(color='black', size=10), 
                name=f"Form 4 Data"
            ))

        fig.update_layout(
            title=f"{hist.iloc[0]['Symbol']} Stock Price and Form 4 Transactions",
            xaxis_title="Date",
            yaxis_title="Price ($)",
            showlegend=True
        )

        graph = dcc.Graph(figure=fig)

    table_header = [
        html.Thead(html.Tr([
            html.Th("Date"),
            html.Th("Symbol"), 
            html.Th("Owner"), 
            html.Th("Company"), 
            html.Th(""),
        ]))]
    rows = [
        html.Tr([
            html.Td(form.formDate.strftime("%a %m/%d/%Y")),
            html.Td(form.symbol), 
            html.Td(form.owner), 
            html.Td(form.company), 
            html.Td([
                    html.A("View Form", className="btn btn-primary mx-2", target="_blank", href=form.formUrl)]),
        ])
        for form in forms
    ]

    table = dbc.Table(table_header + [html.Tbody(rows)], bordered=True, hover=True)

    # Return the layout
    return html.Div([
        graph,
        html.Br(),
        table
    ])

def layout():
    return html.Div([
        html.H2("Form Analysis"),
        dbc.Row([
            dbc.Col([
                dbc.Input(id="symbol-input", placeholder="Enter symbol", type="text"),
                dbc.Button("GO", id="symbol-go-button", n_clicks=0, color="primary", className="mt-2")
            ], width=6),
            dbc.Col([
                dbc.Input(id="owner-input", placeholder="Enter owner", type="text"),
                dbc.Button("GO", id="owner-go-button", n_clicks=0, color="primary", className="mt-2")
            ], width=6),
        ]),
        html.Br(),
        dcc.Loading(id="output-div")
    ])

@dash.callback(
    Output("output-div", "children"),
    [Input("symbol-go-button", "n_clicks"), Input("owner-go-button", "n_clicks")],
    [State("symbol-input", "value"), State("owner-input", "value")]
)
def query_forms(symbol_n_clicks, owner_n_clicks, symbol, owner):
    ctx = dash.callback_context

    if not ctx.triggered:
        return html.P("Enter a symbol or owner and click GO.")

    button_id = ctx.triggered[0]["prop_id"].split(".")[0]

    # If the symbol GO button was clicked
    if button_id == "symbol-go-button" and symbol:
        forms = db.query_forms({"symbol": symbol.upper()})

        if len(forms) == 0:
            return html.P(f"No forms found for {symbol.upper()}.")

        # Fetch the 1-year historical data for the symbol
        ticker = yf.Ticker(symbol.upper())
        hist = ticker.history(period="1y")
        hist["Symbol"] = symbol.upper()

        return create_graph_table(hist, forms)

    # If the owner GO button was clicked
    elif button_id == "owner-go-button" and owner:
        forms = db.query_forms({"owner": {"$regex": f".*{owner}.*", "$options": "i"}})

        if len(forms) == 0:
            return html.P(f"No forms found for owner {owner}.")

        # get most popular symbols by this owner
        counts = {form.symbol: 0 for form in forms}
        for form in forms:
            counts[form.symbol] += 1

        symbol = max(counts, key=counts.get)
        ticker = yf.Ticker(symbol.upper())
        hist = ticker.history(period="1y")
        hist["Symbol"] = symbol.upper()


        return create_graph_table(hist, forms)

    return html.P("Invalid input or action. Please enter a symbol or owner.")
