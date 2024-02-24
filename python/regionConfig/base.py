from abc import abstractmethod


class RegionConfig:

    @abstractmethod
    def get_region_name(self):
        pass

    @abstractmethod
    def get_tick_size(self, price, symbol) -> float:
        pass

    @abstractmethod
    def get_marketdata_path(self, date) -> str:
        pass

    @abstractmethod
    def get_symbology_path(self, date) -> str:
        pass