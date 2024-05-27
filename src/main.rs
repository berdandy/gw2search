mod api;
mod config;
mod request;

use iced::{
    button, scrollable, text_input, pick_list, Alignment, Button, Column, Element, Color,
	PickList, Row, Rule, Sandbox, Scrollable, Settings, Text, TextInput, Checkbox
};

use config::CONFIG;
use std::env;
use std::io;
use std::io::Write;

use crate::api::result_render;

/// api_search!(api::ApiSkill, api::Skill, &CONFIG.skills_file, "skills") -> results
macro_rules! api_search {
	($api_type:ty, $type:ty, $file:expr, $endpoint:expr) => {
		request::get_data($file, || async {
			let api_results: Vec<$api_type> = request::request_paginated($endpoint, &CONFIG.lang).await?;
			Ok(api_results
				.into_iter()
				.map(|api_result| <$type>::from(api_result))
				.collect())
		})
		.await?
	}
}

/// api_filter!(results_to_filter, search_term, in_reverse);
macro_rules! api_filter {
	($results:expr, $search:expr, $reverse:expr) => {
		$results
			.iter()
			.filter_map(|r| match $reverse {
				false => match &r.name.to_ascii_lowercase().contains(&$search.to_ascii_lowercase()) {
					true => Some(r),
					false => None
				},
				true => match &r.id.to_string() == &$search.to_ascii_lowercase() {
					true => Some(r),
					false => None
				}
			})
			.map(|r| result_render(r))
			.collect::<Vec<String>>()
	};
	($results:expr, $search:expr, $reverse:expr, $annotation:expr) => {
		$results
			.iter()
			.filter_map(|r| match $reverse {
				false => match &r.name.to_ascii_lowercase().contains(&$search.to_ascii_lowercase()) {
					true => Some(r),
					false => None
				},
				true => match &r.id.to_string() == &$search.to_ascii_lowercase() {
					true => Some(r),
					false => None
				}
			})
			.map(|r| result_render(r) + $annotation)
			.collect::<Vec<String>>()
	}
}

/// api_searcher!(api::ApiSkill, api::Skill, &CONFIG.skills_file, "skills", search_term, in_reverse);
macro_rules! api_searcher {
	($api_type:ty, $type:ty, $file:expr, $endpoint:expr, $search:expr, $reverse:expr) => {
		let results = api_search!($api_type, $type, $file, $endpoint);

		if $search.is_empty() && !CONFIG.csv && !CONFIG.json {
			return Ok(vec![]);
		}

		return Ok(api_filter!(results, $search, $reverse));
	}
}

pub fn main() -> iced::Result {
    let argc = env::args().count();
    if argc > 1 {
        let mode : SearchMode = match &CONFIG {
            cfg if cfg.any => SearchMode::Any,
            cfg if cfg.skill => SearchMode::Skill,
            cfg if cfg.item => SearchMode::Item,
            cfg if cfg.r#trait => SearchMode::Trait,
            cfg if cfg.spec => SearchMode::Spec,
            cfg if cfg.elite_spec => SearchMode::EliteSpec,
            cfg if cfg.profession => SearchMode::Profession,
            cfg if cfg.pet => SearchMode::Pet,
            cfg if cfg.legend => SearchMode::Legend,
            _ => SearchMode::Skip,
        };

        if mode != SearchMode::Skip {
            let term = match &CONFIG.search_term {
                Some(term) => term.clone(),
                _ => String::from(""),
            };
            let results = match search_api(mode, term, CONFIG.reverse) {
                Ok(results) => results,
                Err(e) => panic!("error searching with commandline search: {}", e),
            };
			let sep = match CONFIG.quiet || CONFIG.json {
				true => ",",
				false => "\n",
			};
	
			if CONFIG.quiet {
				// no trailing newline
				print!("{}", results.join(sep));
				io::stdout().flush().unwrap();
			} else if CONFIG.csv {
				println!("id,name\n{}", results.join(sep));
			} else if CONFIG.json {
				println!("[{}]", results.join(sep));
			} else {
				println!("{}", results.join(sep));
			}
        }
        Ok(())
    } else {
        Gw2Search::run(Settings::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum SearchMode {
    Any,
	#[default]
	Item,
	Skill,
	Trait,
	Spec,
	EliteSpec,
	Profession,
	Pet,
	Legend,
    Skip,
}

// for drop down
impl SearchMode {
    const ALL: [SearchMode; 9] = [
		SearchMode::Any,
		SearchMode::Item,
		SearchMode::Skill,
		SearchMode::Trait,
		SearchMode::Spec,
		SearchMode::EliteSpec,
		SearchMode::Profession,
		SearchMode::Pet,
		SearchMode::Legend,
    ];
}
impl std::fmt::Display for SearchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SearchMode::Any => "Any",
                SearchMode::Item => "Item",
                SearchMode::Skill => "Skill",
                SearchMode::Trait => "Trait",
                SearchMode::Spec => "Spec",
                SearchMode::EliteSpec => "Elite Spec",
                SearchMode::Profession => "Profession",
                SearchMode::Pet => "Pet",
                SearchMode::Legend => "Legend",
                SearchMode::Skip => "---",
            }
        )
    }
}

