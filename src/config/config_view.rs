use iced::{
    alignment::{Horizontal, Vertical},
    Length,
};
use iced_widget::{
    button, checkbox, column, combo_box, container, row, scrollable, text, text_input, Container,
};

use super::style::*;
use super::ConfigFile;
use crate::{
    build_config::{
        ThresholdFifth, ThresholdFirst, ThresholdFourth, ThresholdSecond, ThresholdThird,
    },
    messages::Message,
    ConfigMessage, GearMessage, HppengMessage, PlayerMessage, ThresholdFifthMessage,
    ThresholdFirstMessage, ThresholdFourthMessage, ThresholdSecondMessage, ThresholdThirdMessage,
};

impl ConfigFile {
    pub fn view(&self) -> Container<'_, Message> {
        let content = column![
                text("Edit Configuration File").size(HEADER),
                text("The configuration will be saved automatically when you edit.").size(SUBHEAD),
                text("If the items say \"No items found\", check that items.json is present in the config folder, and close and re-open the application.").size(SUBHEAD),
                // Error if no items.json file is found
                if let Some(error_message) = &self.error_message {
                    text(error_message).size(SUBHEAD).color(ERROR)
                } else {
                    text("").size(SUBHEAD)
                },
                text("Using this tab when there's a red error message above is undocumented and unexpected behavior, here be dragons!").size(SUBHEAD).color(WARNING),
                text("Player Settings").size(SUBHEAD),
                // Player settings
                container(
                    column![
                        row![
                            text("Player Level:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter level (1-106)...",
                                &self.config.player.lvl.to_string()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Player(PlayerMessage::LevelChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Available Points:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter points...", 
                                &self.config.player.available_point.to_string()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Player(PlayerMessage::AvailablePointChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Base HP:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter base HP...",
                                &self.config.player.base_hp.to_string()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Player(PlayerMessage::BaseHpChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                    ]
                )
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(SPACE),
                // Hppeng settings
                text("Hppeng Settings").size(SUBHEAD),
                container(
                    column![
                        row![
                            text("URL Prefix:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter URL prefix...",
                                &self.config.hppeng.url_prefix
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::UrlPrefixChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("URL Suffix:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter URL suffix...",
                                &self.config.hppeng.url_suffix
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::UrlSuffixChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Database Path:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter database path...",
                                &self.config.hppeng.db_path
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::DbPathChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Migrations Path:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter migrations path...",
                                &self.config.hppeng.migrations_path
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::MigrationsPathChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Items File:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter items file path...",
                                &self.config.hppeng.items_file
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::ItemsFileChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Log Builds:").width(Length::Fixed(LABEL_WIDTH)),
                            checkbox(
                                "",
                                self.config.hppeng.log_builds,
                            )
                            .on_toggle(|value| Message::Config(ConfigMessage::Hppeng(HppengMessage::LogBuildsChanged(value))))
                            .size(TEXT)
                            .spacing(SPACE),
                        ],
                        row![
                            text("Log DB Errors:").width(Length::Fixed(LABEL_WIDTH)),
                            checkbox(
                                "",
                                self.config.hppeng.log_db_errors,
                            )
                            .on_toggle(|value| Message::Config(ConfigMessage::Hppeng(HppengMessage::LogDbErrorsChanged(value))))
                            .size(TEXT)
                            .spacing(SPACE),
                        ],
                        row![
                            text("DB Retry Count:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter retry count...",
                                &self.config.hppeng.db_retry_count.to_string()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::DbRetryCountChanged(input))))
                            .size(TEXT)
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                    ]
                )
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(SPACE),
                // Threshold First settings
                text("Threshold First Settings").size(SUBHEAD),
                container(
                    column![
                        row![
                    text("Min HP:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Health Points...",
                        &self.config.threshold_first.as_ref().unwrap_or(&ThresholdFirst::default()).min_hp
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFirst(ThresholdFirstMessage::HpChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                        ],
                    ]
                )
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(SPACE),
                // Threshold Second settings
                text("Threshold Second Settings").size(SUBHEAD),
                container(
                    column![
                        row![
                    text("Min Health Regen Raw:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Health Regen raw...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_hpr_raw
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::HprRawChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Health Regen %:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Health Regen percentage...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_hpr_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::HprPctChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Mana Regen:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Mana Regen...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_mr
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::MrChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Life Steal:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Life Steal...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_ls
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::LsChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Mana Steal:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Mana Steal...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_ms
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::MsChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Speed:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Speed...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_spd
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::SpdChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Spell Damage Raw:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Spell Damage raw...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_sd_raw
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::SdRawChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Spell Damage %:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Spell Damage percentage...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_sd_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::SdPctChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Health Regen:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Health Regen...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_hpr
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::HprChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Experience Bonus:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter minimum Experience bonus...",
                        &self.config.threshold_second.as_ref().unwrap_or(&ThresholdSecond::default()).min_exp_bonus
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::ExpBonusChanged(input))))
                    .size(TEXT)
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                        ],
                    ]
                )
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(SPACE),
                // Threshold Third settings
                text("Threshold Third Settings").size(SUBHEAD),
                container(
                    column![
                        row![
                    text("Min Earth Defense:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min earth defense...",
                        &self.config.threshold_third.as_ref().unwrap_or(&ThresholdThird::default()).min_earth_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::EarthDefChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Thunder Defense:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min thunder defense...",
                        &self.config.threshold_third.as_ref().unwrap_or(&ThresholdThird::default()).min_thunder_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::ThunderDefChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Water Defense:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min water defense...",
                        &self.config.threshold_third.as_ref().unwrap_or(&ThresholdThird::default()).min_water_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::WaterDefChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Fire Defense:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min fire defense...",
                        &self.config.threshold_third.as_ref().unwrap_or(&ThresholdThird::default()).min_fire_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::FireDefChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Air Defense:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min air defense...",
                        &self.config.threshold_third.as_ref().unwrap_or(&ThresholdThird::default()).min_air_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::AirDefChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                    ]
                )
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(SPACE),
                // Threshold Fourth settings
                text("Threshold Fourth Settings").size(SUBHEAD),
                container(
                    column![
                        row![
                    text("Min Neutral Damage %:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min neutral damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap_or(&ThresholdFourth::default()).min_neutral_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::NeutralDamPctChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Earth Damage %:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min earth damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap_or(&ThresholdFourth::default()).min_earth_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::EarthDamPctChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Thunder Damage %:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min thunder damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap_or(&ThresholdFourth::default()).min_thunder_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::ThunderDamPctChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Water Damage %:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min water damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap_or(&ThresholdFourth::default()).min_water_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::WaterDamPctChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Fire Damage %:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min fire damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap_or(&ThresholdFourth::default()).min_fire_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::FireDamPctChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Air Damage %:").width(Length::Fixed(LABEL_WIDTH)),
                    text_input(
                        "Enter min air damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap_or(&ThresholdFourth::default()).min_air_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::AirDamPctChanged(input))))
                    .padding(INPUT_PAD)
                    .width(Length::Fill),
                ],
                    ]
                ),
                // Threshold Fifth settings
                text("Threshold Fifth Settings").size(SUBHEAD),
                container(
                    column![
                        row![
                            text("Min Earth Point:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter min earth point...",
                                &self.config.threshold_fifth.as_ref().unwrap_or(&ThresholdFifth::default()).min_earth_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::EarthPointChanged(input))))
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min Thunder Point:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter min thunder point...",
                                &self.config.threshold_fifth.as_ref().unwrap_or(&ThresholdFifth::default()).min_thunder_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::ThunderPointChanged(input))))
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min Water Point:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter min water point...",
                                &self.config.threshold_fifth.as_ref().unwrap_or(&ThresholdFifth::default()).min_water_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::WaterPointChanged(input))))
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min Fire Point:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter min fire point...",
                                &self.config.threshold_fifth.as_ref().unwrap_or(&ThresholdFifth::default()).min_fire_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::FirePointChanged(input))))
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min Air Point:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter min air point...",
                                &self.config.threshold_fifth.as_ref().unwrap_or(&ThresholdFifth::default()).min_air_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::AirPointChanged(input))))
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min EHP:").width(Length::Fixed(LABEL_WIDTH)),
                            text_input(
                                "Enter min EHP...",
                                &self.config.threshold_fifth.as_ref().unwrap_or(&ThresholdFifth::default()).min_ehp
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::EhpChanged(input))))
                            .padding(INPUT_PAD)
                            .width(Length::Fill),
                        ],
                    ]
                )
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(SPACE),
                // Gear Selection - Helmets
                column![
                    text("Helmets:"),
                    self.gear
                        .helmet_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(SPACE), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.helmets,
                                        "Select helmet...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::HelmetSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveHelmet(idx))))
                                        .padding(BTN_PAD),
                                ]
                                .spacing(SPACE)
                            )
                        }),
                    button("Add Helmet").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddHelmet))),
                ]
                .spacing(SPACE),
                // Gear Selection - Chestplates
                column![
                    text("Chestplates:"),
                    self.gear
                        .chestplate_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(SPACE), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.chestplates,
                                        "Select chestplate...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::ChestplateSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveChestplate(idx))))
                                        .padding(BTN_PAD),
                                ]
                                .spacing(SPACE)
                            )
                        }),
                    button("Add Chestplate").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddChestplate))),
                ]
                .spacing(SPACE),
                // Gear Selection - Leggings
                column![
                    text("Leggings:"),
                    self.gear
                        .leggings_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(SPACE), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.leggings,
                                        "Select leggings...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::LeggingsSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveLeggings(idx))))
                                        .padding(BTN_PAD),
                                ]
                                .spacing(SPACE)
                            )
                        }),
                    button("Add Leggings").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddLeggings))),
                ]
                .spacing(SPACE),
                // Gear Selection - Boots
                column![
                    text("Boots:"),
                    self.gear
                        .boots_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(SPACE), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.boots,
                                        "Select boots...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::BootsSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveBoots(idx))))
                                        .padding(BTN_PAD),
                                ]
                                .spacing(SPACE)
                            )
                        }),
                    button("Add Boots").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddBoots))),
                ]
                .spacing(SPACE),
                // Gear Selection - Rings
                column![
                    text("Rings:"),
                    self.gear
                        .rings_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(SPACE), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.rings,
                                        "Select ring...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::RingsSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveRings(idx))))
                                        .padding(BTN_PAD),
                                ]
                                .spacing(SPACE)
                            )
                        }),
                    button("Add Ring").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddRings))),
                ]
                .spacing(SPACE),
                // Gear Selection - Bracelets
                column![
                    text("Bracelets:"),
                    self.gear
                        .bracelets_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(SPACE), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.bracelets,
                                        "Select bracelet...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::BraceletsSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveBracelets(idx))))
                                        .padding(BTN_PAD),
                                ]
                                .spacing(SPACE)
                            )
                        }),
                    button("Add Bracelet").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddBracelets))),
                ]
                .spacing(SPACE),
                // Gear Selection - Necklaces
                column![
                    text("Necklaces:"),
                    self.gear
                        .necklaces_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(SPACE), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.necklaces,
                                        "Select necklace...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::NecklacesSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveNecklaces(idx))))
                                        .padding(BTN_PAD),
                                ]
                                .spacing(SPACE)
                            )
                        }),
                    button("Add Necklace").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddNecklaces))),
                ]
                .spacing(SPACE),
                // Gear Selection - Weapon
                column![
                    text("Weapon:"),
                    combo_box(
                        &self.gear.weapons,
                        "Select weapon...",
                        self.gear.selected_weapon.as_ref(),
                        |name| Message::Config(ConfigMessage::Gear(GearMessage::WeaponSelected(name))),
                    ),
                ]
                .spacing(SPACE),
            ]
            .spacing(SPACE)
            .align_x(Horizontal::Left);

        // Wrap the content in a container with padding before scrollable
        container(
            scrollable(
                container(content)
                    .padding(SPACE) // Add padding around the content
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Top)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
