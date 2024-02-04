from typing import NamedTuple


class Quote (NamedTuple):
    timestamp: str
    symbol: str
    bid_price: float
    bid_size: int
    ask_price: float
    ask_size: int
    market_period: str
    type: str

class Trade(NamedTuple):
    timestamp: str
    symbol: str
    trade_price: float
    trade_volume: int
    type: str