use iced_widget::combo_box;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct GearSelections {
    pub helmets: combo_box::State<String>,
    pub helmet_selections: Vec<Option<String>>,
    pub chestplates: combo_box::State<String>,
    pub chestplate_selections: Vec<Option<String>>,
    pub leggings: combo_box::State<String>,
    pub leggings_selections: Vec<Option<String>>,
    pub boots: combo_box::State<String>,
    pub boots_selections: Vec<Option<String>>,
    pub rings: combo_box::State<String>,
    pub rings_selections: Vec<Option<String>>,
    pub bracelets: combo_box::State<String>,
    pub bracelets_selections: Vec<Option<String>>,
    pub necklaces: combo_box::State<String>,
    pub necklaces_selections: Vec<Option<String>>,
    pub weapons: combo_box::State<String>,
    pub selected_weapon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GearList {
    pub items: Vec<Gear>,
}

impl GearList {
    pub fn from_json(path: &str) -> Result<Self, String> {
        let items_json_string = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read items file: {}", e))?;

        match serde_json::from_str::<GearList>(&items_json_string) {
            Ok(gear_list) => Ok(gear_list),
            Err(e) => {
                eprintln!("Deserialization error: {}", e);
                Err(format!("Failed to parse items JSON: {}", e))
            }
        }
    }

    pub fn get_gear_by_type(&self, gear_type: GearType) -> Vec<String> {
        self.items
            .iter()
            .filter(|gear| gear.gear_type == gear_type)
            .map(|gear| gear.name.clone())
            .collect()
    }

    // Helper methods for getting specific gear types
    pub fn helmets(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Helmet)
    }

    pub fn chestplates(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Chestplate)
    }

    pub fn leggings(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Leggings)
    }

    pub fn boots(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Boots)
    }

    pub fn rings(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Ring)
    }

    pub fn bracelets(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Bracelet)
    }

    pub fn necklaces(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Necklace)
    }

    pub fn weapons(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Spear)
            .into_iter()
            .chain(self.get_gear_by_type(GearType::Wand))
            .chain(self.get_gear_by_type(GearType::Bow))
            .chain(self.get_gear_by_type(GearType::Dagger))
            .chain(self.get_gear_by_type(GearType::Relik))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Gear {
    id: i64,
    name: String,
    tier: String,
    #[serde(rename = "type")]
    gear_type: GearType,
}

impl Gear {
    pub fn default_for_type(gear_type: GearType, name: &str, id: i64) -> Self {
        Self {
            id,
            name: name.to_string(),
            tier: String::from("Common"),
            gear_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum GearType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Ring,
    Bracelet,
    Necklace,
    Bow,
    Spear,
    Wand,
    Dagger,
    Relik,
    #[default]
    None,
}
