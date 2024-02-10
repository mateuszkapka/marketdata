from abc import abstractmethod


class Filter:
    @abstractmethod
    def shouldFilter(self, row) -> bool:
        pass

class NoopFilter(Filter):
    def shouldFilter(self, row):
        return False

class SymbolFilter(Filter):
    def __init__(self, symbol: str):
        self.symbol = symbol

    def shouldFilter(self, row) -> bool:
        return row.symbol != self.symbol
