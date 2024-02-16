## Magpie
Magpie is a tool for scraping target bird species from eBird. Any hotspot
or geographic region has a "Target Species" feature. This feature all the
species found at that location that the user has not already seen in some
specified time period and geographic range. The most general form would
be

```{NUMBER} of species in {FIRST_LOCATION} that you need for your {SECOND_LOCATION} {TIME} list```

The first and second locations can be the same or the second location can
be a region that contains the first. `TIME` is usually life or year (as in
life list or year list) but can be month (current month, all years) or day
(current date, all years). The list will be all species seen in `SECOND_LOCATION`.

Two concrete examples are below:

#### 180 species observed in Sogn og Fjordane, NO that you need for your Sogn og Fjordane Life List

#### 130 species observed in Sogn og Fjordane, NO that you need for your World Life List

Note that, in this example, when `SECOND_LOCATION` covers a larger geographic area, the
number of target species goes down. In other examples it might stay the
same but will never go up.

Magpie works by collecting the data from multiple, indeed many, target
bird species pages and saving them in a single CSV file. Once saved
these files can be used to perform a variety of analyses. Examples
include determining what regions, or even hotspots, have the most species that can be
conveniently seen, and at what time of year or when and where a given
species is most likely to be found.

## Magpie and eBird Geography

Geographic regions in eBird can come in one of three levels. The coarsest,
and easiest to explain, is the country level. For the most part, this is
identical to the usual meaning of country but it does include some political
oddballs like Hong Kong and the Isle of Man.

Most, but not all, countries are divided into smaller geographic regions.
In the United States these would be the 50 states, in Canada the provinces
and territories, in the UK the four constituent countries (England, Scotland,
Wales, and Northern Ireland), and in most other countries their equivalent
of states or provinces. Since there is no generally applicable name, in Magpie
they are simply called regions.

In some, mostly larger, countries there is a second level of subdivision.
In the US (excluding Alaska where they are census regions) and UK these are
just counties. In other countries these region have other names. Once again,
there is no general name for these boundaries; Magpie uses the generic,
if unoriginal, term subregion.

All countries, regions, and subregions are stored in `regions.csv`. In
cases where a region has no subregions, the region is considered its
own subregion and the `region` and `sub-region` columns have the same data.
In the rare cases where countries are not subdivided, `region` and `sub-region`
are taken to be the whole country.

Since all regions and sub-regions have been provided, it is recommended
that users select the subset of locations of interest from this file.

## Inputs

Magpie takes as inputs a CSV file consisting of all the locations
for which target species should be extracted. This file should either
have the structure of `regions.csv` or 

