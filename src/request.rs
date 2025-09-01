#![allow(dead_code)]

use bincode::{deserialize_from, serialize_into};
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use futures::{stream, StreamExt};

use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::path::PathBuf;

use crate::config;

const PARALLEL_REQUESTS: usize = 10;
const MAX_PAGE_SIZE: i32 = 200; // https://wiki.guildwars2.com/wiki/API:2#Paging
const MAX_ITEM_ID_LENGTH: i32 = 200; // error returned for greater than this amount
const API_VERSION: &str = "2025-09-01T00:00:00Z";

pub async fn get_data<T, Fut>(
    data_path: impl AsRef<Path>,
    getter: impl FnOnce() -> Fut,
) -> Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
    Fut: Future<Output = Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>>,
{
    if let Ok(file) = File::open(&data_path) {
        let bitstream = DeflateDecoder::new(file);
        deserialize_from(bitstream).map_err(|e| {
            format!(
                "Failed to deserialize existing data at '{}' ({}). \
                 Try using the --reset-data flag to replace the data files.",
                data_path.as_ref().display(),
                e,
            )
            .into()
        })
    } else {
        let items = getter().await?;

        let file = File::create(data_path)?;
        let bitstream = DeflateEncoder::new(file, Compression::default());
        serialize_into(bitstream, &items)?;

        Ok(items)
    }
}

pub async fn request_paginated<T>(
    url_path: &str,
    lang: &Option<config::Language>,
) -> Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    let mut page_no = 0;
    let mut page_total = None;

    // update page total with first request
    let mut items: Vec<T> = request_page(url_path, page_no, &mut page_total, lang).await?;

    // fetch remaining pages in parallel batches
    page_no += 1;

    // try fetching one extra page in case page total increased while paginating
    // ***this is not needed for gw2search, all mostly static, this is holdover code from trade stuff
    // let page_total = page_total.expect("Missing page total") + 1;
    let page_total = page_total.expect("Missing page total");

    let request_results = stream::iter((page_no..page_total).map(|page_no| async move {
        request_page::<T>(url_path, page_no, &mut Some(page_total), lang).await
    }))
    .buffered(PARALLEL_REQUESTS)
    .collect::<Vec<Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>>>()
    .await;

    for result in request_results.into_iter() {
        let mut new_items = result?;
        items.append(&mut new_items);
    }

    Ok(items)
}

async fn request_page<T>(
    url_path: &str,
    page_no: usize,
    page_total: &mut Option<usize>,
    lang: &Option<config::Language>,
) -> Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    let url = if let Some(code) = config::Language::code(lang) {
        format!(
            "https://api.guildwars2.com/v2/{}?lang={}&page={}&page_size={}&v={}",
            url_path, code, page_no, MAX_PAGE_SIZE, API_VERSION
        )
    } else {
        format!(
            "https://api.guildwars2.com/v2/{}?page={}&page_size={}&v={}",
            url_path, page_no, MAX_PAGE_SIZE, API_VERSION
        )
    };

    eprintln!("Fetching {}", url);
    let response = reqwest::get(&url).await?;
    if page_total.is_none() {
        let page_total_str = response
            .headers()
            .get("X-Page-Total")
            .expect("Missing X-Page-Total header")
            .to_str()
            .expect("X-Page-Total header contains invalid string");
        *page_total =
            Some(page_total_str.parse().unwrap_or_else(|_| {
                panic!("X-Page-Total is an invalid integer: {}", page_total_str)
            }));
    }

    let txt = response.text().await?;
    if txt.contains("page out of range") {
        return Ok(vec![]);
    }
    let de = &mut serde_json::Deserializer::from_str(&txt);
    serde_path_to_error::deserialize(de).map_err(|e| e.into())
}

async fn cached_fetch<T>(
    url: &str,
    display: Option<&str>,
    cache_dir: &Path,
) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    let cache_path = url_to_cache_path(url, cache_dir);
    if let Ok(file) = File::open(&cache_path) {
        let stream = DeflateDecoder::new(file);
        let v = deserialize_from(stream)?;
        return Ok(v);
    }

    let v = fetch(url, display).await?;

    // save cache file
    let file = File::create(cache_path)?;
    let stream = DeflateEncoder::new(file, Compression::default());
    serialize_into(stream, &v)?;

    Ok(v)
}

async fn fetch<T>(url: &str, display: Option<&str>) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    if let Some(url) = display {
        eprintln!("Fetching {}", url);
    } else {
        eprintln!("Fetching {}", url);
    }

    let response = reqwest::get(url).await?;
    let status = response.status();
    if !status.is_success() {
        let err: serde_json::value::Value = response.json().await?;
        let text = err
            .get("text")
            .and_then(|text| text.as_str())
            .unwrap_or_else(|| status.as_str());
        return Err(text.into());
    }

    let bytes = response.bytes().await?;
    let de = &mut serde_json::Deserializer::from_slice(&bytes);
    let v: T = serde_path_to_error::deserialize(de)?;

    Ok(v)
}

fn url_to_cache_path(url: &str, cache_dir: &Path) -> PathBuf {
    let mut hash = DefaultHasher::new();
    url.hash(&mut hash);
    let hash = hash.finish();

    let mut path = cache_dir.to_owned();
    path.push(format!("{}{}", config::CACHE_PREFIX, hash));
    path
}
