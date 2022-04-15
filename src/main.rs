mod api;
mod config;
mod request;
#[cfg(test)]
mod tests;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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

		if let Some(skill_name) = &CONFIG.search_term {
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

		if let Some(trait_name) = &CONFIG.search_term {
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

		if let Some(item_name) = &CONFIG.search_term {
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
	}

	Ok(())
}
