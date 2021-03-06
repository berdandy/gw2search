mod api;
mod config;
mod request;
#[cfg(test)]
mod tests;

use iced::{
    scrollable, text_input, pick_list, Alignment, Color, Column, Element,
	PickList, Row, Rule, Sandbox, Scrollable, Settings, Text, TextInput,
};

use config::CONFIG;
use std::env;

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
        println!("Running in command line mode. Run with no options to open gui");
        let mode : SearchMode = match &CONFIG {
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
            let results = match search_api(mode, term) {
                Ok(results) => results,
                Err(e) => panic!("error searching with commandline search: {}", e),
            };
            for result in results {
                println!("{}", result);
            }
        }
        Ok(())
    } else {
        Gw2Search::run(Settings::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchMode {
	Item,
	Skill,
	Trait,
    Skip,
}
impl Default for SearchMode {
    fn default() -> Self { SearchMode::Item }
}
impl SearchMode {
    const ALL: [SearchMode; 3] = [
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
	results: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
	Search,
	SearchTermChanged(String),
	SearchModeSelected(SearchMode),
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
					match search_api(self.search_mode, self.search_term.clone()) {
						Ok(results) => {
							self.results = results;
						}
						Err(error) => panic!("Problem with search {:?}", error)
					}
				}
			}
			Message::Search => {
				match search_api(self.search_mode, self.search_term.clone()) {
					Ok(results) => {
						self.results = results;
					}
					Err(error) => panic!("Problem with search {:?}", error)
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
            .push(Rule::horizontal(20))
            .push(
				Row::new()
				.push(TextInput::new(
						&mut self.input,
						"Search Term",
						&self.search_term,
						Message::SearchTermChanged
					).on_submit(Message::Search)
				)
				.push(
					PickList::new(
						&mut self.pick_list,
						&SearchMode::ALL[..],
						Some(self.search_mode),
						Message::SearchModeSelected)
				)
			)
			.push(Text::new("input search term and hit [ENTER] to search"))
			.push(Text::new("WARNING: BE PATIENT. The first search of each mode can take up to several minutes to cache")
				.size(14)
				.color(Color::from_rgb8(0xDD, 0x22, 0x22))
			)
			.push(Text::new("There is no feedback for the download unless running from console/cmd").size(12))
			.push(Text::new("To reset cache, delete files in %APPDATA%\\Roaming\\gw2search").size(12))
			.push(Text::new("If you find this useful, tips welcomed in-game at berdandy.1968 :)").size(16))
			.push(
				Scrollable::new(&mut self.scroll)
					.padding(40)
					.push(results)
			)
			.into()
		}
}

#[tokio::main]
async fn search_api(search_mode: SearchMode, search_term: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

			let skill_name = &search_term;
			let results: Vec<_> = skills
				.iter()
				.filter_map(|skill| match &skill.name.to_ascii_lowercase().contains(&skill_name.to_ascii_lowercase()) {
					true => Some(skill),
					false => None
				})
				.map(|result| format!("{}: {}", result.id, result.name))
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

			let trait_name = &search_term;
			let results: Vec<_> = traits
				.iter()
				.filter_map(|r#trait| match &r#trait.name.to_ascii_lowercase().contains(&trait_name.to_ascii_lowercase()) {
					true => Some(r#trait),
					false => None
				})
				.map(|result| format!("{}: {}", result.id, result.name))
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

			let item_name = &search_term;
			let results: Vec<_> = items
				.iter()
				.filter_map(|item| match &item.name.to_ascii_lowercase().contains(&item_name.to_ascii_lowercase()) {
					true => Some(item),
					false => None
				})
				.map(|result| format!("{}: {}", result.id, result.name))
				.collect::<Vec<String>>();

			return Ok(results);
		}
	}
}
