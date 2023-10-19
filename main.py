# This is a sample Python script.
import argparse
import pandas as pd

from login import login
from scrape import SubRegionScraper, CountryScraper, GlobalScraper, RegionScraper


# Press ⌃R to execute it or replace it with your code.
# Press Double ⇧ to search everywhere for classes, files, tool windows, actions, and settings.


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--num-cores", default=8)
    parser.add_argument("--output", required=True)
    parser.add_argument("--hotspot", default=None)

    list_type = parser.add_mutually_exclusive_group()
    list_type.add_argument("--local",
                           action="store_true")
    list_type.add_argument("--region",
                           action="store_true")
    list_type.add_argument("--country",
                           action="store_true")
    list_type.add_argument("--world",
                           action="store_true")

    date_range = parser.add_mutually_exclusive_group()
    date_range.add_argument("--life",
                            action="store_true")
    date_range.add_argument("--ytd",
                            action="store_true")
    date_range.add_argument("--mtd",
                            action="store_true")
    date_range.add_argument("--day",
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

    if args.hotspot and (args.region or args.country):
        raise Exception("Hotspot must use local or world list type.")

    regions = pd.read_csv(args.hotspot) if args.hotspot else pd.read_csv("regions.csv")
    scraper = SubRegionScraper(args, regions) if args.local \
        else GlobalScraper(args, regions) if args.hotspot \
        else RegionScraper(args, regions) if args.region \
        else CountryScraper(args, regions) if args.country \
        else GlobalScraper(args, regions)
    print("Start Scraping")
    data = scraper.scrape_data(session)
    data.to_csv(args.output, index=False)


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    main()
