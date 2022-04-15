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
		println!("TODO!");

	} else if CONFIG.r#trait {
		debug!("Loading traits");
		println!("TODO!");

	} else { // if CONFIG.item 
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

		if let Some(item_name) = &CONFIG.item_name {
			let results: Vec<_> = items
				//.into_iter()
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
