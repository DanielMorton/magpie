import multiprocessing

import pandas as pd
import time

from bs4 import BeautifulSoup
from itertools import product

from tqdm import tqdm


class Scraper:
    base_url = "https://ebird.org/targets"
    months = {1: 'January', 2: 'February',
              3: 'March', 4: 'April',
              5: 'May', 6: 'June',
              7: 'July', 8: 'August',
              9: 'September', 10: 'October',
              11: 'November', 12: 'December'}

    def __init__(self, args, counties):
        self.output = args.output
        self.counties = counties
        self.params_list = self.parse_params(args)
        self.num_cores = int(args.num_cores)

    @staticmethod
    def parse_params(args):
        params = {}

        if args.life:
            params['t2'] = "life"
        elif args.ytd:
            params['t2'] = "year"
        elif args.mtd:
            params['t2'] = "month"
        elif args.day:
            params['t2'] = "day"
        else:
            raise Exception("Time parameter missing.")

        if args.month:
            params['bmo'] = int(args.month)
            params['emo'] = int(args.month)
        elif args.range:
            start, end = args.range.split('-')
            start, end = int(start), int(end)
            if start < 1 or start > 12 or end < 1 or end > 12:
                raise Exception("Month endpoints out of range")
            params['bmo'] = start
            params['emo'] = end
        elif args.year:
            params['bmo'] = 1
            params['emo'] = 12
        elif args.all_months:
            params_list = []
            for n in range(1, 13):
                p = params.copy()
                p['bmo'] = n
                p['emo'] = n
                params_list.append(p)
            return params_list
        else:
            raise Exception("Time of Year parameter missing.")

        return [params]

    def loc_params(self, params, row):
        pass

    @staticmethod
    def parse_percent(soup):
        return [float(d['title'].split('% ')[0])
                for d in soup.find_all('div', {'class': 'ResultsStats-stats'})]

    @staticmethod
    def parse_species(soup):
        native = soup.find('section', {'aria-labelledby': 'native-and-naturalized'})
        if not native:
            return pd.DataFrame({'common name': []}), True
        species = [[s.text.strip() for s in d.find('a').contents]
                   for d in native.find_all('div', {'class': 'SpecimenHeader'})
                   if d.find('a')]
        if not species:
            return pd.DataFrame({'common name': []}), True
        if len(species[0]) > 1:
            common_name = []
            scientific_name = []
            for s in species:
                common_name.append(s[0] if len(s[0]) else None)
                scientific_name.append(s[1])
            return pd.DataFrame({'common name': common_name, 'scientific name': scientific_name}), False
        else:
            return pd.DataFrame({'common name': species}), False

    def scrape_page(self, params, row, session):
        sleep = 1
        percent = []
        params = self.loc_params(params, row)
        while not percent:
            r = session.get(self.base_url, params=params)
            soup = BeautifulSoup(r.content, 'html.parser')
            has_species = len([int(s.text) for s in soup.find_all('strong') if s.has_attr('class')]) > 0
            if not has_species:
                time.sleep(sleep)
                sleep *= 2
                continue
            df, is_empty = self.parse_species(soup)
            if is_empty:
                return df
            percent = self.parse_percent(soup)[:df.shape[0]]
        df['percent'] = percent
        df['country'] = row['country']
        df['region'] = row['region']
        df['sub region'] = row['sub_region']
        df['code'] = row['sub_region_code']
        df['start month'], df['end month'] = Scraper.months[params['bmo']], Scraper.months[params['emo']]
        if df['common name'].isna().sum() == df.shape[0]:
            df.drop(columns=['common name'], inplace=True)
        return df

    def scrape_data(self, session):
        with multiprocessing.Pool(self.num_cores) as pool:
            results = [pool.apply_async(self.scrape_page, (params, row, session)) for
                       params, (_, row) in product(self.params_list, self.counties.iterrows())]
            results = [r.get() for r in tqdm(results, total=len(results))]
        return pd.concat(results).reset_index(drop=True)


class GlobalScraper(Scraper):

    def __init__(self, args, counties):
        super().__init__(args, counties)

    def loc_params(self, params, row):
        params['r1'] = row['sub_region_code']
        params['r2'] = 'world'
        return params


class CountryScraper(Scraper):

    def __init__(self, args, counties):
        super().__init__(args, counties)

    def loc_params(self, params, row):
        params['r1'] = row['sub_region_code']
        params['r2'] = row['country_code']
        return params


class RegionScraper(Scraper):

    def __init__(self, args, counties):
        super().__init__(args, counties)

    def loc_params(self, params, row):
        params['r1'] = row['sub_region_code']
        params['r2'] = row['region_code']
        return params


class SubRegionScraper(Scraper):

    def __init__(self, args, counties):
        super().__init__(args, counties)

    def loc_params(self, params, row):
        params['r1'] = row['sub_region_code']
        params['r2'] = row['sub_region_code']
        return params
