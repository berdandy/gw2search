mod api;
mod config;
mod request;
#[cfg(test)]
mod tests;

use iced::{
    button, scrollable, text_input, Align, Button, Column, Container, Element, Length,
    ProgressBar, Radio, Row, Rule, Sandbox, Scrollable, Settings, Space, Text, TextInput,
};

use config::CONFIG;

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
    Gw2Search::run(Settings::default())
}

struct Gw2Search {
    scroll: scrollable::State,
    input: text_input::State,
    search_term: String,
	results: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
	Search,
	SearchTermChanged(String),
	// SearchComplete(Vec<String>),
	// Scrolled(usize, f32),
}

impl Sandbox for Gw2Search {
    type Message = Message;

	fn new() -> Self {
		Gw2Search {
			scroll: scrollable::State::new(),
			input: text_input::State::new(),
			search_term: String::new(), 
			results: Vec::new(),
		}
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
			Message::Search => {
				println!("TODO: SEARCH {}", self.search_term);
				match search_api(self.search_term.clone()) {
					Ok(()) => println!("done"),
					Err(error) => panic!("Problem with search {:?}", error)
				}
			}
		}
	}

	fn view(&mut self) -> Element<Self::Message> {

		let results_col = Column::new()
			.spacing(20)
			.padding(20)
			.align_items(Align::Start);
			//.push(self.results.iter().map(|&result| Text::new(result)));

		for result in &self.results {
			results_col.push(Text::new(result));
		}

		Column::new()
            .spacing(20)
			.padding(20)
			.align_items(Align::Center)
            .push(Text::new("Gw2Search"))
            .push(Rule::horizontal(20))
            .push(
				TextInput::new(
					&mut self.input,
					"Search Term",
					&self.search_term,
					Message::SearchTermChanged
				).on_submit(Message::Search)
			)
			.push(
				Scrollable::new(&mut self.scroll)
					.padding(40)
					.push(results_col)
			)
			.into()
		}
}

#[tokio::main]
async fn search_api(search_term: String) -> Result<(), Box<dyn std::error::Error>> {
	if CONFIG.skill {
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
			"Loaded {} items stored at '{}'",
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
			.collect();

		println!("Results found: {}", results.len());

		for result in results {
			println!("{} : {}",
				result.id,
				result.name
			);
		}

	} else if CONFIG.r#trait {
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
			"Loaded {} items stored at '{}'",
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
			.collect();

		println!("Results found: {}", results.len());

		for result in results {
			println!("{} : {}",
				result.id,
				result.name
			);
		}

	} else if CONFIG.item {
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
			.collect();

		println!("Results found: {}", results.len());

		for result in results {
			println!("{} : {} [{:?}]",
				result.id,
				result.name,
				result.item_type
			);
		}
	}

	Ok(())
}