<!-- next-header -->

## [Unreleased]

- Added --reset option to control cache refresh. E.g. "--reset --item" or "--reset --any"
- Added weapon skill output for professions

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

## 0.9.1 (2025-07-07)

### New Features

 - <csr-id-93cf59575dd553ac29e0911dcaffdd2f2c5a2bd4/> added weapon skills to profession (mostly for json output)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 84 calendar days.
 - 315 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added weapon skills to profession (mostly for json output) ([`93cf595`](https://github.com/berdandy/gw2search/commit/93cf59575dd553ac29e0911dcaffdd2f2c5a2bd4))
    - Added superquiet feature for tooling ([`bd319f5`](https://github.com/berdandy/gw2search/commit/bd319f55192b95583643ec036d5593696260d0b7))
</details>

## v0.9.0 (2024-08-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 89 calendar days.
 - 90 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added --reset flag to reset only specified data (using search type flag) ([`0dcabc2`](https://github.com/berdandy/gw2search/commit/0dcabc276851117240914acab798d259dc7e08f2))
    - Added Itemstats Attributes to see stats in json export ([`66ede93`](https://github.com/berdandy/gw2search/commit/66ede93d81c399ae4ac46fd8b29d0eaf1367a2f1))
    - Added itemstats endpoint ([`49a5d49`](https://github.com/berdandy/gw2search/commit/49a5d4923720d178195cb0abf47f6422a64cfcb1))
</details>

## v0.8.13 (2024-05-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Debug to stderr ([`2f3f8b3`](https://github.com/berdandy/gw2search/commit/2f3f8b3370b47703d0bab44fce25a70c9a5abc86))
</details>

## v0.8.12 (2024-05-27)

## v0.8.11 (2024-05-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 42 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fixed --help message swapped between elite spec and non-elite spec ([`b0ebd44`](https://github.com/berdandy/gw2search/commit/b0ebd443828d04062fc61a5381175f7a6e32a848))
</details>

## v0.8.10 (2024-04-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added elite spec search to gui ([`180856f`](https://github.com/berdandy/gw2search/commit/180856f83ce9ce34b947917f94b046c5307780aa))
</details>

## v0.8.9 (2024-04-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added -E search option to search elite specs only ([`bd23cc5`](https://github.com/berdandy/gw2search/commit/bd23cc52882826ce49875f1a96a12341593aeb1c))
</details>

## v0.8.8 (2024-04-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fixed yet another skill id typo for alliance legend ([`2b270d6`](https://github.com/berdandy/gw2search/commit/2b270d67ad9cd286559daf6558ed013b0b2b34dd))
    - Fixed another skill id typo for alliance legend ([`83bf9db`](https://github.com/berdandy/gw2search/commit/83bf9db85db796e4d523752058db5b6d17c5f359))
    - Fixed a couple of skill id typoes for alliance legend ([`d0046b6`](https://github.com/berdandy/gw2search/commit/d0046b6474bd460c62a66cc4c3cfa9301aac02d6))
    - More clippy happiness ([`2167f89`](https://github.com/berdandy/gw2search/commit/2167f8955bd4648fccde7a779ba2a503fb0b57e8))
    - Clippy happiness ([`7d2054d`](https://github.com/berdandy/gw2search/commit/7d2054d72a9f4f559df88a3da4873b2e49af752c))
</details>

## v0.8.5 (2024-04-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added fake legends 7 & 8 to accomodate alliance legend (until api changes) ([`e71e3dd`](https://github.com/berdandy/gw2search/commit/e71e3ddd5843ea4afc54d827cb4f6bc96ea2b58d))
    - Added pet icon to model ([`d2d7aa4`](https://github.com/berdandy/gw2search/commit/d2d7aa494c88a6849729a1ececabb15e2e91e078))
    - Less german ([`da902af`](https://github.com/berdandy/gw2search/commit/da902afa4aeafa6c98667a5b43263e1e8ab43790))
</details>

## v0.8.3 (2024-03-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added json exporter and some additional data ([`b22b0ee`](https://github.com/berdandy/gw2search/commit/b22b0eec5be285843c6735d8b5ace71947782b21))
    - Added version string to urls ([`0dfdd9d`](https://github.com/berdandy/gw2search/commit/0dfdd9d82567ba88a5ecb57dce2b5f449e55b2d0))
    - Refactor to implement pretty-print/id/csv output as a derive macro ([`c0a092c`](https://github.com/berdandy/gw2search/commit/c0a092c93092cb2e19b27f7a121b490b76ce6f28))
    - Refactor to make api endpoints not quite as much of a copy-paste garbage fire ([`62017c8`](https://github.com/berdandy/gw2search/commit/62017c88c3dd31e028279af351ede71a068019eb))
</details>

## v0.8.2 (2024-03-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fixed broken legend api endpoint ([`afe416e`](https://github.com/berdandy/gw2search/commit/afe416e4ed290c0d577699506420fac6e3546ae4))
</details>

## v0.8.1 (2024-03-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Converted csv output to have quotes for name field ([`0af7ef1`](https://github.com/berdandy/gw2search/commit/0af7ef1ef856794e33cf91e57493acec1bf29af2))
    - Added spec, profession, pet and legend endpoints ([`19394a6`](https://github.com/berdandy/gw2search/commit/19394a688d5229bfb51d65f9a06a73d47ef13a0e))
    - Partial add for Specializations ([`3ffdaba`](https://github.com/berdandy/gw2search/commit/3ffdaba7cad6571b074ad5af701a94ea8896fd31))
</details>

## v0.7.4 (2024-03-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added csv export mechanism ([`4afb9d2`](https://github.com/berdandy/gw2search/commit/4afb9d2cc9838dfd21ded87c0c17db38326addf6))
</details>

## v0.7.3 (2024-03-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added a filter to avoid searching with empty search term (but still download data files) ([`5940bd0`](https://github.com/berdandy/gw2search/commit/5940bd054ae351397ee83a4494263d57b3e4807d))
</details>

## v0.7.2 (2024-03-20)

<csr-id-2365421f2fb3897ac40bbb973d8200a84dd523c9/>

### Chore

 - <csr-id-2365421f2fb3897ac40bbb973d8200a84dd523c9/> added cargo dist configuration

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 175 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added cargo dist configuration ([`2365421`](https://github.com/berdandy/gw2search/commit/2365421f2fb3897ac40bbb973d8200a84dd523c9))
    - Removed trailing newline from -q searches ([`06c5843`](https://github.com/berdandy/gw2search/commit/06c584373789aa392304de745b3a206f4307417a))
    - Added quiet option to display only ids of result ([`c0d4e09`](https://github.com/berdandy/gw2search/commit/c0d4e090d28d9cea5b14074e17f70dcec929da92))
    - Dead code trim, layout improvements, buttons ([`8cbd0d6`](https://github.com/berdandy/gw2search/commit/8cbd0d6b7d568944d51d9d4e2de7bfde4466fb9e))
</details>

## v0.7.0 (2023-09-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 33 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added new item types: fishing stuff, service chips, relics ([`8cd0dc1`](https://github.com/berdandy/gw2search/commit/8cd0dc11674f994d3f44201a8c96ddc11f3073b2))
</details>

## v0.6.1 (2023-08-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added placeholder relic type ([`54d4815`](https://github.com/berdandy/gw2search/commit/54d4815ad68ab994bdc6dc314ba745c4cbf86e28))
</details>

## v0.6.0 (2023-08-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 35 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added reverse search by id ([`a25802a`](https://github.com/berdandy/gw2search/commit/a25802ae8750b15a29150b6c57c69fd25ba944c9))
</details>

## v0.5.0 (2023-07-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 52 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added 'Any' option to search. Using it will show all matches, whether item or skill or trait. ([`cf7d11e`](https://github.com/berdandy/gw2search/commit/cf7d11e5a8a745dedb94ec737b375fa11d6f6b9e))
    - Added "any" option to search items/skills/traits. This will likely become the new default ([`e050cd6`](https://github.com/berdandy/gw2search/commit/e050cd6c89bc3fbdc122e359d4e44880e2752df0))
</details>

## v0.4.2 (2023-05-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 344 calendar days.
 - 344 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Updated items api to work with JadeTechModule/PowerCore instead of Quux/Qux temporary api names ([`68e9fe4`](https://github.com/berdandy/gw2search/commit/68e9fe4c11e6b226e0fa8946187f39d63d7eea0d))
    - Minor knowledge cleanup ([`3aa8ad4`](https://github.com/berdandy/gw2search/commit/3aa8ad4244f5884c800e43b0c77bfe60b6b5b7c2))
</details>

## v0.4.1 (2022-06-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 45 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Upgraded dependency versions (and adjusted for breaking changes) ([`5682eb4`](https://github.com/berdandy/gw2search/commit/5682eb462c150c2c8ea1e77ee75680e0ab4f3256))
    - Merge branch 'gui' mixed mode is ready ([`4c5f114`](https://github.com/berdandy/gw2search/commit/4c5f11474853d6d24e8f71fba5df42778c946ce8))
    - Better skipping of search when a non-search option is entered (like --reset-data) ([`56030f7`](https://github.com/berdandy/gw2search/commit/56030f7ce9fd0fc64780548f9b948493291f15da))
    - Added mixed mode gui/cmdline. Call with 0 arguments for gui, provide them for command line expert mode ([`826beb9`](https://github.com/berdandy/gw2search/commit/826beb90cbc43de5db801a2a441f8dbe1a5afdad))
</details>

## v0.3.0 (2022-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 9 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added some instructions ([`6382c1e`](https://github.com/berdandy/gw2search/commit/6382c1efef5916208aff249ba8b2572e44776bec))
    - Working gui implementation. Some warts: initial download is blocking and feedback is 100% on console, so invisible to casual users ([`157c9b3`](https://github.com/berdandy/gw2search/commit/157c9b3320e53ea6cfa6fabac9bb61a979d8c0d9))
    - Added some gui experiments ([`c3e140f`](https://github.com/berdandy/gw2search/commit/c3e140f63b0c295178768c29a8ade1eb839f8787))
</details>

## v0.2.0 (2022-04-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added skill and trait support ([`d123f9d`](https://github.com/berdandy/gw2search/commit/d123f9d097bc6d375d0030d9235cc0b75da46761))
    - Cleanup and hacking to bits ([`d98c32b`](https://github.com/berdandy/gw2search/commit/d98c32b3bb3d1e9d83b0e68a36489c775e516f86))
    - Preliminary proof-of-concept. Needs SIGNIFICANT cleanup ([`3ba8ff4`](https://github.com/berdandy/gw2search/commit/3ba8ff411446d0afde4a3c78e12804ade5009b7e))
</details>

