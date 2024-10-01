import dash
from dash import dcc, html, Input, Output, State
import dash_bootstrap_components as dbc

from data import db


dash.register_page(__name__, path="/index")


def layout():
    search_bar = dbc.Row([
        dbc.Col(dbc.Input(id='search-input', placeholder="Search by owner, company, or symbol", type='text')),
        dbc.Col(dbc.Button("Search", id='search-button', class_name="btn-primary"))
    ])
    
    return html.Div([
        search_bar,
        html.Br(),
        dcc.Loading(id='loading-output', children=[html.Div(id='results')])
    ])

@dash.callback(
    Output('results', 'children'),
    Input('search-button', 'n_clicks'),
    State('search-input', 'value'),
)
def update_results(n_clicks, search_value):
    if search_value and n_clicks is not None:
        query = [
            {'owner': {'$regex': f'.*{search_value}.*', '$options': 'i'}},
            {'company': {'$regex': f'.*{search_value}.*', '$options': 'i'}},
            {'symbol': {'$regex': f'.*{search_value}.*', '$options': 'i'}}
        ]
        forms = db.query_forms({'$or': query})
        if forms:
            table_header = [
                html.Thead(html.Tr([
                    html.Th("Date"),
                    html.Th("Symbol"), 
                    html.Th("Owner"), 
                    html.Th("Company"), 
                    html.Th(""),
                    html.Th(""),
                ]))]
            rows = [
                html.Tr([
                    html.Td(form.formDate.strftime("%m/%d/%Y")),
                    html.Td(form.symbol), 
                    html.Td(form.owner), 
                    html.Td(form.company), 
                    html.Td([html.A("Graph", className="btn btn-primary", href=f"/graph/{form.symbol}"),
                            html.A("View Form", className="btn btn-secondary mx-2", target="_blank", href=form.formUrl)]),
                ])
                for form in forms
            ]

            results = dbc.Table(table_header + [html.Tbody(rows)], bordered=True, hover=True)
        else:
            results = "No results found."
    else:
        results = "Please enter a search term."
    
    return results
