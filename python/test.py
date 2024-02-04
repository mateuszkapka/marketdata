#!/usr/bin/env python
from datetime import time

import aggregator
import filters


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


agg = aggregator.Aggregator("../WSE_marketdata.parquet","../WSE_symbology.parquet", filters.SymbolFilter('PKO'))
agg.registerAggregate(testAggregate)
agg.run()
x = 1
