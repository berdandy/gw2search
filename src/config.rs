use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

use lazy_static::lazy_static;

pub const CACHE_PREFIX: &str = "cache_";
pub const PRODUCT_PREFIX: &str = "gw2search";

#[derive(Default, Debug)]
pub struct Config {
	pub any: bool,
	pub skill: bool,
	pub r#trait: bool,
	pub item: bool,
	pub spec: bool,
	pub elite_spec: bool,
	pub profession: bool,
	pub pet: bool,
	pub legend: bool,

	pub quiet: bool,
	pub csv: bool,
	pub json: bool,

    pub reverse: bool,

    pub lang: Option<Language>,

    pub cache_dir: PathBuf,
    pub items_file: PathBuf,
    pub skills_file: PathBuf,
    pub traits_file: PathBuf,
    pub specs_file: PathBuf,
    pub professions_file: PathBuf,
    pub pets_file: PathBuf,
    pub legends_file: PathBuf,

    pub search_term: Option<String>,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

impl Config {
    fn new() -> Self {
        let mut config = Config::default();

        let opt = Opt::from_args();

        if ! opt.reset_data {
            config.any = opt.any;
            config.skill = opt.skill;
            config.r#trait = opt.r#trait;
            config.item = opt.item;
            config.spec = opt.spec;
            config.elite_spec = opt.elite_spec;
            config.profession = opt.profession;
            config.pet = opt.pet;
            config.legend = opt.legend;
            config.reverse = opt.reverse;
            config.quiet = opt.quiet;
            config.csv = opt.csv;
            config.json = opt.json;

            // default (but only when not resetting data)
            if ! (config.any || config.skill || config.r#trait || config.item || config.spec || config.elite_spec || config.profession || config.pet || config.legend) {
                println!("no search type; assuming default item search");
                config.item = true;
            }
        }

        config.lang = opt.lang;
        config.search_term = opt.search_term;

        let file: ConfigFile = match get_file_config(&opt.config_file) {
            Ok(config) => config,
            Err(_) => {
                ConfigFile::default()
            }
        };

        if config.lang.is_none() {
            if let Some(code) = file.lang {
                config.lang = code.parse().map_or_else(
                    |e| {
                        println!("Config file: {}", e);
                        None
                    },
                    Some,
                )
            }
        }

        let cache_dir = cache_dir(&opt.cache_dir).expect("Failed to identify cache dir");
        ensure_dir(&cache_dir).expect("Failed to create cache dir");
		if let Err(e) = flush_cache(&cache_dir) {
			println!("Failed to flush cache dir {}: {}", &cache_dir.display(), e);
		}
        config.cache_dir = cache_dir;

        let data_dir = data_dir(&opt.data_dir).expect("Failed to identify data dir");
        ensure_dir(&data_dir).expect("Failed to create data dir");

        let lang_suffix =
            Language::code(&config.lang).map_or_else(|| "".to_string(), |c| format!("_{}", c));

        let mut items_path = data_dir.clone();
        items_path.push(format!("items{}.bin", lang_suffix));
        config.items_file = items_path;

        let mut skills_path = data_dir.clone();
        skills_path.push(format!("skills{}.bin", lang_suffix));
        config.skills_file = skills_path;

        let mut traits_path = data_dir.clone();
        traits_path.push(format!("traits{}.bin", lang_suffix));
        config.traits_file = traits_path;

        let mut specs_path = data_dir.clone();
        specs_path.push(format!("specs{}.bin", lang_suffix));
        config.specs_file = specs_path;

        let mut professions_path = data_dir.clone();
        professions_path.push(format!("professions{}.bin", lang_suffix));
        config.professions_file = professions_path;

        let mut pets_path = data_dir.clone();
        pets_path.push(format!("pets{}.bin", lang_suffix));
        config.pets_file = pets_path;

        let mut legends_path = data_dir.clone();
        legends_path.push(format!("legends{}.bin", lang_suffix));
        config.legends_file = legends_path;

        if opt.reset_data {
            for file in [&config.items_file, &config.skills_file, &config.traits_file, &config.specs_file, &config.professions_file, &config.pets_file, &config.legends_file] {
				if let Err(e) = remove_data_file(file) {
					println!("Failed to remove file {}: {}", file.display(), e);
				}
            }
        }

        config
    }
}

fn get_file_config(file: &Option<PathBuf>) -> Result<ConfigFile, Box<dyn std::error::Error>> {
    let mut file = File::open(config_file(file)?)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(toml::from_str(&s)?)
}

#[derive(Debug, Default, Deserialize)]
struct ConfigFile {
    lang: Option<String>,
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Search for skill, trait or item
    #[structopt(short = "a", long)]
    any: bool,

    /// Search for skill
    #[structopt(short = "s", long)]
    skill: bool,

    /// Search for trait
    #[structopt(short = "t", long)]
    r#trait: bool,

    /// Search for item (default)
    #[structopt(short = "i", long)]
    item: bool,

    /// Search for specialization
    #[structopt(short = "E", long)]
    elite_spec: bool,

    /// Search for only elite specializations
    #[structopt(short = "S", long)]
    spec: bool,

    /// Search for professions
    #[structopt(short = "p", long)]
    profession: bool,

    /// Search for pets
    #[structopt(short = "P", long)]
    pet: bool,

    /// Search for legends
    #[structopt(short = "l", long)]
    legend: bool,

    /// Search for id instead of name
    #[structopt(short = "r", long)]
    reverse: bool,

	/// Only print id number results upon match
	#[structopt(short = "q", long)]
	quiet: bool,

	/// Print results as csv-formatted data
	#[structopt(short = "c", long)]
	csv: bool,

