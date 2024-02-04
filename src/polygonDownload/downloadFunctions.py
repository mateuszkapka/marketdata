import requests
import json
from datetime import datetime, timezone
import pickle
from utils import get_nano_timestamp, convert_nano, check_pickled


class Downloads:
    API_KEY = 'lsyR9wzgDH0pYHG1637D8SDx5QfGGuPk'

    @staticmethod
    def get_all_tickers():
        if check_pickled('tickers'):
            with open('tickers.pickle', 'rb') as file:
                loaded_list = pickle.load(file)
            return loaded_list

        else:
            alpha_num = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
                         'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']

            tickers_list = []

            for char in alpha_num:
                # print(char)
                query = f'https://api.polygon.io/v3/reference/' \
                                   f'tickers?ticker.gte={char}&' \
                                   f'type=CS&exchange=XNAS&' \
                                   f'active=true&' \
                                   f'order=asc&' \
                                   f'limit=1000&' \
                                   f'sort=ticker&' \
                                   f'apiKey={Downloads.API_KEY}'

                bytes_data = requests.get(query).content
                data = json.loads(bytes_data.decode('utf-8'))
                results = data['results']
                tickers = {result['ticker'] for result in results}
                tickers_list.extend(list(tickers))

            with open('tickers.pickle', 'wb') as file:
                pickle.dump(tickers_list, file)

            return sorted(list(set(tickers_list)))

    @staticmethod
    def get_quotes(unique_ticker_list):
        results_list = []

        for ticker in unique_ticker_list:
            print(ticker)
            end_datetime = datetime(2023, 12, 31, 0, 0, tzinfo=timezone.utc)
            start_datetime = datetime(2018, 1, 1, 0, 0, tzinfo=timezone.utc)

            end_nano = get_nano_timestamp(end_datetime)
            start_nano = get_nano_timestamp(start_datetime)

            results_list_temp = []
            while start_nano < end_nano:
                quotes_query = f'https://api.polygon.io/v3/quotes/' \
                               f'{ticker}?' \
                               f'timestamp.gte={start_nano}&' \
                               f'order=asc&' \
                               f'limit=50000&' \
                               f'sort=timestamp&' \
                               f'apiKey={Downloads.API_KEY}'

                bytes_data = requests.get(quotes_query).content
                data = json.loads(bytes_data.decode('utf-8'))
                results = data['results']
                results_list_temp.extend(results)
                start_nano = results[-1]['participant_timestamp']
                print(convert_nano(start_nano))

            results_list.extend(results_list_temp)

        return results_list
