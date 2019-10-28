# budget

This script is a work in progress. The motivation for it is that I was trying to track finances through Google Sheets, and then through some JSON processed by python.

A lot of this led to imparting unnecessary structure, so I rethought the problem and decided that all I really needed was *spending categories*, *names* and *values*. Anything else is either pleasant, or just a distraction.

So...this tool manages finances with a directory structure similar to below:

    .
    ├── archive
    │  ├── carpets.txt
    │  ├── coffee-table.txt
    │  ├── dishwasher.txt
    │  ├── ebooks-2016.txt
    │  ├── ebooks-2017.txt
    │  ├── ebooks-2018.txt
    │  ├── ebooks-2019.txt
    │  ├── ...
    │  ├── ...
    │  └── ...
    ├── monthly
    │  ├── broadband.txt
    │  ├── council-tax.txt
    │  ├── energy.txt
    │  ├── food.txt
    │  ├── ...
    │  ├── ...
    │  └── ...
    ├── oneoff
    │  └── ring2-doorbell.txt
    └── yearly
        ├── dropbox.txt
        ├── insurance.txt
        ├── pinboard.txt
        └── ...
        └── ...

Here, *monthly*, *yearly*, and *oneoff* are categories. There is nothing special about them--the tool will simply aggregate costs within each directory. *archive* is special, in that a directory with the name *archive* is by-default hidden, with a command line flag to show it.

Each finance entry has only 2 required fields: `name` and `cost`. For example, `yearly/dropbox.txt`:

    name: dropbox
    cost: 96
    frequency: yearly

Here, only name and cost are read. *frequency* is just a personal key, such that if the txt file is read without the script, you can put in more helpful information. A more representative example may be a note of `archive/nintendo-switch-games-2018`, where you have an absolute total, but within it you maintain a list of games bought (for the sake of looking back).

The name and cost can be displayed alongside the aggregate output by providing the `-v` flag.

By default, it looks for an env var `$BUDGETDIR`. If this is not defined, you must pass a directory as the first argument. If both exist, the passed directory is preferred.

Example output of `budget -v`

    monthly  ~ £449
        150 -- food
        80 -- energy
        80 -- council tax
        30 -- water
        30 -- factor
        27 -- broadband
        20 -- boiler cover
        9 -- spotify
        9 -- netflix
        9 -- amazon prime
        5 -- twitch critical role
    oneoff   ~ £180
        180 -- ring2 doorbell
    yearly   ~ £417
        157 -- TV license
        155 -- insurance
        96 -- dropbox
        9 -- pinboard

Example output of `budget -a`

    monthly  ~ £449
    oneoff   ~ £180
    yearly   ~ £417
    archive  ~ £9999
