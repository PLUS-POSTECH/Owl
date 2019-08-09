# pip install gql
from gql import Client
from gql.transport.requests import RequestsHTTPTransport

transport = RequestsHTTPTransport("http://localhost:4466/", use_json=True)
client = Client(transport=transport, fetch_schema_from_transport=True)
