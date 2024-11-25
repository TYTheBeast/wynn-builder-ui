use std::fmt::{self, Display};

use crate::{builder::BuilderProgress, Tab};
use iced::Theme;
use iced_table::table::Column;
use iced_widget::{scrollable::AbsoluteOffset, text_editor::Action};

#[derive(Debug, Clone)]
pub enum SearchMessage {
    InputChanged(String),
    InputSubmitted,
    ItemEditorAction(Action),
}

#[derive(Debug, Clone)]
pub enum GearMessage {
    HelmetSelected(usize, String),
    AddHelmet,
    ChestplateSelected(usize, String),
    AddChestplate,
    LeggingsSelected(usize, String),
    AddLeggings,
    BootsSelected(usize, String),
    AddBoots,
    RingsSelected(usize, String),
    AddRings,
    BraceletsSelected(usize, String),
    AddBracelets,
    NecklacesSelected(usize, String),
    AddNecklaces,
    WeaponSelected(String),
    RemoveHelmet(usize),
    RemoveChestplate(usize),
    RemoveLeggings(usize),
    RemoveBoots(usize),
    RemoveRings(usize),
    RemoveBracelets(usize),
    RemoveNecklaces(usize),
}

#[derive(Debug, Clone)]
pub enum PlayerMessage {
    LevelChanged(String),
    AvailablePointChanged(String),
    BaseHpChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdFirstMessage {
    HpChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdSecondMessage {
    HprRawChanged(String),
    HprPctChanged(String),
    MrChanged(String),
    LsChanged(String),
    MsChanged(String),
    SpdChanged(String),
    SdRawChanged(String),
    SdPctChanged(String),
    HprChanged(String),
    ExpBonusChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdThirdMessage {
    EarthDefChanged(String),
    ThunderDefChanged(String),
    WaterDefChanged(String),
    FireDefChanged(String),
    AirDefChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdFourthMessage {
    NeutralDamPctChanged(String),
    EarthDamPctChanged(String),
    ThunderDamPctChanged(String),
    WaterDamPctChanged(String),
    FireDamPctChanged(String),
    AirDamPctChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdFifthMessage {
    EarthPointChanged(String),
    ThunderPointChanged(String),
    WaterPointChanged(String),
    FirePointChanged(String),
    AirPointChanged(String),
    EhpChanged(String),
}

#[derive(Debug, Clone)]
pub enum HppengMessage {
    UrlPrefixChanged(String),
    UrlSuffixChanged(String),
    DbPathChanged(String),
    MigrationsPathChanged(String),
    ItemsFileChanged(String),
    LogBuildsChanged(bool),
    LogDbErrorsChanged(bool),
    DbRetryCountChanged(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    ThemeChanged(Theme),
    Search(SearchMessage),
    Config(ConfigMessage),
    Builder(BuilderMessage),
    DBReader(DBReaderMessage),
}

#[derive(Debug, Clone)]
pub enum DBReaderMessage {
    Sync(AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
    ResizeColumnsEnabled(bool),
    FooterEnabled(bool),
    MinWidthEnabled(bool),
    Notes(usize, String),
    Theme(Theme),
    Category(usize, Category),
    Enabled(usize, bool),
    Delete(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Category {
    #[default]
    A,
    B,
    C,
    D,
    E,
}

impl Category {
    pub const ALL: &'static [Self] = &[Self::A, Self::B, Self::C, Self::D, Self::E];
}

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::A => "A",
            Category::B => "B",
            Category::C => "C",
            Category::D => "D",
            Category::E => "E",
        }
        .fmt(f)
    }
}

#[derive(Default)]
pub enum ColumnKind {
    #[default]
    Index,
    Category,
    Enabled,
    Notes,
    Delete,
}

#[derive(Debug, Clone)]
pub enum BuilderMessage {
    Communication(Result<BuilderProgress, String>),
    StartBinary,
    StopBinary,
    SetOutputLines(i32),
}

#[derive(Debug, Clone)]
pub enum ConfigMessage {
    Gear(GearMessage),
    Player(PlayerMessage),
    ThresholdFirst(ThresholdFirstMessage),
    ThresholdSecond(ThresholdSecondMessage),
    ThresholdThird(ThresholdThirdMessage),
    ThresholdFourth(ThresholdFourthMessage),
    ThresholdFifth(ThresholdFifthMessage),
    Hppeng(HppengMessage),
}