#[derive(Default)]
struct Gw2Search {
    scroll: scrollable::State,
    input: text_input::State,
	pick_list: pick_list::State<SearchMode>,
	search_mode: SearchMode,
    search_term: String,
    reverse: bool,
	results: Vec<String>,
	search_button: button::State,
	delete_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
	Search,
	SearchTermChanged(String),
	SearchModeSelected(SearchMode),
	ReverseSearchChanged(bool),
	DeleteData,
}

impl Sandbox for Gw2Search {
    type Message = Message;

	fn new() -> Self {
		Self::default()
	}

	fn title(&self) -> String {
		String::from("Gw2Search")
	}

	fn update(&mut self, message: Message)
	{
		match message {
			Message::SearchTermChanged(term) => {
				self.search_term = term;
			}
			Message::SearchModeSelected(search_mode) => {
				self.search_mode = search_mode;

				if ! self.search_term.is_empty() {
					match search_api(self.search_mode, self.search_term.clone(), self.reverse) {
						Ok(results) => {
							self.results = results;
						}
						Err(error) => panic!("Problem with search {:?}", error)
					}
				}
			}
			Message::ReverseSearchChanged(reverse_search) => {
				self.reverse = reverse_search;
			}
			Message::Search => {
				match search_api(self.search_mode, self.search_term.clone(), self.reverse) {
					Ok(results) => {
						self.results = results;
					}
					Err(error) => panic!("Problem with search {:?}", error)
				}
			}
			Message::DeleteData => {
				for file in [&CONFIG.items_file, &CONFIG.skills_file, &CONFIG.traits_file, &CONFIG.specs_file, &CONFIG.professions_file, &CONFIG.pets_file, &CONFIG.legends_file] {
					if let Err(e) = config::remove_data_file(file) {
						eprintln!("Failed to remove file {}: {}", file.display(), e);
					}
				}
			}
		}
	}

	fn view(&mut self) -> Element<Self::Message> {

		let results = self.results.iter().fold(
            Column::new().spacing(10).push(Text::new("")),
            |column: Column<Message>, result| {
                column.push(
					Text::new(result)
                )
            },
        );

		Column::new()
            .spacing(8)
			.padding(8)
			.align_items(Alignment::Center)
            .push(Text::new("gw2search").size(32))
            .push(Rule::horizontal(30))
            .push(
				Row::new()
				.push(TextInput::new(
						&mut self.input,
						"Search Term",
						&self.search_term,
						Message::SearchTermChanged
					).on_submit(Message::Search)
				)
				.push(Button::new(&mut self.search_button, Text::new("Search"))
					.on_press(Message::Search)
				)
				.push(
					PickList::new(
						&mut self.pick_list,
						&SearchMode::ALL[..],
						Some(self.search_mode),
						Message::SearchModeSelected)
				)
				.push(
					Checkbox::new(
						self.reverse,
						"Reverse",
						Message::ReverseSearchChanged)
				)
			)
			.push(Button::new(&mut self.delete_button, Text::new("Delete Data Files"))
				.on_press(Message::DeleteData)
			)
			.push(Text::new("Input search term."))
			.push(Text::new("Delete Data Files to clear permanently cached results")
				.size(12)
				.color(Color::from_rgb8(0xDD, 0x22, 0x22))
			)
			.push(Text::new("made by berdandy.1968").size(12))
			.push(
				Scrollable::new(&mut self.scroll)
					.padding(50)
					.push(results)
			)
			.into()
		}
}

