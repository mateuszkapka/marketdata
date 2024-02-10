#!/usr/bin/env python
from datetime import time

import aggregator
import filters
from regionConfig.WSE import WSE



class SpreadByTickSizeAggregate(aggregator.Aggregate):
    def __init__(self, region, symbol):
        self.last_bid = 0.0
        self.last_ask = 0.0
        self.region = region
        self.tick_series = []
        self.symbol = symbol

    def on_quote(self, quote):
        self.spread = quote.ask_price - quote.bid_price

    def on_trade(self, trade):
        tick_size = self.region.get_tick_size(trade.trade_price, trade.symbol)
        if tick_size != 0:
            self.tick_series.append(self.spread / tick_size)
            if self.symbol == "PKO":
                x = 5

    def compute_slice(self, slice):
        if self.symbol == "PKO":
                x = 5
        if len(self.tick_series) == 0:
            return 0

        value = sum(self.tick_series) / len(self.tick_series)
        return value

class testAggregate(aggregator.Aggregate):
    def __init__(self):
        self.index = 0
        self.trade_index = 0
        self.volume = 0

    def on_quote(self, quote):
        pass

    def on_trade(self, trade):
        self.volume += trade.trade_volume

    def compute_slice(self, slice: time):
        ret = self.volume
        self.volume = 0
        return ret


agg = aggregator.Aggregator("../WSE_marketdata.parquet",
                            "../WSE_symbology.parquet",
                            WSE())
agg.registerAggregate(SpreadByTickSizeAggregate)
result = agg.run()
x = 1
