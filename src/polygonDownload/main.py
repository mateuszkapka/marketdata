import multiprocessing
from downloadFunctions import Downloads
import pickle

if __name__ == '__main__':
    tickers = Downloads.get_all_tickers()
    #tickers = tickers[0:10]
    pool = multiprocessing.Pool(processes=4)
    results = pool.map(Downloads.get_quotes, [tickers])
    pool.close()
    pool.join()

    with open('quotes.pickle', 'wb') as file:
        pickle.dump(results, file)