#[tokio::main]
async fn search_api(search_mode: SearchMode, search_term: String, in_reverse: bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
	match search_mode {
        SearchMode::Skip => {
            Ok(vec![])
        },
		SearchMode::Skill => {
			api_searcher!(api::ApiSkill, api::Skill, &CONFIG.skills_file, "skills", search_term, in_reverse);
		}
		SearchMode::Trait => {
			api_searcher!(api::ApiTrait, api::Trait, &CONFIG.traits_file, "traits", search_term, in_reverse);
		}
		SearchMode::Item => {
			api_searcher!(api::ApiItem, api::Item, &CONFIG.items_file, "items", search_term, in_reverse);
		}
		SearchMode::Spec => {
			api_searcher!(api::ApiSpec, api::Spec, &CONFIG.specs_file, "specializations", search_term, in_reverse);
		}
		SearchMode::Profession => {
			api_searcher!(api::ApiProfession, api::Profession, &CONFIG.professions_file, "professions", search_term, in_reverse);
		}
		SearchMode::Pet => {
			api_searcher!(api::ApiPet, api::Pet, &CONFIG.pets_file, "pets", search_term, in_reverse);
		}
		SearchMode::EliteSpec => {
			let elite_specs: Vec<api::Spec> = api_search!(api::ApiSpec, api::Spec, &CONFIG.specs_file, "specializations")
				.iter()
				.filter(|s| s.elite)
				.cloned()
				.collect();

			if search_term.is_empty() && !CONFIG.csv && !CONFIG.json {
				return Ok(vec![]);
			}

			Ok(api_filter!(elite_specs, search_term, in_reverse))
		}
		SearchMode::Legend => {
			// can't use api_searcher! macro because we need to doctor the results:
			let mut results = api_search!(api::ApiLegend, api::Legend, &CONFIG.legends_file, "legends");

			// these are not in the API, so we make them up
			results.push(api::Legend {
					id: String::from("Legend7"),
					name: String::from("Alliance/Luxon/Archemorus"),

					code: 7,
					swap: 62891,
					heal: 62719,
					utilities: [62832, 62962, 62878],
					elite: 62942,
			});
			results.push(api::Legend {
                id: String::from("Legend8"),
                name: String::from("Alliance/Kurzick/Saint Viktor"),

                code: 8,
                swap: 62891,
                heal: 62679,
                utilities: [62701, 62941, 62796],
                elite: 62686,
			});

			if search_term.is_empty() && !CONFIG.csv && !CONFIG.json {
				return Ok(vec![]);
			}

			return Ok(api_filter!(results, search_term, in_reverse));
		}
		SearchMode::Any => {
			let skills = api_search!(api::ApiSkill, api::Skill, &CONFIG.skills_file, "skills");
			let traits = api_search!(api::ApiTrait, api::Trait, &CONFIG.traits_file, "traits");
			let items = api_search!(api::ApiItem, api::Item, &CONFIG.items_file, "items");
			let specs = api_search!(api::ApiSpec, api::Spec, &CONFIG.specs_file, "specializations");
			let professions = api_search!(api::ApiProfession, api::Profession, &CONFIG.professions_file, "professions");
			let pets = api_search!(api::ApiPet, api::Pet, &CONFIG.pets_file, "pets");
			let legends = api_search!(api::ApiLegend, api::Legend, &CONFIG.legends_file, "legends");

			// ------------------------------------------------------------

			if search_term.is_empty() && !CONFIG.csv && !CONFIG.json {
				return Ok(vec![]);
			}

			let mut results: Vec<String> = vec![];
			results.extend(api_filter!(skills, search_term, in_reverse, " [SKILL]"));
			results.extend(api_filter!(traits, search_term, in_reverse, " [TRAIT]"));
			results.extend(api_filter!(items, search_term, in_reverse, " [ITEM]"));
			results.extend(api_filter!(professions, search_term, in_reverse, " [PROFESSION]"));
			results.extend(api_filter!(specs, search_term, in_reverse, " [SPECIALIZATION]"));
			results.extend(api_filter!(pets, search_term, in_reverse, " [PET]"));
			results.extend(api_filter!(legends, search_term, in_reverse, " [LEGEND]"));

			Ok(results)
		}
	}
}
