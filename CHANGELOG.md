# Version 0.7.2 - quiet ui

- Added -q option to display only the id number of results
- UI improvements. Like actual buttons

# Version 0.7.0 - Added official relic type Latest

The Anet API now has an actual relic type, and not the placeholder Mwcc

Some other item types are also new in the API: fishing stuff, service chips, sensory arrays. They've been added as well.

# Version 0.6.1 - Added relic type

Added the placeholder relic type to items api

# Version 0.6.0 - Reverse search by id

This version adds a reverse search option.

This can be particularly useful when the API is down or broken.

# Version 0.5.0 - Any option

This version adds the (currently non-default) Any option. Selecting that option from the dropdown, or using it on the command line with -a will have gw2search return results from skills, traits and items. They will be annotated on display.

This option will become the new default once I implement a gui progress meter, as api download feedback is still only available in the commandline mode.

# Version 0.4.2 - api fix

Minor update to re-align API data model to use official jade bot module and power core category names, so items can once again be searched.

# Version 0.4.1 - mixed mode

This release brings GUI edition and command-line edition back into the same binary.

Simply run the executable with no arguments (or double-click) to get GUI mode.

If you provide arguments (even -h), it'll use the "expert" command-line interface.

Note that --reset-data is only available on command line, but it affects both modes.

For other platforms besides windows: Use the source, Luke.

# Version 0.3.0 GUI Edition

This version adds a gui interface. Currently there is no support for both modes of operation. Use v0.2.0 if you prefer commandline (actual search functionality is unchanged).

I expect to eventually have simultaneous support for both modes of operation.

# Version 0.2.0

Initial release of the gw2search command line tool (win64)

This searches the api for items, skills or traits by name, and returns the matching ids.

Note: API data is cached. Refresh it with --reset-data