	/// Print results as json-formatted data
	#[structopt(short = "j", long)]
	json: bool,

    /// Search for this
    search_term: Option<String>,

    /// Download content from the GW2 API, replacing any previously cached content
    #[structopt(long)]
    reset_data: bool,

    #[structopt(long, parse(from_os_str), help = &CACHE_DIR_HELP)]
    cache_dir: Option<PathBuf>,

    #[structopt(long, parse(from_os_str), help = &DATA_DIR_HELP)]
    data_dir: Option<PathBuf>,

    #[structopt(long, parse(from_os_str), help = &CONFIG_FILE_HELP)]
    config_file: Option<PathBuf>,

    /// One of "en", "es", "de", or "fr". Defaults to "en"
    // /// One of "en", "es", "de", "fr", or "zh". Defaults to "en"
    #[structopt(long, parse(try_from_str = get_lang))]
    lang: Option<Language>,
}

static CACHE_DIR_HELP: Lazy<String> = Lazy::new(|| {
    format!(
        r#"Save cached API calls to this directory

If provided, the parent directory of the cache directory must already exist. Defaults to '{}'."#,
        cache_dir(&None).unwrap().display()
    )
});

static DATA_DIR_HELP: Lazy<String> = Lazy::new(|| {
    format!(
        r#"Save cached items and other content to this directory

If provided, the parent directory of the cache directory must already exist. Defaults to '{}'."#,
        data_dir(&None).unwrap().display()
    )
});

static CONFIG_FILE_HELP: Lazy<String> = Lazy::new(|| {
    format!(
        r#"Read config options from this file. Supported options:

    lang = "<lang>"

The default file location is '{}'."#,
        config_file(&None).unwrap().display()
    )
});

#[derive(Debug, EnumString, EnumVariantNames)]
pub enum Language {
    #[strum(serialize = "en")]
    English,
    #[strum(serialize = "es")]
    Spanish,
    #[strum(serialize = "de")]
    German,
    #[strum(serialize = "fr")]
    French,
}
impl Language {
    pub fn code(lang: &Option<Language>) -> Option<&str> {
        if let Some(lang) = lang {
            match lang {
                Language::English => None, // English is the default, so leave it off
                Language::Spanish => Some("es"),
                Language::German => Some("de"),
                Language::French => Some("fr"),
            }
        } else {
            None
        }
    }
}

fn get_lang<Language: FromStr + VariantNames>(
    code: &str,
) -> Result<Language, Box<dyn std::error::Error>> {
    Language::from_str(code).map_err(|_| {
        format!(
            "Invalid language: {} (valid values are {})",
            code,
            Language::VARIANTS.join(", ")
        )
        .into()
    })
}

#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Display, EnumString, EnumVariantNames,
)]
pub enum Discipline {
    Artificer,
    Armorsmith,
    Chef,
    Huntsman,
    Jeweler,
    Leatherworker,
    Tailor,
    Weaponsmith,
    Scribe,
    // A few more for compatibility with gw2efficiency
    #[strum(serialize = "Mystic Forge")]
    MysticForge,
    #[strum(serialize = "Double Click")]
    DoubleClick,
    Salvage,
    Merchant,
    Charge,
    Achievement,
    Growing,
}

fn ensure_dir(dir: &PathBuf) -> Result<&PathBuf, Box<dyn std::error::Error>> {
    if !dir.exists() {
        std::fs::create_dir(dir)
            .map_err(|e| format!("Failed to create '{}' ({})", dir.display(), e).into())
            .and(Ok(dir))
    } else {
        Ok(dir)
    }
}

fn cache_dir(dir: &Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(dir) = dir {
        return Ok(dir.clone());
    }
    dirs::cache_dir()
        .filter(|d| d.exists())
        .map(|mut cache_dir| {
            cache_dir.push(PRODUCT_PREFIX);
            cache_dir
        })
        .or_else(|| std::env::current_dir().ok())
        .ok_or_else(|| "Failed to access current working directory".into())
}

fn flush_cache(cache_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // flush any cache files older than 5 mins - which is how long the API caches url results.
    // Assume our request triggered the cache
    // Give a prefix; on Windows the user cache and user local data folders are the same
    let expired = SystemTime::now() - Duration::new(300, 0);
    for file in fs::read_dir(cache_dir)? {
        let file = file?;
        let filename = file.file_name().into_string();
        if let Ok(name) = filename {
            if !name.starts_with(CACHE_PREFIX) {
                continue;
            }
        }
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            continue;
        }
        if metadata.created()? <= expired {
            fs::remove_file(file.path())?;
        }
    }
    Ok(())
}

fn data_dir(dir: &Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(dir) = dir {
        return Ok(dir.clone());
    }
    dirs::data_dir()
        .filter(|d| d.exists())
        .map(|mut data_dir| {
            data_dir.push(PRODUCT_PREFIX);
            data_dir
        })
        .or_else(|| std::env::current_dir().ok())
        .ok_or_else(|| "Failed to access current working directory".into())
}

pub fn remove_data_file(file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if file.exists() {
        println!("Removing existing data file at '{}'", file.display());
        std::fs::remove_file(file)
            .map_err(|e| format!("Failed to remove '{}' ({})", file.display(), e))?;
    }
    Ok(())
}

fn config_file(file: &Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(file) = file {
        return Ok(file.clone());
    }
    dirs::config_dir()
        .filter(|d| d.exists())
        .map(|mut config_dir| {
            config_dir.push(PRODUCT_PREFIX);
            config_dir
        })
        .or_else(|| std::env::current_dir().ok())
        .map(|mut path| {
            path.push(PRODUCT_PREFIX.to_owned() + ".toml");
            path
        })
        .ok_or_else(|| "Failed to access current working directory".into())
}
