from abc import abstractmethod
from typing import Tuple, Optional


class Filter:
    @abstractmethod
    def shouldFilter(self, row) -> bool:
        pass

    def parquetFilter(self) -> Optional[Tuple[str,str,str]]:
        return None

class NoopFilter(Filter):
    def shouldFilter(self, row):
        return False

class SymbolFilter(Filter):
    def __init__(self, symbol: str):
        self.symbol = symbol

    def shouldFilter(self, row) -> bool:
        return row.symbol != self.symbol

    def parquetFilter(self):
        return ('symbol','=',self.symbol)
