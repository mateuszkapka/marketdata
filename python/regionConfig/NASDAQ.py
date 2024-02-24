from typing import NamedTuple

from regionConfig.base import RegionConfig
import csv

class NASDAQ(RegionConfig):
    def __init__(self):
        super().__init__()



    def get_tick_size(self, price, symbol) -> float:
        return 0.01

    def get_marketdata_path(self, date) -> str:
        return "../normalised_data/NASDAQ.parquet"

    def get_symbology_path(self, date) -> str:
        return "../symbology_data/NASDAQ.parquet"

