#!/usr/bin/env python
import aggregator
import filters


class testAggregate(aggregator.Aggregate):
    def __init__(self):
        self.index = 0
        self.trade_index = 0

    def on_quote(self, quote):
        if self.index < 10:
            self.index += 1
            print(f"Quote just happened {quote}")

    def on_trade(self, trade):
        if self.trade_index < 10:
            self.trade_index += 1
            print(f"Trade just happened {trade}")


agg = aggregator.Aggregator("test.parquet", filters.SymbolFilter('PKO'))
agg.registerAggregate(testAggregate())
agg.run()
