from abc import abstractmethod
from datetime import time, timedelta, datetime

from pandas import Timestamp


class SliceSchedule:
    def __init__(self, on_trigger):
        self.schedule = []
        self.last_slice_index = -1
        self.on_trigger = on_trigger

    def trigger_maybe(self, current_time: Timestamp):
        if self.last_slice_index + 1 == len(self.schedule):
            return

        if self.last_slice_index == -1 or \
                current_time.time() > self.schedule[self.last_slice_index + 1]:
            self.last_slice_index += 1
            self.on_trigger(current_time, self.schedule[self.last_slice_index])


class WallClockSliceSchedule(SliceSchedule):
    def __init__(self, start_time: time, end_time: time, slice_interval: timedelta, on_trigger):
        super().__init__(on_trigger)
        current_time = datetime.combine(datetime.today(), start_time)
        exit_time = datetime.combine(datetime.today(), end_time)
        while current_time <= exit_time:
            self.schedule.append(current_time.time())
            current_time += slice_interval

        if self.schedule[-1] != exit_time:
            self.schedule.append(end_time)
