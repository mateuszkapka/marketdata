
# https://docs.databento.com/knowledge-base/new-users/dbn-encoding

import pyarrow.parquet as pq
import databento as db
import json

file = 'xnas-itch-20240122.mbp-1.dbn.zst'
path = f'/Users/stephaniejury/marketdata/sample_nasdaq_databento/{file}'


# # Read saved .dbn.zst
# data = db.DBNStore.from_file(path)
#
# data.to_parquet(
#     "nasdaq.parquet",
#     pretty_ts = True,
#     price_type = "float",
#     map_symbols = True,
# )

# Open the Parquet file
parquet_file = pq.ParquetFile('nasdaq.parquet')

# Read the first 10 rows
first_10_rows = parquet_file.read().to_pandas().head(10)

# Print the first 10 rows
print(first_10_rows)

# file = 'metadata.json'
# file = 'condition.json'
# file = 'manifest.json'
# file = 'symbology.json'
path = f'/Users/stephaniejury/marketdata/sample_nasdaq_databento/{file}'

with open(path, 'r') as file:
    data = json.load(file)

# Process the JSON data (e.g., print it)
print(data)

