#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;

use phf::{phf_map, phf_set};
use strum::Display;

use crate::config;
use config::CONFIG;

pub trait ResultRender {
    fn pretty(&self) -> String;
    fn id_only(&self) -> String;
    fn csv(&self) -> String; // id,name
}

// types for /skills
#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: u32,
    pub name: String,
    // TODO: the rest?
}

impl ResultRender for Skill {
    fn pretty(&self) -> String  { format!("{}: {}", self.id, self.name) }
    fn id_only(&self) -> String { format!("{}", self.id) }
    fn csv(&self) -> String    { format!("{},{}", self.id, self.name) }
}

pub fn result_render(result: &impl ResultRender) -> String {
    match CONFIG.quiet {
        true => result.id_only(),
        false => match CONFIG.csv {
            true => result.csv(),
            false => result.pretty()
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct ApiSkill(Skill);

impl<'de> Deserialize<'de> for ApiSkill {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct SkillDeser {
            pub id: u32,
            pub name: String,
        }

        let skill = SkillDeser::deserialize(d)?;
        Ok(ApiSkill(Skill {
            id: skill.id,
            name: skill.name,
        }))
    }
}

impl From<ApiSkill> for Skill {
    fn from(skill: ApiSkill) -> Self {
        Skill {
            id: skill.0.id,
            name: skill.0.name,
        }
    }
}

// types for /traits
#[derive(Debug, Serialize, Deserialize)]
pub struct Trait {
    pub id: u32,
    pub name: String,
    // TODO: the rest?
}

impl ResultRender for Trait {
    fn pretty(&self) -> String  { format!("{}: {}", self.id, self.name) }
    fn id_only(&self) -> String { format!("{}", self.id) }
    fn csv(&self) -> String    { format!("{},{}", self.id, self.name) }
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct ApiTrait(Trait);

impl<'de> Deserialize<'de> for ApiTrait {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct TraitDeser {
            pub id: u32,
            pub name: String,
        }

        let r#trait = TraitDeser::deserialize(d)?;
        Ok(ApiTrait(Trait {
            id: r#trait.id,
            name: r#trait.name,
        }))
    }
}

impl From<ApiTrait> for Trait {
    fn from(r#trait: ApiTrait) -> Self {
        Trait {
            id: r#trait.0.id,
            name: r#trait.0.name,
        }
    }
}

// types for /items
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    rarity: ItemRarity,
    level: i32,
    vendor_value: i32,
    flags: Vec<ItemFlag>,
    restrictions: Vec<String>,
    upgrades_into: Option<Vec<ItemUpgrade>>,
    upgrades_from: Option<Vec<ItemUpgrade>>,
    details: Option<ItemDetails>,
}

impl ResultRender for Item {
    fn pretty(&self) -> String  { format!("{}: {}", self.id, self.name) }
    fn id_only(&self) -> String { format!("{}", self.id) }
    fn csv(&self) -> String    { format!("{},{}", self.id, self.name) }
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct ApiItem(Item);

impl<'de> Deserialize<'de> for ApiItem {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct ItemDeser {
            pub id: u32,
            pub name: String,
            #[serde(rename = "type")]
            item_type: ItemType,
            rarity: ItemRarity,
            level: i32,
            vendor_value: i32,
            flags: Vec<ItemFlag>,
            restrictions: Vec<String>,
            upgrades_into: Option<Vec<ItemUpgrade>>,
            upgrades_from: Option<Vec<ItemUpgrade>>,
            #[serde(default)]
            details: Option<serde_json::Value>,
        }

        let item = ItemDeser::deserialize(d)?;
        let details = match (&item.item_type, item.details) {
            (ItemType::Consumable, Some(details)) => Some(ItemDetails::Consumable(
                serde_json::from_value(details).map_err(de::Error::custom)?,
            )),
            _ => None,
        };

        Ok(ApiItem(Item {
            id: item.id,
            name: item.name,
            item_type: item.item_type,
            rarity: item.rarity,
            level: item.level,
            vendor_value: item.vendor_value,
            flags: item.flags,
            restrictions: item.restrictions,
            upgrades_into: item.upgrades_into,
            upgrades_from: item.upgrades_from,
            details,
        }))
    }
}

impl From<ApiItem> for Item {
    fn from(item: ApiItem) -> Self {
        Item {
            id: item.0.id,
            name: item.0.name,
            item_type: item.0.item_type,
            rarity: item.0.rarity,
            level: item.0.level,
            vendor_value: item.0.vendor_value,
            flags: item.0.flags,
            restrictions: item.0.restrictions,
            upgrades_into: item.0.upgrades_into,
            upgrades_from: item.0.upgrades_from,
            details: item.0.details,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    Armor,
    Back,
    Bag,
    Consumable,
    Container,
    CraftingMaterial,
    Gathering,
    Gizmo,
    Key,
    MiniPet,
    JadeTechModule,
    PowerCore,
    Tool,
    Trait,
    Trinket,
    Trophy,
    UpgradeComponent,
    Weapon,
    FishingRod,
    FishingBait,
    FishingLure,
    SensoryArray,
    ServiceChip,
    Relic,
}

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum ItemRarity {
    Junk,
    Basic,
    Fine,
    Masterwork,
    Rare,
    Exotic,
    Ascended,
    Legendary,
}

impl ItemRarity {
    fn crafted_localized(&self) -> String {
        let lang = CONFIG.lang.as_ref().unwrap_or(&config::Language::English);
        // NOTE: these strings were extracted by hand from client crafting interface
        match lang {
            config::Language::English => match &self {
                Self::Masterwork => "Master".to_string(),
                _ => self.to_string(),
            },
            config::Language::Spanish => match &self {
                Self::Masterwork => "maestro".to_string(),
                Self::Rare => "excepcional".to_string(),
                Self::Exotic => "exótico".to_string(),
                Self::Ascended => "Ascendido".to_string(),
                _ => self.to_string(),
            },
            config::Language::German => match &self {
                Self::Masterwork => "Meister".to_string(),
                Self::Rare => "Selten".to_string(),
                Self::Exotic => "Exotisch".to_string(),
                Self::Ascended => "Aufgestiegen".to_string(),
                _ => self.to_string(),
            },
            config::Language::French => match &self {
                Self::Masterwork => "Maître".to_string(),
                // Rare is the same in French
                Self::Exotic => "Exotique".to_string(),
                Self::Ascended => "Elevé".to_string(),
                _ => self.to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ItemFlag {
    AccountBindOnUse,
    AccountBound,
    Attuned,
    BulkConsume,
    DeleteWarning,
    HideSuffix,
    Infused,
    MonsterOnly,
    NoMysticForge,
    NoSalvage,
    NoSell,
    NotUpgradeable,
    NoUnderwater,
    SoulbindOnAcquire,
    SoulBindOnUse,
    Tonic,
    Unique,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemDetails {
    Consumable(ItemConsumableDetails),
    // don't care about the rest for now
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemConsumableDetails {
    #[serde(rename = "type")]
    consumable_type: ItemConsumableType,
    recipe_id: Option<u32>,
    extra_recipe_ids: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemConsumableType {
    AppearanceChange,
    Booze,
    ContractNpc,
    Currency,
    Food,
    Generic,
    Halloween,
    Immediate,
    MountRandomUnlock,
    RandomUnlock,
    Transmutation,
    Unlock,
    UpgradeRemoval,
    Utility,
    TeleportToFriend,
}

// NOTE: most can only be purchased in blocks of 10 - we ignore that for now
// NOTE: doesn't include karma purchases, since the karma to gold rate is undefined and we don't
// support multiple currencies
static VENDOR_ITEMS: phf::Set<u32> = phf_set! {
    19792_u32, // Spool of Jute Thread - 10
    19789_u32, // Spool of Wool Thread - 10
    19794_u32, // Spool of Cotton Thread - 10
    19793_u32, // Spool of Linen Thread - 10
    19791_u32, // Spool of Silk Thread - 10
    19790_u32, // Spool of Gossamer Thread - 10
    13010_u32, // Minor Rune of Holding
    13006_u32, // Rune of Holding
    13007_u32, // Major Rune of Holding
    13008_u32, // Greater Rune of Holding
    13009_u32, // Superior Rune of Holding
    19704_u32, // Lump of Tin - 10
    19750_u32, // Lump of Coal - 10
    19924_u32, // Lump of Primordium - 10
    12157_u32, // Jar of Vinegar - 10
    12151_u32, // Packet of Baking Powder - 10
    12158_u32, // Jar of Vegetable Oil - 10
    12153_u32, // Packet of Salt - 10
    12155_u32, // Bag of Sugar - 10
    12156_u32, // Jug of Water - 10 - only 10?
    12324_u32, // Bag of Starch - 10
    12136_u32, // Bag of Flour - 1, from some vendors, 10 from master chefs
    12271_u32, // Bottle of Soy Sauce - 10
    76839_u32, // Milling Basin - can buy one at a time from chefs and scribe
    70647_u32, // Crystalline Bottle - can buy one at a time from master scribe
    75762_u32, // Bag of Mortar - can buy one at a time from master scribe
    75087_u32, // Essence of Elegance - buy one at a time
};
// Sell price is _not_ buy price * 8
static SPECIAL_VENDOR_ITEMS: phf::Map<u32, i32> = phf_map! {
    46747_u32 => 150, // Thermocatalytic Reagent - 1496 for 10
    91739_u32 => 150, // Pile of Compost Starter - 1496 for 10
    91702_u32 => 200, // Pile of Powdered Gelatin Mix - 5 for 1000; prereq achievement
    90201_u32 => 40000, // Smell-Enhancing Culture; prereq achievement
};
impl Item {
    pub fn vendor_cost(&self) -> Option<i32> {
        if VENDOR_ITEMS.contains(&self.id) {
            if self.vendor_value > 0 {
                // standard vendor sell price is generally buy price * 8, see:
                //  https://forum-en.gw2archive.eu/forum/community/api/How-to-get-the-vendor-sell-price
                Some(self.vendor_value * 8)
            } else {
                None
            }
        } else if SPECIAL_VENDOR_ITEMS.contains_key(&self.id) {
            Some(SPECIAL_VENDOR_ITEMS[&self.id])
        } else {
            None
        }
    }

    pub fn is_restricted(&self) -> bool {
        // 76363 == legacy catapult schematic
        self.id == 76363
            || self
                .flags
                .iter()
                .any(|flag| *flag == ItemFlag::AccountBound || *flag == ItemFlag::SoulbindOnAcquire)
    }

    pub fn is_common_ascended_material(&self) -> bool {
        // Empyreal Fragment, Dragonite Ore, Pile of Bloodstone Dust
        self.id == 46735 || self.id == 46733 || self.id == 46731
    }

    pub fn recipe_unlocks(&self) -> Option<Vec<u32>> {
        match (&self.item_type, &self.details) {
            (ItemType::Consumable, Some(ItemDetails::Consumable(details))) => {
                let mut unlocks = vec![];
                if let Some(recipe_id) = details.recipe_id {
                    unlocks.push(recipe_id);
                } else if let Some(extra_recipe_ids) = &details.extra_recipe_ids {
                    unlocks.extend(extra_recipe_ids);
                }
                Some(unlocks)
            }
            (ItemType::Consumable, None) => {
                eprintln!("Item {} is a consumable with no details", self.id);
                None
            }
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn mock(id: u32, name: &str, vendor_value: i32) -> Self {
        Item {
            id,
            name: name.to_string(),
            vendor_value,
            item_type: ItemType::Armor,
            rarity: ItemRarity::Junk,
            level: 0,
            flags: vec![],
            restrictions: vec![],
            upgrades_into: None,
            upgrades_from: None,
            details: None,
        }
    }
}

// When printing an item, add rarity if a trinket, as most trinkets use the same
// name for different rarities
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let ItemType::Trinket = &self.item_type {
            write!(f, "{} ({})", &self.name, &self.rarity.crafted_localized())
        } else {
            write!(f, "{}", &self.name)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemUpgrade {
    upgrade: String,
    item_id: i32,
}