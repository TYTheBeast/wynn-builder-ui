use super::ConfigFile;
use crate::messages::{
    ConfigMessage, GearMessage, HppengMessage, Message, PlayerMessage, ThresholdFifthMessage,
    ThresholdFirstMessage, ThresholdFourthMessage, ThresholdSecondMessage, ThresholdThirdMessage,
};
use iced::Task;

impl ConfigFile {
    pub fn update(&mut self, message: ConfigMessage) -> Task<Message> {
        match message {
            ConfigMessage::Gear(gear_message) => match gear_message {
                GearMessage::HelmetSelected(idx, name) => {
                    if let Some(selection) = self.gear.helmet_selections.get_mut(idx) {
                        *selection = Some(name.clone());
                    }
                    self.config.items.helmets.push(name);
                    self.save_config();
                }
                GearMessage::AddHelmet => {
                    self.gear.helmet_selections.push(None);
                }
                GearMessage::ChestplateSelected(idx, name) => {
                    if let Some(selection) = self.gear.chestplate_selections.get_mut(idx) {
                        *selection = Some(name.clone());
                    }
                    self.config.items.chest_plates.push(name);
                    self.save_config();
                }
                GearMessage::AddChestplate => {
                    self.gear.chestplate_selections.push(None);
                }
                GearMessage::LeggingsSelected(idx, name) => {
                    if let Some(selection) = self.gear.leggings_selections.get_mut(idx) {
                        *selection = Some(name.clone());
                    }
                    self.config.items.leggings.push(name);
                    self.save_config();
                }
                GearMessage::AddLeggings => {
                    self.gear.leggings_selections.push(None);
                }
                GearMessage::BootsSelected(idx, name) => {
                    if let Some(selection) = self.gear.boots_selections.get_mut(idx) {
                        *selection = Some(name.clone());
                    }
                    self.config.items.boots.push(name);
                    self.save_config();
                }
                GearMessage::AddBoots => {
                    self.gear.boots_selections.push(None);
                }
                GearMessage::RingsSelected(idx, name) => {
                    if let Some(selection) = self.gear.rings_selections.get_mut(idx) {
                        *selection = Some(name.clone());
                    }
                    self.config.items.rings.push(name);
                    self.save_config();
                }
                GearMessage::AddRings => {
                    self.gear.rings_selections.push(None);
                }
                GearMessage::BraceletsSelected(idx, name) => {
                    if let Some(selection) = self.gear.bracelets_selections.get_mut(idx) {
                        *selection = Some(name.clone());
                    }
                    self.config.items.bracelets.push(name);
                    self.save_config();
                }
                GearMessage::AddBracelets => {
                    self.gear.bracelets_selections.push(None);
                }
                GearMessage::NecklacesSelected(idx, name) => {
                    if let Some(selection) = self.gear.necklaces_selections.get_mut(idx) {
                        *selection = Some(name.clone());
                    }
                    self.config.items.necklaces.push(name);
                    self.save_config();
                }
                GearMessage::AddNecklaces => {
                    self.gear.necklaces_selections.push(None);
                }
                GearMessage::WeaponSelected(name) => {
                    self.gear.selected_weapon = Some(name);
                    self.save_config();
                }
                GearMessage::RemoveHelmet(idx) => {
                    self.gear.helmet_selections.remove(idx);
                    let helmet = self.config.items.helmets.get(idx).cloned();
                    if let Some(helmet) = helmet {
                        self.config.items.helmets.retain(|x| x != &helmet);
                    }
                    self.save_config();
                }
                GearMessage::RemoveChestplate(idx) => {
                    self.gear.chestplate_selections.remove(idx);
                    let chestplate = self.config.items.chest_plates.get(idx).cloned();
                    if let Some(chestplate) = chestplate {
                        self.config.items.chest_plates.retain(|x| x != &chestplate);
                    }
                    self.save_config();
                }
                GearMessage::RemoveLeggings(idx) => {
                    self.gear.leggings_selections.remove(idx);
                    let legging = self.config.items.leggings.get(idx).cloned();
                    if let Some(legging) = legging {
                        self.config.items.leggings.retain(|x| x != &legging);
                    }
                    self.save_config();
                }
                GearMessage::RemoveBoots(idx) => {
                    self.gear.boots_selections.remove(idx);
                    let boot = self.config.items.boots.get(idx).cloned();
                    if let Some(boot) = boot {
                        self.config.items.boots.retain(|x| x != &boot);
                    }
                    self.save_config();
                }
                GearMessage::RemoveRings(idx) => {
                    self.gear.rings_selections.remove(idx);
                    let ring = self.config.items.rings.get(idx).cloned();
                    if let Some(ring) = ring {
                        self.config.items.rings.retain(|x| x != &ring);
                    }
                    self.save_config();
                }
                GearMessage::RemoveBracelets(idx) => {
                    self.gear.bracelets_selections.remove(idx);
                    let bracelet = self.config.items.bracelets.get(idx).cloned();
                    if let Some(bracelet) = bracelet {
                        self.config.items.bracelets.retain(|x| x != &bracelet);
                    }
                    self.save_config();
                }
                GearMessage::RemoveNecklaces(idx) => {
                    self.gear.necklaces_selections.remove(idx);
                    let necklace = self.config.items.necklaces.get(idx).cloned();
                    if let Some(necklace) = necklace {
                        self.config.items.necklaces.retain(|x| x != &necklace);
                    }
                    self.save_config();
                }
            },
            ConfigMessage::Player(player_message) => match player_message {
                PlayerMessage::LevelChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config.player.lvl = content.parse().unwrap_or(1);
                        self.save_config();
                    }
                }
                PlayerMessage::AvailablePointChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config.player.available_point = content.parse().unwrap_or(200);
                        self.save_config();
                    }
                }
                PlayerMessage::BaseHpChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config.player.base_hp = content.parse().unwrap_or(500);
                        self.save_config();
                    }
                }
            },
            ConfigMessage::ThresholdFirst(threshold_first_message) => match threshold_first_message
            {
                ThresholdFirstMessage::HpChanged(content) => {
                    if content.is_empty() {
                        self.config.threshold_first.as_mut().unwrap().min_hp = None;
                    } else if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config.threshold_first.as_mut().unwrap().min_hp =
                            Some(content.parse().unwrap_or(0));
                    }
                    self.save_config();
                }
            },
            ConfigMessage::ThresholdSecond(threshold_second_message) => {
                if let Some(threshold) = self.config.threshold_second.as_mut() {
                    match threshold_second_message {
                        ThresholdSecondMessage::HprRawChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_hpr_raw = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::HprPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_hpr_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::MrChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_mr = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::LsChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_ls = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::MsChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_ms = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::SpdChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_spd = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::SdRawChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_sd_raw = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::SdPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_sd_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::HprChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_hpr = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::ExpBonusChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_exp_bonus = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                    }
                    self.save_config();
                }
            }
            ConfigMessage::ThresholdThird(threshold_third_message) => {
                if let Some(threshold) = self.config.threshold_third.as_mut() {
                    match threshold_third_message {
                        ThresholdThirdMessage::EarthDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_earth_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdThirdMessage::ThunderDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_thunder_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdThirdMessage::WaterDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_water_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdThirdMessage::FireDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_fire_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdThirdMessage::AirDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_air_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                    }
                    self.save_config();
                }
            }
            ConfigMessage::ThresholdFourth(threshold_fourth_message) => {
                if let Some(threshold) = self.config.threshold_fourth.as_mut() {
                    match threshold_fourth_message {
                        ThresholdFourthMessage::NeutralDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_neutral_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::EarthDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_earth_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::ThunderDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_thunder_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::WaterDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_water_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::FireDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_fire_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::AirDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_air_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                    }
                    self.save_config();
                }
            }
            ConfigMessage::ThresholdFifth(threshold_fifth_message) => {
                if let Some(threshold) = self.config.threshold_fifth.as_mut() {
                    match threshold_fifth_message {
                        ThresholdFifthMessage::EarthPointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_earth_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::ThunderPointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_thunder_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::WaterPointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_water_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::FirePointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_fire_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::AirPointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_air_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::EhpChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_ehp = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                    }
                    self.save_config();
                }
            }
            ConfigMessage::Hppeng(hppeng_message) => {
                match hppeng_message {
                    HppengMessage::UrlPrefixChanged(content) => {
                        self.config.hppeng.url_prefix = content;
                    }
                    HppengMessage::UrlSuffixChanged(content) => {
                        self.config.hppeng.url_suffix = content;
                    }
                    HppengMessage::DbPathChanged(content) => {
                        self.config.hppeng.db_path = content;
                    }
                    HppengMessage::MigrationsPathChanged(content) => {
                        self.config.hppeng.migrations_path = content;
                    }
                    HppengMessage::ItemsFileChanged(content) => {
                        self.config.hppeng.items_file = content;
                    }
                    HppengMessage::LogBuildsChanged(value) => {
                        self.config.hppeng.log_builds = value;
                    }
                    HppengMessage::LogDbErrorsChanged(value) => {
                        self.config.hppeng.log_db_errors = value;
                    }
                    HppengMessage::DbRetryCountChanged(content) => {
                        self.config.hppeng.db_retry_count = content.parse().unwrap_or(3);
                    }
                }
                self.save_config();
            }
        }
        Task::none()
    }
}
