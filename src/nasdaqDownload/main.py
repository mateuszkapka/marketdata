import json
import requests

API_KEY = 'tboFXoNQ-DMy71TaSwPy'

query = f"https://data.nasdaq.com/api/v3/datatables/MER/F1.xml?" \
        f"&mapcode=-5370" \
        f"&compnumber=39102" \
        f"&reporttype=A" \
        f"&qopts.columns=reportdate,amount" \
        f"&api_key={API_KEY}"

bytes_data = requests.get(query).content
data = json.loads(bytes_data.decode('utf-8'))
results = data['results']
a=1