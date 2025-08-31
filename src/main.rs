mod api;
mod config;
mod request;

use config::CONFIG;
use std::env;
use std::io;
use std::io::Write;

use iced::{
    task,
	Element, Alignment, Background, Border, Theme, Task,
    widget::{
        Column, Text, TextInput,
        scrollable, text_input, pick_list, button, column, row, text, horizontal_rule, checkbox,
    }
};

use crate::api::result_render;

// synchronous
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

/*
/// api_search_async!(api::ApiSkill, api::Skill, &CONFIG.skills_file, "skills") -> results
macro_rules! api_search {
	($api_type:ty, $type:ty, $file:expr, $endpoint:expr) => {
		task::spawn(async move {
			request::get_data($file, || async {
				let api_results: Vec<$api_type> = request::request_paginated($endpoint, &CONFIG.lang).await?;
				Ok(api_results
					.into_iter()
					.map(|api_result| <$type>::from(api_result))
					.collect())
			})
			.await.unwrap()
		}).await?
	}
}
*/

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
	Itemstat,
    Skip,
}

// for drop down
impl SearchMode {
    const ALL: [SearchMode; 10] = [
		SearchMode::Any,
		SearchMode::Item,
		SearchMode::Skill,
		SearchMode::Trait,
		SearchMode::Spec,
		SearchMode::EliteSpec,
		SearchMode::Profession,
		SearchMode::Pet,
		SearchMode::Legend,
		SearchMode::Itemstat,
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
                SearchMode::Itemstat => "Itemstat",
                SearchMode::Legend => "Legend",
                SearchMode::Skip => "---",
            }
        )
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
            cfg if cfg.itemstat => SearchMode::Itemstat,
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
		iced::run("GW2 Search", Gw2Search::update, Gw2Search::view)
    }
}

// state
struct Gw2Search {
    search_term: String,
	search_mode: Option<SearchMode>,
	reverse: bool,
	results: Vec<String>,

    search_state: SearchState,
}

enum SearchState {
    Idle,
    Searching { _abort: task::Handle }
}

impl SearchState {
    fn is_idle(&self) -> bool {
        match *self {
            SearchState::Idle => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
	Search,
    SearchResults(Vec<String>),
	SearchTermChanged(String),
	SearchModeSelected(SearchMode),
	ReverseSearchChanged(bool),
	DeleteData,
}

impl Default for Gw2Search {
	fn default() -> Self {
		Gw2Search {
			search_term: String::default(),
			results: vec!(),
			reverse: false,
			search_mode: Some(SearchMode::Item),
            search_state: SearchState::Idle,
		}
	}
}

impl Gw2Search {
/*
    // SIMPLE SYNCHRONOUS
	fn search_to_results(&mut self) {
		if let Some(mode) = self.search_mode {
			match search_api(mode, self.search_term.clone(), self.reverse) {
				Ok(results) => {
					self.results = results;
				}
				Err(error) => panic!("Problem with search {:?}", error)
			}
		}
	}
*/

	pub fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::Search => {
                if self.search_state.is_idle() {
                    let search_mode = self.search_mode.clone();
                    let search_term = self.search_term.clone();
                    let search_reverse = self.reverse.clone();
                    if let Some(mode) = search_mode {
                        let (task, handle) = Task::perform(
                            async move { 
                                match search_api(mode, search_term, search_reverse) {
                                    Ok(results) => results,
                                    Err(error) => panic!("Problem with search {:?}", error)
                                }
                            },
                            |results| Message::SearchResults(results),
                        ).abortable();
                        self.search_state = SearchState::Searching { _abort: handle };
                        return task;
                    }
                }
                Task::none()
			}
            Message::SearchResults(results) => {
                self.search_state = SearchState::Idle;
                self.results = results;
                Task::none()
            }
			Message::SearchTermChanged(term) => {
				self.search_term = term;
                Task::none()
			}
			Message::SearchModeSelected(search_mode) => {
				self.search_mode = Some(search_mode);
                Task::none()
			}
			Message::ReverseSearchChanged(reverse_search) => {
				self.reverse = reverse_search;
                Task::none()
			}
			Message::DeleteData => {
				for file in [&CONFIG.items_file, &CONFIG.skills_file, &CONFIG.traits_file, &CONFIG.specs_file, &CONFIG.professions_file, &CONFIG.pets_file, &CONFIG.legends_file] {
					if let Err(e) = config::remove_data_file(file) {
						eprintln!("Failed to remove file {}: {}", file.display(), e);
					}
				}
                Task::none()
			}
		}
	}

