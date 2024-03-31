# gw2search

A tool to search things in the gw2 api by name and shows them in various formats (id-only, id-name, csv, json)

## Supports:

- Items
- Skills
- Traits
- Professions
- Specializations
- Pets
- Legends (partial)

## GUI Mode

Run with no arguments (or double click) to open a gui tool. There is no installer.

Note that some errors are not visible in gui mode, and command-line mode must be used.

![Sample Screenshot of Item Search](img/sample_item.png?raw=true "Sample Item Search")

![Sample Screenshot of Skill Search](img/sample_skill.png?raw=true "Sample Skill Search")

![Sample Screenshot of Trait Search](img/sample_trait.png?raw=true "Sample Trait Search")

## Command-line Expert Mode

Run with argument usage shown below for command-line "expert" mode using cmd.exe (or linux/mac shells)

### Usage:

```
gw2search 0.8.3

USAGE:
    gw2search [FLAGS] [OPTIONS] [search-term]

FLAGS:
    -a, --any           Search for skill, trait or item
    -c, --csv           Print results as csv-formatted data
    -h, --help          Prints help information
    -i, --item          Search for item (default)
    -j, --json          Print results as json-formatted data
    -l, --legend        Search for legends
    -P, --pet           Search for pets
    -p, --profession    Search for professions
    -q, --quiet         Only print id number results upon match
        --reset-data    Download content from the GW2 API, replacing any previously cached kontent
    -r, --reverse       Search for id instead of name
    -s, --skill         Search for skill
    -S, --spec          Search for specialization
    -t, --trait         Search for trait
    -V, --version       Prints version information

OPTIONS:
        --cache-dir <cache-dir>        Save cached API calls to this directory

                                       If provided, the parent directory of the cache directory must already exist.
                                       Defaults to '/home/andy/.cache/gw2search'.
        --config-file <config-file>    Read config options from this file. Supported options:

                                           lang = "<lang>"

                                       The default file location is '/home/andy/.config/gw2search/gw2search.toml'.
        --data-dir <data-dir>          Save cached items and other content to this directory

                                       If provided, the parent directory of the cache directory must already exist.
                                       Defaults to '/home/andy/.local/share/gw2search'.
        --lang <lang>                  One of "en", "es", "de", or "fr". Defaults to "en"

ARGS:
    <search-term>    Search for this
```

## Output:

```
$ gw2search -s shroud
10574 : Death Shroud
10585 : End Death Shroud
30792 : Reaper's Shroud
30961 : Exit Reaper's Shroud
44663 : Desert Shroud
54870 : Sandstorm Shroud
62540 : Exit Harbinger Shroud
62567 : Harbinger Shroud
63155 : Enter Shadow Shroud
63251 : Exit Shadow Shroud
```

```
$ gw2search -cs thousand
id,name
13026,"Prepare Thousand Needles"
24755,"Thousand Cuts"
56898,"Thousand Needles"
```

```
$ gw2search -js thousand
[{"description":"Preparation. Mark your current area with inhibiting magic, readying the location to poison enemies.","id":13026,"name":"Prepare Thousand Needles"},{"description":"Psionic. Open a portal that devastates targets in a line in front of you with a flurry of blades.","id":24755,"name":"Thousand Cuts"},{"description":"Unleash a hail of needles that immobilizes enemies on impact and poisons the ground, inflicting conditions over time.","id":56898,"name":"Thousand Needles"}]
---

This project contains MIT-licensed code from gw2-arbitrage. The MIT license file was not provided, but [it may be found here](https://github.com/t-mw/gw2-arbitrage).
