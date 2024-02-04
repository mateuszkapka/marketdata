import datetime
import os

def get_nano_timestamp(datetime_obj):
    unix_timestamp = datetime_obj.timestamp()
    nanoseconds = int(unix_timestamp * 1e9)
    return nanoseconds

def convert_nano(nano_timestamp):
    seconds = nano_timestamp / 1e9  # 1e9 nanoseconds = 1 second
    datetime_obj = datetime.datetime.fromtimestamp(seconds)
    return datetime_obj

def check_pickled(object):
    current_directory = os.path.dirname(os.path.abspath(__file__))
    file_name = str(object + '.pickle')
    file_path = os.path.join(current_directory, file_name)

    if os.path.isfile(file_path):
        return True
    else:
        return False
