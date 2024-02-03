#!/usr/bin/env python
from abc import abstractmethod
from typing import NamedTuple

import pandas as pd

class Quote (NamedTuple):
    timestamp: str
    bid_price: float
    bid_size: int
    ask_price: float
    ask_size: int
    market_period: str
    type: str

class Trade(NamedTuple):
    timestamp: str
    trade_price: float
    trade_volume: int
    type: str

class Aggregate():
    @abstractmethod
    def on_quote(self, quote: Quote):
        pass

    @abstractmethod
    def on_trade(self, trade: Trade):
        pass

class Aggregator:
    def __init__(self, market_data_path):
        self.market_data_path = market_data_path
        self.aggregates = []

    def registerAggregate(self, aggregate: Aggregate):
        self.aggregates.append(aggregate)
    def run(self):
        df = pd.read_parquet('test.parquet', engine='pyarrow')

        for row in df.itertuples():
            if row.type == "Quote":
                quote = Quote(
                    row.timestamp,
                    row.bid_price,
                    row.bid_size,
                    row.ask_price,
                    row.ask_size,
                    row.market_period,
                    row.type
                )
                for aggregate in self.aggregates:
                    aggregate.on_quote(quote)
            elif row.type == "Trade":
                trade = Trade(
                    row.timestamp,
                    row.trade_price,
                    row.trade_volume,
                    row.type,
                )
                for aggregate in self.aggregates:
                    aggregate.on_trade(trade)
            else:
                raise Exception(f"Unrecognized type {row.type}")




