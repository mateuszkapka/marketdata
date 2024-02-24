from typing import NamedTuple

from regionConfig.base import RegionConfig
import csv

TICK_TABLES_PATH = "../tick_tables_wse.csv"
SYMBOL_MAPPING_PATH= "../tick_mapping_wse.csv"


class TickMapping(NamedTuple):
    id: int
    lower_bound: float
    tick_size: float

class SymbollMapping(NamedTuple):
    isin: str
    name: str
    symbol: str
    tickId:int

class WSE(RegionConfig):
    def __init__(self):
        super().__init__()
        self.load_tick_tables()

    def load_tick_tables(self):
        self.tick_table = {}
        self.stock_mapping = {}

        with open(TICK_TABLES_PATH, newline='') as csvfile:
            reader = csv.DictReader(csvfile, delimiter=',', quotechar='|')
            for row in reader:
                tick_mapping = TickMapping(
                    id=int(row["TableId"]),
                    lower_bound=float(row["lower_bound"].strip('"')),
                    tick_size=float(row["tick_size"].strip('"'))
                )

                self.tick_table.setdefault(tick_mapping.id, []).append(tick_mapping)

        for key, value in self.tick_table.items():
            value.sort(key=lambda tick_mapping: tick_mapping.lower_bound)

        with open(SYMBOL_MAPPING_PATH, newline='') as csvfile:
            reader = csv.DictReader(csvfile, delimiter=',', quotechar='|')
            for row in reader:
                mapping = SymbollMapping(
                    isin = row["ISIN"],
                    name = row["Name"],
                    symbol=row["Ticker"],
                    tickId=int(row["Tick table ID -  UTP system"].strip('"'))
                )

                self.stock_mapping[mapping.symbol] = mapping

    def get_tick_size(self, price, symbol) -> float:
        mapping = self.stock_mapping.get(symbol, None)
        if mapping is None:
            return 0
        tick_id = mapping.tickId
        tick_table = self.tick_table[tick_id]

        for tick_ladder in tick_table:
            if price > tick_ladder.lower_bound:
                return tick_ladder.tick_size

        return tick_table[-1].tick_size

    def get_marketdata_path(self, date) -> str:
        return "../normalised_data/WSE.parquet"

    def get_symbology_path(self, date) -> str:
        return "../symbology_data/WSE.parquet"