	pub fn view(&self) -> Element<Message> {
		let results = self.results.iter().fold(
			Column::new().push(Text::new("")),
			|column: Column<Message>, result| {
				column.push(
					// Text::new(result) // looks good
					TextInput::new("Result", result).style(|theme: &Theme, _| {
                        let palette = theme.extended_palette();
                        crate::text_input::Style 
                        {
                            background: Background::Color(palette.background.base.color),
                            border: Border {
                                radius: 2.0.into(),
                                width: 1.0,
                                color: palette.background.strong.color,
                            },
                            icon: palette.background.weak.text,
                            placeholder: palette.secondary.base.color,
                            value: palette.background.base.text,
                            selection: palette.primary.weak.color,
                        }
                    })
				)
			}
		);
		scrollable(
			column![
				text("gw2search").size(32),
				horizontal_rule(30),
				row![
					text_input("Search Term", &self.search_term)
						.size(40)
						.on_input(|s| Message::SearchTermChanged(s))
                        .on_submit_maybe(
                            if self.search_state.is_idle() {
                                Some(Message::Search)
                            } else {
                                None
                            }
                        ),
					column![
						row![
							button("SEARCH").on_press_maybe(
                                if self.search_state.is_idle() {
                                    Some(Message::Search)
                                } else {
                                    None
                                }
                            ),
							pick_list(
								&SearchMode::ALL[..],
								self.search_mode,
								Message::SearchModeSelected,
							),
							checkbox("Reverse", self.reverse).on_toggle(Message::ReverseSearchChanged),
						],
						button("Delete API Cache")
							.style(button::danger)
							.on_press_maybe(
                                if self.search_state.is_idle() {
                                    Some(Message::DeleteData)
                                } else {
                                    None
                                }
                            ),
					]
				],
				scrollable(results)
			]
			.spacing(8)
			.padding(12)
			.align_x(Alignment::Center)
		).into()
	}
}

#[tokio::main]
async fn search_api(search_mode: SearchMode, search_term: String, in_reverse: bool) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
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
		SearchMode::Itemstat => {
			api_searcher!(api::ApiItemstat, api::Itemstat, &CONFIG.itemstats_file, "itemstats", search_term, in_reverse);
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
			let itemstats = api_search!(api::ApiItemstat, api::Itemstat, &CONFIG.itemstats_file, "itemstats");
			let legends = api_search!(api::ApiLegend, api::Legend, &CONFIG.legends_file, "legends");

			// ------------------------------------------------------------

			if search_term.is_empty() && !CONFIG.csv && !CONFIG.json {
				return Ok(vec![]);
			}

			let mut results: Vec<String> = vec![];
            if CONFIG.superquiet
            {
                results.extend(api_filter!(skills, search_term, in_reverse));
                results.extend(api_filter!(traits, search_term, in_reverse));
                results.extend(api_filter!(items, search_term, in_reverse));
                results.extend(api_filter!(professions, search_term, in_reverse));
                results.extend(api_filter!(specs, search_term, in_reverse));
                results.extend(api_filter!(pets, search_term, in_reverse));
                results.extend(api_filter!(itemstats, search_term, in_reverse));
                results.extend(api_filter!(legends, search_term, in_reverse));
            }
            else
            {
                results.extend(api_filter!(skills, search_term, in_reverse, " [SKILL]"));
                results.extend(api_filter!(traits, search_term, in_reverse, " [TRAIT]"));
                results.extend(api_filter!(items, search_term, in_reverse, " [ITEM]"));
                results.extend(api_filter!(professions, search_term, in_reverse, " [PROFESSION]"));
                results.extend(api_filter!(specs, search_term, in_reverse, " [SPECIALIZATION]"));
                results.extend(api_filter!(pets, search_term, in_reverse, " [PET]"));
                results.extend(api_filter!(itemstats, search_term, in_reverse, " [ITEMSTAT]"));
                results.extend(api_filter!(legends, search_term, in_reverse, " [LEGEND]"));
            }

			Ok(results)
		}
	}
}
