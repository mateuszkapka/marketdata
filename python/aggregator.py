    #!/usr/bin/env python
from abc import abstractmethod
from collections import namedtuple
from typing import NamedTuple, Type
from datetime import time, timedelta

import pandas as pd

from data import Quote, Trade
from filters import Filter, NoopFilter
from schedule import WallClockSliceSchedule

class Aggregate():

    @abstractmethod
    def on_quote(self, quote: Quote):
        pass

    @abstractmethod
    def on_trade(self, trade: Trade):
        pass

    @abstractmethod
    def compute_slice(self, slice: time):
        pass

class Aggregator:
    def __init__(self, market_data_path: str,symbology_path: str, filter: Filter = NoopFilter()):
        self.market_data_path = market_data_path
        self.aggregates = {}
        self.aggregate_values = {}
        self.slice_schedule = WallClockSliceSchedule(time(hour=9, minute=00),
                                                     time(hour=15, minute=50),
                                                     timedelta(minutes=5),
                                                     self.on_slice_triggered)
        self.filter = filter
        self.readSymbology(symbology_path)

    def readSymbology(self, symbology_path):
        df = pd.read_parquet(symbology_path, engine='pyarrow')
        Row = namedtuple("row", "symbol")
        self.symbology = [s for s in df["symbol"].unique().tolist() if  not self.filter.shouldFilter(Row(s))]

    def registerAggregate(self, aggregate: Type):
        for symbol in self.symbology:
            self.aggregates.setdefault(symbol, []).append(aggregate())

    def run(self):
        df = pd.read_parquet(self.market_data_path, engine='pyarrow')

        for row in df.itertuples():
            if self.filter.shouldFilter(row):
                continue

            self.slice_schedule.trigger_maybe(row.timestamp)

            if row.type == "Quote":
                quote = Quote(
                    row.timestamp,
                    row.symbol,
                    row.bid_price,
                    row.bid_size,
                    row.ask_price,
                    row.ask_size,
                    row.market_period,
                    row.type
                )
                for aggregate in self.aggregates[row.symbol]:
                    aggregate.on_quote(quote)
            elif row.type == "Trade":
                trade = Trade(
                    row.timestamp,
                    row.symbol,
                    row.trade_price,
                    row.trade_volume,
                    row.type,
                )
                for aggregate in self.aggregates[row.symbol]:
                    aggregate.on_trade(trade)
            else:
                raise Exception(f"Unrecognized type {row.type}")

        return self.aggs_to_dataframe()

    def on_slice_triggered(self, current_time, slice_time):
        for symbol in self.symbology:
            for agg in self.aggregates[symbol]:
                aggregte_value = agg.compute_slice(slice_time)
                (self.aggregate_values.setdefault(symbol, {})
                    .setdefault(type(agg).__name__.replace("Aggregate", ""), {})
                    .setdefault(slice_time, aggregte_value))

    def aggs_to_dataframe(self):
        dataset = []
        for symbol, aggs in self.aggregate_values.items():
            for agg_name, slices in aggs.items():
                for slice, value in slices.items():
                    dataset.append([symbol, slice,agg_name, value])

        return pd.DataFrame(dataset, columns=["symbol", "slice","aggregate", "value"])





