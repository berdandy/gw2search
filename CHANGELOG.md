<!-- next-header -->

## [Unreleased]

- Added --reset option to control cache refresh. E.g. "--reset --item" or "--reset --any"

## [0.8.14-rc.1]

- Added Itemstats Attributes to see stats in json export

## [0.8.13]

- Added itemstats endpoint

## [0.8.12]

- Minor CSV format correction for double quotes in name field
- Changed debug info to go to stderr

## [0.8.11]

- Elite spec search type and non-elite spec search help text were swapped

## [0.8.10]

- Added elite spec search type to only read from spec elite=true results (-E option)
- Added elite spec search type to gui drop down
- Fixed skill id typos for alliance legend
- Clippy happiness

## [0.8.5]

- Added pet icon url to model
- Added legends skills to model
- Added hardcoded fake legends 7 & 8 to accomodate alliance legend (until api changes)

## [0.8.3]

- Added json exporter
- New json data:
	specialization:major_traits
	profession:skills_by_palette
	skill:description

- Refactor to implement pretty-print/id/csv output as a derive macro
- Refactor to make api endpoints not quite as much of a copy-paste garbage fire

## [0.8.2]

- Legend endpoints addressed slightly. There is a hardcoded id-name mapping, because that doesn't exist in the API
- Placeholder Legend7 (Alliance) is also there, should the api actually return it someday

## [0.8.1]

- Added new endpoints: professions, specializations, pets, legends (just id/name data available)
- Changed csv output to have quoted name (supports internal comma)

## [0.7.4]

- using a -c option will format output as an 'id,name' csv file. It can be combined with -a/-s/-t/-i options and search terms

## [0.7.3]

- If a blank search term is entered, a search is not performed. Data file download still occurs, however, making this useful for a pre-cache with `gw2search -a`

## [0.7.2]

- Added -q option to display only the id number of results
- UI improvements. Like actual buttons

## [0.7.0]

The Anet API now has an actual relic type, and not the placeholder Mwcc

Some other item types are also new in the API: fishing stuff, service chips, sensory arrays. They've been added as well.

## [0.6.1]

Added the placeholder relic type to items api

## [0.6.0]

This version adds a reverse search option.

This can be particularly useful when the API is down or broken.

## [0.5.0]

This version adds the (currently non-default) Any option. Selecting that option from the dropdown, or using it on the command line with -a will have gw2search return results from skills, traits and items. They will be annotated on display.

This option will become the new default once I implement a gui progress meter, as api download feedback is still only available in the commandline mode.

## [0.4.2]

Minor update to re-align API data model to use official jade bot module and power core category names, so items can once again be searched.

## [0.4.1]

This release brings GUI edition and command-line edition back into the same binary.

Simply run the executable with no arguments (or double-click) to get GUI mode.

If you provide arguments (even -h), it'll use the "expert" command-line interface.

Note that --reset-data is only available on command line, but it affects both modes.

For other platforms besides windows: Use the source, Luke.

## [0.3.0-gui]

This version adds a gui interface. Currently there is no support for both modes of operation. Use v0.2.0 if you prefer commandline (actual search functionality is unchanged).

I expect to eventually have simultaneous support for both modes of operation.

## [0.2.0]

Initial release of the gw2search command line tool (win64)

This searches the api for items, skills or traits by name, and returns the matching ids.

Note: API data is cached. Refresh it with --reset-data

<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v0.9.0...HEAD
