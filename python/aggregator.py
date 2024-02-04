#!/usr/bin/env python
from abc import abstractmethod
from typing import NamedTuple
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
    def get_aggregate_value(self, slice: time):
        pass

class Aggregator:
    def __init__(self, market_data_path, filter: Filter = NoopFilter()):
        self.market_data_path = market_data_path
        self.aggregates = []
        self.slice_schedule = WallClockSliceSchedule(time(hour=9, minute=00),
                                                     time(hour=15, minute=50),
                                                     timedelta(minutes=5),
                                                     self.on_slice_triggered)
        self.filter = filter

    def registerAggregate(self, aggregate: Aggregate):
        self.aggregates.append(aggregate)
    def run(self):
        df = pd.read_parquet('/Users/annakapka/code/rust/marketdata/test.parquet', engine='pyarrow')

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
                for aggregate in self.aggregates:
                    aggregate.on_quote(quote)
            elif row.type == "Trade":
                trade = Trade(
                    row.timestamp,
                    row.symbol,
                    row.trade_price,
                    row.trade_volume,
                    row.type,
                )
                for aggregate in self.aggregates:
                    aggregate.on_trade(trade)
            else:
                raise Exception(f"Unrecognized type {row.type}")

    def on_slice_triggered(self, current_time, slice_time):
        pass


