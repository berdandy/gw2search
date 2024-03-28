mod api;
mod config;
mod request;
#[cfg(test)]
mod tests;

use iced::{
    button, scrollable, text_input, pick_list, Alignment, Button, Column, Element, Color,
	PickList, Row, Rule, Sandbox, Scrollable, Settings, Text, TextInput, Checkbox
};

use config::CONFIG;
use std::env;
use std::io;
use std::io::Write;

use crate::api::result_render;

macro_rules! debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                dbg!($($e),+)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($e),+)
            }
        }
    };
}

pub fn main() -> iced::Result {
    let argc = env::args().count();
    if argc > 1 {
        let mode : SearchMode = match &CONFIG {
            cfg if cfg.any => SearchMode::Any,
            cfg if cfg.skill => SearchMode::Skill,
            cfg if cfg.item => SearchMode::Item,
            cfg if cfg.r#trait => SearchMode::Trait,
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
			let sep = match CONFIG.quiet {
				false => "\n",
				true => ",",
			};
	
			if CONFIG.quiet {
				// no trailing newline
				print!("{}", results.join(sep));
				io::stdout().flush().unwrap();
			} else if CONFIG.csv {
				println!("id,name\n{}", results.join(sep));
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
pub enum SearchMode {
    Any,
	Item,
	Skill,
	Trait,
    Skip,
}
impl Default for SearchMode {
    fn default() -> Self { SearchMode::Item }
}
// for drop down
impl SearchMode {
    const ALL: [SearchMode; 4] = [
		SearchMode::Any,
		SearchMode::Item,
		SearchMode::Skill,
		SearchMode::Trait,
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
				for file in [&CONFIG.items_file, &CONFIG.skills_file, &CONFIG.traits_file] {
					match config::remove_data_file(file) {
						Err(e) => println!(
							"Failed to remove file {}: {}",
							file.display(),
							e
						),
						_ => (),
					};
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
			debug!("Loading skills");
			let skills: Vec<api::Skill> = request::get_data(&CONFIG.skills_file, || async {
				let api_skills: Vec<api::ApiSkill> =
					request::request_paginated("skills", &CONFIG.lang).await?;
				Ok(api_skills
					.into_iter()
					.map(|api_skill| api::Skill::from(api_skill))
					.collect())
			})
			.await?;
			debug!(
				"Loaded {} skills stored at '{}'",
				skills.len(),
				CONFIG.skills_file.display()
			);

			if search_term.is_empty() && ! CONFIG.csv {
				return Ok(vec![]);
			}

			let results: Vec<_> = skills
				.iter()
				.filter_map(|skill| match in_reverse {
					false => match &skill.name.to_ascii_lowercase().contains(&search_term.to_ascii_lowercase()) {
						true => Some(skill),
						false => None
					},
					true => match &skill.id.to_string() == &search_term.to_ascii_lowercase() {
						true => Some(skill),
						false => None
					}
				})
				.map(|result| result_render(result))
				.collect::<Vec<String>>();

			return Ok(results);

		}
		SearchMode::Trait => {
			debug!("Loading traits");

			let traits: Vec<api::Trait> = request::get_data(&CONFIG.traits_file, || async {
				let api_traits: Vec<api::ApiTrait> =
					request::request_paginated("traits", &CONFIG.lang).await?;
				Ok(api_traits
					.into_iter()
					.map(|api_trait| api::Trait::from(api_trait))
					.collect())
			})
			.await?;
			debug!(
				"Loaded {} traits stored at '{}'",
				traits.len(),
				CONFIG.traits_file.display()
			);

			if search_term.is_empty() && ! CONFIG.csv {
				return Ok(vec![]);
			}

			let results: Vec<_> = traits
				.iter()
				.filter_map(|r#trait| match in_reverse {
						false => match &r#trait.name.to_ascii_lowercase().contains(&search_term.to_ascii_lowercase()) {
							true => Some(r#trait),
							false => None
						},
						true => match &r#trait.id.to_string() == &search_term.to_ascii_lowercase() {
							true => Some(r#trait),
							false => None
						}
				})
				.map(|result| result_render(result))
				.collect::<Vec<String>>();

			return Ok(results);

		}
		SearchMode::Item => {
			debug!("Loading items");
			let items: Vec<api::Item> = request::get_data(&CONFIG.items_file, || async {
				let api_items: Vec<api::ApiItem> =
					request::request_paginated("items", &CONFIG.lang).await?;
				Ok(api_items
					.into_iter()
					.map(|api_item| api::Item::from(api_item))
					.collect())
			})
			.await?;
			debug!(
				"Loaded {} items stored at '{}'",
				items.len(),
				CONFIG.items_file.display()
			);

			if search_term.is_empty() && ! CONFIG.csv {
				return Ok(vec![]);
			}

			let results: Vec<_> = items
				.iter()
				.filter_map(|item| match in_reverse {
						false => match &item.name.to_ascii_lowercase().contains(&search_term.to_ascii_lowercase()) {
							true => Some(item),
							false => None
						},
						true => match &item.id.to_string() == &search_term.to_ascii_lowercase() {
							true => Some(item),
							false => None
						}
				})
				.map(|result| result_render(result))
				.collect::<Vec<String>>();

			return Ok(results);
		}
		SearchMode::Any => {
			debug!("Loading skills");
			let skills: Vec<api::Skill> = request::get_data(&CONFIG.skills_file, || async {
				let api_skills: Vec<api::ApiSkill> =
					request::request_paginated("skills", &CONFIG.lang).await?;
				Ok(api_skills
					.into_iter()
					.map(|api_skill| api::Skill::from(api_skill))
					.collect())
			})
			.await?;
			debug!(
				"Loaded {} skills stored at '{}'",
				skills.len(),
				CONFIG.skills_file.display()
			);

			debug!("Loading traits");
			let traits: Vec<api::Trait> = request::get_data(&CONFIG.traits_file, || async {
				let api_traits: Vec<api::ApiTrait> =
					request::request_paginated("traits", &CONFIG.lang).await?;
				Ok(api_traits
					.into_iter()
					.map(|api_trait| api::Trait::from(api_trait))
					.collect())
			})
			.await?;
			debug!(
				"Loaded {} traits stored at '{}'",
				traits.len(),
				CONFIG.traits_file.display()
			);

			debug!("Loading items");
			let items: Vec<api::Item> = request::get_data(&CONFIG.items_file, || async {
				let api_items: Vec<api::ApiItem> =
					request::request_paginated("items", &CONFIG.lang).await?;
				Ok(api_items
					.into_iter()
					.map(|api_item| api::Item::from(api_item))
					.collect())
			})
			.await?;
			debug!(
				"Loaded {} items stored at '{}'",
				items.len(),
				CONFIG.items_file.display()
			);

			// ------------------------------------------------------------

			if search_term.is_empty() && ! CONFIG.csv {
				return Ok(vec![]);
			}

			Ok(skills
				.iter()
				.filter_map(|skill| match in_reverse {
					false => match &skill.name.to_ascii_lowercase().contains(&search_term.to_ascii_lowercase()) {
						true => Some(skill),
						false => None
					},
					true => match &skill.id.to_string() == &search_term.to_ascii_lowercase() {
						true => Some(skill),
						false => None
					}
				})	
				.map(|result| format!("{}: {} [SKILL]", result.id, result.name))
				.chain(traits
					.iter()
					.filter_map(|r#trait| match in_reverse {
							false => match &r#trait.name.to_ascii_lowercase().contains(&search_term.to_ascii_lowercase()) {
								true => Some(r#trait),
								false => None
							},
							true => match &r#trait.id.to_string() == &search_term.to_ascii_lowercase() {
								true => Some(r#trait),
								false => None
							}
					})
					.map(|result| format!("{}: {} [TRAIT]", result.id, result.name))
					.chain(items
						.iter()
						.filter_map(|item| match in_reverse {
								false => match &item.name.to_ascii_lowercase().contains(&search_term.to_ascii_lowercase()) {
									true => Some(item),
									false => None
								},
								true => match &item.id.to_string() == &search_term.to_ascii_lowercase() {
									true => Some(item),
									false => None
								}
						})
						.map(|result| format!("{}: {} [ITEM]", result.id, result.name))
					)
				)
			.collect::<Vec<String>>())
		}
	}
}
