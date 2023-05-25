# gw2search

A tool to search things in the gw2 api by name and shows their IDs

## Supports:

- Items
- Skills
- Traits

## GUI Mode

Run with no arguments (or double click) to open a gui tool. There is no installer.

Note that some errors are not visible in gui mode, and command-line mode must be used.

- ![Sample Screenshot of Item Search](img/sample_item.png?raw=true "Sample Item Search")
- ![Sample Screenshot of Skill Search](img/sample_skill.png?raw=true "Sample Skill Search")
- ![Sample Screenshot of Trait Search](img/sample_trait.png?raw=true "Sample Trait Search")

## Command-line Expert Mode

Run with argument usage shown below for command-line "expert" mode using cmd.exe (or linux/mac shells)

### Usage:

```
gw2search 0.4.2

USAGE:
    gw2search [FLAGS] [OPTIONS] [search-term]

FLAGS:
    -h, --help          Prints help information
    -i, --item          Search for item (default)
        --reset-data    Download content from the GW2 API, replacing any previously cached content
    -s, --skill         Search for skill
    -t, --trait         Search for trait
    -V, --version       Prints version information

OPTIONS:
        --cache-dir <cache-dir>        Save cached API calls to this directory

                                       If provided, the parent directory of the cache directory must already exist.
                                       Defaults to '[REDACTED]'.
        --config-file <config-file>    Read config options from this file. Supported options:

                                           lang = "<lang>"

                                       The default file location is '[REDACTED]'.
        --data-dir <data-dir>          Save cached items and other content to this directory

                                       If provided, the parent directory of the cache directory must already exist.
                                       Defaults to '[REDACTED]'.
        --lang <lang>                  One of "en", "es", "de", or "fr". Defaults to "en"

ARGS:
    <search-term>    Search for this
```

## Output:

```
$ gw2search -s shroud
Results found: 10
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

---

This project contains MIT-licensed code from gw2-arbitrage. The MIT license file was not provided, but [it may be found here](https://github.com/t-mw/gw2-arbitrage).
