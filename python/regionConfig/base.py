from abc import abstractmethod


class RegionConfig:

    @abstractmethod
    def get_region_name(self):
        pass

    @abstractmethod
    def get_tick_size(self, price, symbol) -> float:
        pass