# This is a sample Python script.
import argparse
import pandas as pd

from login import login
from scrape import CountryScraper, GlobalScraper, RegionScraper, SubRegionScraper


# Press ⌃R to execute it or replace it with your code.
# Press Double ⇧ to search everywhere for classes, files, tool windows, actions, and settings.


def main():
    parser = argparse.ArgumentParser()
    region = parser.add_mutually_exclusive_group()
    parser.add_argument("--output", required=True)
    region.add_argument("--local",
                        action="store_true")
    region.add_argument("--region",
                        action="store_true")
    region.add_argument("--country",
                        action="store_true")
    region.add_argument("--world",
                        action="store_true")

    list_type = parser.add_mutually_exclusive_group()
    list_type.add_argument("--life",
                           action="store_true")
    list_type.add_argument("--ytd",
                           action="store_true")
    list_type.add_argument("--mtd",
                           action="store_true")
    list_type.add_argument("--day",
                           action="store_true")

    time = parser.add_mutually_exclusive_group()
    time.add_argument("--month")
    time.add_argument("--range")
    time.add_argument("--all-months",
                      action="store_true")
    time.add_argument("--year",
                      action="store_true")

    args = parser.parse_args()
    session = login()
    counties = pd.read_csv("counties_test.csv")
    scraper = SubRegionScraper(args, counties) if args.local else RegionScraper(args, counties) if args.region \
        else CountryScraper(args, counties) if args.country else GlobalScraper(args, counties)
    data = scraper.scrape_data(session)
    data.to_csv(args.output, index=False)


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    main()
