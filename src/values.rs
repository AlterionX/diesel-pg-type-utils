use std::{ops::{Add, AddAssign, Sub, SubAssign}, str::FromStr, cmp::Ordering};

use diesel::{
    pg::Pg,
    AsExpression,
    FromSqlRow,
    sql_types::{
        Text,
        Integer,
    },
};

use crate::{
    wrap::{
        impl_sql_convert,
        wrap_u64,
        wrap_u32,
        wrap_i32,
    },
    error::LevelError,
};

type DB = Pg;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum InventoryScope {
    Bot,
    All,
}

impl FromStr for InventoryScope {
    type Err = (); // TODO Figure out GAT FromStr
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.to_lowercase();
        match normalized.as_str() {
            "bot" => Ok(InventoryScope::Bot),
            "all" => Ok(InventoryScope::All),
            _ => Err(()),
        }
    }
}

impl InventoryScope {
    // TODO Impl correct trait.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bot => "bot",
            Self::All => "all",
        }
    }
}

impl_sql_convert!(
    <DB>
    Text > String > InventoryScope
    |s| {
        InventoryScope::from_str(s.as_str())
            .ok().ok_or("bad value")?
    }
    |scope| {
        &scope.as_str().to_owned()
    }
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum TransactionPurpose {
    Withdrawal,
    Deposit,
    Trade,
    ClaimBounced,
}

impl TransactionPurpose {
    // TODO Impl correct trait.
    pub fn as_str(self) -> &'static str {
        match self {
            TransactionPurpose::Deposit => "deposit",
            TransactionPurpose::Withdrawal => "withdrawal",
            TransactionPurpose::Trade => "trade",
            TransactionPurpose::ClaimBounced => "claimbounced",
        }
    }

    // TODO Impl correct trait.
    fn from_str(s: &str) -> Option<Self> {
        let normalized = s.to_lowercase();
        match normalized.as_str() {
            "deposit" => Some(TransactionPurpose::Deposit),
            "withdrawal" => Some(TransactionPurpose::Withdrawal),
            "trade" => Some(TransactionPurpose::Trade),
            "claimbounced" => Some(TransactionPurpose::ClaimBounced),
            _ => None,
        }
    }
}

impl_sql_convert!(
    <DB>
    Text > String > TransactionPurpose
    |s| {
        TransactionPurpose::from_str(s.as_str())
            .ok_or("bad config")?
    }
    |tier| {
        &tier.as_str().to_owned()
    }
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum SoloExpeditionRewardTier {
    Bad,
    Okay,
    Good,
    Bonus,
}

impl PartialOrd for SoloExpeditionRewardTier {
    fn partial_cmp(&self, other: &SoloExpeditionRewardTier) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SoloExpeditionRewardTier {
    fn cmp(&self, other: &SoloExpeditionRewardTier) -> Ordering {
        match (self, other) {
            (SoloExpeditionRewardTier::Bad, SoloExpeditionRewardTier::Bad) => Ordering::Equal,
            (SoloExpeditionRewardTier::Bad, SoloExpeditionRewardTier::Okay) => Ordering::Less,
            (SoloExpeditionRewardTier::Bad, SoloExpeditionRewardTier::Good) => Ordering::Less,
            (SoloExpeditionRewardTier::Bad, SoloExpeditionRewardTier::Bonus) => Ordering::Less,

            (SoloExpeditionRewardTier::Okay, SoloExpeditionRewardTier::Bad) => Ordering::Greater,
            (SoloExpeditionRewardTier::Okay, SoloExpeditionRewardTier::Okay) => Ordering::Equal,
            (SoloExpeditionRewardTier::Okay, SoloExpeditionRewardTier::Good) => Ordering::Less,
            (SoloExpeditionRewardTier::Okay, SoloExpeditionRewardTier::Bonus) => Ordering::Less,

            (SoloExpeditionRewardTier::Good, SoloExpeditionRewardTier::Bad) => Ordering::Greater,
            (SoloExpeditionRewardTier::Good, SoloExpeditionRewardTier::Okay) => Ordering::Greater,
            (SoloExpeditionRewardTier::Good, SoloExpeditionRewardTier::Good) => Ordering::Equal,
            (SoloExpeditionRewardTier::Good, SoloExpeditionRewardTier::Bonus) => Ordering::Less,

            (SoloExpeditionRewardTier::Bonus, SoloExpeditionRewardTier::Bad) => Ordering::Greater,
            (SoloExpeditionRewardTier::Bonus, SoloExpeditionRewardTier::Okay) => Ordering::Greater,
            (SoloExpeditionRewardTier::Bonus, SoloExpeditionRewardTier::Good) => Ordering::Greater,
            (SoloExpeditionRewardTier::Bonus, SoloExpeditionRewardTier::Bonus) => Ordering::Equal,
        }
    }
}

impl SoloExpeditionRewardTier {
    // TODO Impl correct trait.
    pub fn as_str(self) -> &'static str {
        match self {
            SoloExpeditionRewardTier::Bad => "bad",
            SoloExpeditionRewardTier::Okay => "okay",
            SoloExpeditionRewardTier::Good => "good",
            SoloExpeditionRewardTier::Bonus => "bonus",
        }
    }

    // TODO Impl correct trait.
    fn from_str(s: &str) -> Option<Self> {
        let normalized = s.to_lowercase();
        if normalized.starts_with("bad") {
            Some(SoloExpeditionRewardTier::Bad)
        } else if normalized.starts_with("okay") {
            Some(SoloExpeditionRewardTier::Okay)
        } else if normalized.starts_with("good") {
            Some(SoloExpeditionRewardTier::Good)
        } else if normalized.starts_with("bonus") {
            Some(SoloExpeditionRewardTier::Bonus)
        } else {
            None
        }
    }
}

impl_sql_convert!(
    <DB>
    Text > String > SoloExpeditionRewardTier
    |s| {
        SoloExpeditionRewardTier::from_str(s.as_str())
            .ok_or("bad config")?
    }
    |tier| {
        &tier.as_str().to_owned()
    }
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum ShopLimitBoundary {
    Daily,
    Weekly,
    Never,
}

impl FromStr for ShopLimitBoundary {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "never" => Ok(Self::Never),
            "weekly" => Ok(Self::Weekly),
            "daily" => Ok(Self::Daily),
            _ => Err(()),
        }
    }
}

impl ShopLimitBoundary {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::Weekly => "weekly",
            Self::Daily => "daily",
        }
    }
}

impl_sql_convert!(
    <DB>
    Text > String > ShopLimitBoundary
    |s| {
        ShopLimitBoundary::from_str(s.as_str())
            .map_err(|()| "bad shop limit boundary")?
    }
    |tier| {
        &tier.to_str().to_owned()
    }
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "n" | "north" => Ok(Self::North),
            "2" | "e" | "east" | "east " => Ok(Self::East),
            "3" | "s" | "south" => Ok(Self::South),
            "4" | "w" | "west" | "west " => Ok(Self::West),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::North => "north",
            Self::East => "east",
            Self::South => "south",
            Self::West => "west",
        }
    }
}

impl_sql_convert!(
    <DB>
    Text > String > Direction
    |s| {
        Direction::from_str(s.as_str())
            .map_err(|()| "bad config")?
    }
    |tier| {
        &tier.to_str().to_owned()
    }
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum UserJourneyKind {
    AdminGrantItem,

    ExpeditionsSolo,
    ExpeditionsCheckSolo,
    ExpeditionsWander,
    ExpeditionsWeekly,

    ExplorationAtlas,
    ExplorationDestinations,
    ExplorationLocation,
    ExplorationMap,
    ExplorationTravel,
    ExplorationTripPlan,

    GeneralClassInfo,
    GeneralImage,
    GeneralInfo,
    GeneralHelp,
    GeneralPing,
    GeneralRandom,

    ItemInfo,
    ItemUse,

    ProfileDefaultSet,
    ProfileDefaultGet,
    ProfileWalletClear,
    ProfileWalletGet,
    ProfileWalletSet,
    ProfileCooldown,
    ProfileInventory,
    ProfileParty,
    ProfilePreferenceSet,

    ProtagonistStatusCheck,
    ProtagonistClass, // Merged get info & update
    ProtagonistEquip,
    ProtagonistDisplay,
    ProtagonistInfo,
    ProtagonistNameSet, // Also reset
    ProtagonistRoll,
    ProtagonistUnequip,
    ProtagonistUnequipAll,

    StatisticsLevels,
    StatisticsPopulations,

    TrainingTrain,

    TransactionClaimBounced,
    TransactionDeposit,
    TransactionWithdraw,
    TransactionSend,
    TransactionTrade,

    ShopShop,
    ShopBuy,

    OregonTrailMinigame,
}

impl FromStr for UserJourneyKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "grantitem" => Ok(Self::AdminGrantItem),

            "solo" => Ok(Self::ExpeditionsSolo),
            "checksolo" => Ok(Self::ExpeditionsCheckSolo),
            "wander" => Ok(Self::ExpeditionsWander),
            "weekly" => Ok(Self::ExpeditionsWeekly),

            "atlas" => Ok(Self::ExplorationAtlas),
            "destinations" => Ok(Self::ExplorationDestinations),
            "location" => Ok(Self::ExplorationLocation),
            "map" => Ok(Self::ExplorationMap),
            "travel" => Ok(Self::ExplorationTravel),
            "tripplan" => Ok(Self::ExplorationTripPlan),

            "classinfo" => Ok(Self::GeneralClassInfo),
            "image" => Ok(Self::GeneralImage),
            "info" => Ok(Self::GeneralInfo),
            "help" => Ok(Self::GeneralHelp),
            "ping" => Ok(Self::GeneralPing),
            "random" => Ok(Self::GeneralRandom),

            "iteminfo" => Ok(Self::ItemInfo),
            "itemuse" => Ok(Self::ItemUse),

            "setdefault" => Ok(Self::ProfileDefaultSet),
            "getdefault" => Ok(Self::ProfileDefaultGet),
            "clearwallet" => Ok(Self::ProfileWalletClear),
            "getwallet" => Ok(Self::ProfileWalletGet),
            "setwallet" => Ok(Self::ProfileWalletSet),
            "cooldown" => Ok(Self::ProfileCooldown),
            "inventory" => Ok(Self::ProfileInventory),
            "party" => Ok(Self::ProfileParty),
            "setpreference" => Ok(Self::ProfilePreferenceSet),

            "status" => Ok(Self::ProtagonistStatusCheck),
            "class" => Ok(Self::ProtagonistClass), // Merged get info & update
            "equip" => Ok(Self::ProtagonistEquip),
            "protagonist" => Ok(Self::ProtagonistDisplay),
            "protaginfo" => Ok(Self::ProtagonistInfo),
            "rename" => Ok(Self::ProtagonistNameSet), // Also reset
            "roll" => Ok(Self::ProtagonistRoll),
            "unequip" => Ok(Self::ProtagonistUnequip),
            "unequipall" => Ok(Self::ProtagonistUnequipAll),

            "levels" => Ok(Self::StatisticsLevels),
            "population" => Ok(Self::StatisticsPopulations),

            "train" => Ok(Self::TrainingTrain),

            "claimbounced" => Ok(Self::TransactionClaimBounced),
            "deposit" => Ok(Self::TransactionDeposit),
            "withdraw" => Ok(Self::TransactionWithdraw),
            "send" => Ok(Self::TransactionSend),
            "trade" => Ok(Self::TransactionTrade),

            "shop" => Ok(Self::ShopShop),
            "buy" => Ok(Self::ShopBuy),

            "oregon" => Ok(Self::OregonTrailMinigame),

            _ => Err(()),
        }
    }
}

impl UserJourneyKind {
    pub fn from_command_str(s: &str) -> Option<Self> {
        match s {
            "grantitem" => Some(Self::AdminGrantItem),

            "solo" => Some(Self::ExpeditionsSolo),
            "checksolo" => Some(Self::ExpeditionsCheckSolo),
            "wander" => Some(Self::ExpeditionsWander),
            "weekly" => Some(Self::ExpeditionsWeekly),

            "atlas" => Some(Self::ExplorationAtlas),
            "destinations" => Some(Self::ExplorationDestinations),
            "location" => Some(Self::ExplorationLocation),
            "map" => Some(Self::ExplorationMap),
            "travel" => Some(Self::ExplorationTravel),
            "tripplan" => Some(Self::ExplorationTripPlan),

            "classinfo" => Some(Self::GeneralClassInfo),
            "image" => Some(Self::GeneralImage),
            "info" => Some(Self::GeneralInfo),
            "help" => Some(Self::GeneralHelp),
            "ping" => Some(Self::GeneralPing),
            "random" => Some(Self::GeneralRandom),

            "iteminfo" => Some(Self::ItemInfo),
            "useitem"
            | "use"
            | "itemuse" => Some(Self::ItemUse),

            "setdefault" => Some(Self::ProfileDefaultSet),
            "getdefault" => Some(Self::ProfileDefaultGet),
            "clearwallet"
            | "clearw"
            | "clearaddr"
            | "clearaddress" => Some(Self::ProfileWalletClear),
            "getwallet" => Some(Self::ProfileWalletGet),
            "setwallet" => Some(Self::ProfileWalletSet),
            "cd"
            | "cooldown" => Some(Self::ProfileCooldown),
            "inventory"
            | "inv" => Some(Self::ProfileInventory),
            "party" => Some(Self::ProfileParty),
            "setpreference" => Some(Self::ProfilePreferenceSet),

            "status"
            | "checkstatus" => Some(Self::ProtagonistStatusCheck),
            "class" => Some(Self::ProtagonistClass), // Merged get info & update
            "equip" => Some(Self::ProtagonistEquip),
            "protagonist"
            | "protag" => Some(Self::ProtagonistDisplay),
            "protaginfo" => Some(Self::ProtagonistInfo),
            "rename" => Some(Self::ProtagonistNameSet), // Also reset
            "roll" => Some(Self::ProtagonistRoll),
            "unequip" => Some(Self::ProtagonistUnequip),
            "unequipall" => Some(Self::ProtagonistUnequipAll),

            "levels" => Some(Self::StatisticsLevels),
            "population" => Some(Self::StatisticsPopulations),

            "trainall"
            | "train" => Some(Self::TrainingTrain),

            "claimbounced" => Some(Self::TransactionClaimBounced),
            "deposit" => Some(Self::TransactionDeposit),
            "withdraw" => Some(Self::TransactionWithdraw),
            "send" => Some(Self::TransactionSend),
            "trade" => Some(Self::TransactionTrade),

            "shop" => Some(Self::ShopShop),
            "buy" => Some(Self::ShopBuy),

            "oregon" => Some(Self::OregonTrailMinigame),

            _ => None,
        }
    }

    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::AdminGrantItem => "grantitem",

            Self::ExpeditionsSolo => "solo",
            Self::ExpeditionsCheckSolo => "checksolo",
            Self::ExpeditionsWander => "wander",
            Self::ExpeditionsWeekly => "weekly",

            Self::ExplorationAtlas => "atlas",
            Self::ExplorationDestinations => "destinations",
            Self::ExplorationLocation => "location",
            Self::ExplorationMap => "map",
            Self::ExplorationTravel => "travel",
            Self::ExplorationTripPlan => "tripplan",

            Self::GeneralClassInfo => "classinfo",
            Self::GeneralImage => "image",
            Self::GeneralInfo => "info",
            Self::GeneralHelp => "help",
            Self::GeneralPing => "ping",
            Self::GeneralRandom => "random",

            Self::ItemInfo => "iteminfo",
            Self::ItemUse => "itemuse",

            Self::ProfileDefaultSet => "setdefault",
            Self::ProfileDefaultGet => "getdefault",
            Self::ProfileWalletClear => "clearwallet",
            Self::ProfileWalletGet => "getwallet",
            Self::ProfileWalletSet => "setwallet",
            Self::ProfileCooldown => "cooldown",
            Self::ProfileInventory => "inventory",
            Self::ProfileParty => "party",
            Self::ProfilePreferenceSet => "setpreference",

            Self::ProtagonistStatusCheck => "status",
            Self::ProtagonistClass => "class",
            Self::ProtagonistEquip => "equip",
            Self::ProtagonistDisplay => "protagonist",
            Self::ProtagonistInfo => "protaginfo",
            Self::ProtagonistNameSet => "rename",
            Self::ProtagonistRoll => "roll",
            Self::ProtagonistUnequip => "unequip",
            Self::ProtagonistUnequipAll => "unequipall",

            Self::StatisticsLevels => "levels",
            Self::StatisticsPopulations => "population",

            Self::TrainingTrain => "train",

            Self::TransactionClaimBounced => "claimbounced",
            Self::TransactionDeposit => "deposit",
            Self::TransactionSend => "send",
            Self::TransactionTrade => "trade",
            Self::TransactionWithdraw => "withdraw",

            Self::ShopShop => "shop",
            Self::ShopBuy => "buy",

            Self::OregonTrailMinigame => "oregon",
        }
    }

    pub const fn str_forms() -> [&'static str; 58] {
        [
            "grantitem",

            "solo",
            "checksolo",
            "wander",
            "weekly",

            "atlas",
            "destinations",
            "location",
            "map",
            "travel",
            "tripplan",

            "classinfo",
            "image",
            "info",
            "help",
            "ping",
            "random",

            "iteminfo",
            "useitem",
            "use",
            "itemuse",

            "setdefault",
            "getdefault",
            "clearwallet",
            "clearw",
            "clearaddr",
            "clearaddress",
            "getwallet",
            "setwallet",
            "cd",
            "cooldown",
            "inventory",
            "inv",
            "party",
            "setpreference",

            "status",
            "checkstatus",
            "class",
            "equip",
            "protagonist",
            "protag",
            "protaginfo",
            "rename",
            "roll",
            "unequip",
            "unequipall",

            "levels",
            "population",

            "trainall",
            "train",

            "claimbounced",
            "deposit",
            "withdraw",
            "send",
            "trade",

            "shop",
            "buy",

            "oregon",
        ]
    }

    pub const fn is_atomic_command(&self) -> bool {
        match self {
            Self::AdminGrantItem => true,

            Self::ExpeditionsSolo => false,
            Self::ExpeditionsCheckSolo => true,
            Self::ExpeditionsWander => true,
            Self::ExpeditionsWeekly => true,

            Self::ExplorationAtlas => true,
            Self::ExplorationDestinations => true,
            Self::ExplorationLocation => true,
            Self::ExplorationMap => true,
            Self::ExplorationTravel => false,
            Self::ExplorationTripPlan => true,

            Self::GeneralClassInfo => true,
            Self::GeneralImage => true,
            Self::GeneralInfo => true,
            Self::GeneralHelp => true,
            Self::GeneralPing => true,
            Self::GeneralRandom => true,

            Self::ItemInfo => true,
            Self::ItemUse => false,

            Self::ProfileDefaultSet => true,
            Self::ProfileDefaultGet => true,
            Self::ProfileWalletClear => true,
            Self::ProfileWalletGet => true,
            Self::ProfileWalletSet => true,
            Self::ProfileCooldown => true,
            Self::ProfileInventory => true,
            Self::ProfileParty => true,
            Self::ProfilePreferenceSet => true,

            Self::ProtagonistStatusCheck => true,
            Self::ProtagonistClass => false,
            Self::ProtagonistEquip => true,
            Self::ProtagonistDisplay => true,
            Self::ProtagonistInfo => true,
            Self::ProtagonistNameSet => true,
            Self::ProtagonistRoll => true,
            Self::ProtagonistUnequip => true,
            Self::ProtagonistUnequipAll => true,

            Self::StatisticsLevels => true,
            Self::StatisticsPopulations => true,

            Self::TrainingTrain => false,

            Self::TransactionTrade => false,
            Self::TransactionSend => false,
            Self::TransactionWithdraw => true,
            Self::TransactionDeposit => false,
            Self::TransactionClaimBounced => true,

            Self::ShopShop => true,
            Self::ShopBuy => false,

            Self::OregonTrailMinigame => false,
        }
    }
}

impl_sql_convert!(
    <DB>
    Text > String > UserJourneyKind
    |s| {
        UserJourneyKind::from_str(s.as_str())
            .map_err(|()| "bad config")?
    }
    |tier| {
        &tier.to_str().to_owned()
    }
);

wrap_u32!(XP<DB>);
impl XP {
    pub fn inner_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}
impl From<u32> for XP {
    fn from(xp: u32) -> Self {
        Self(xp)
    }
}
// TODO Should this be a u32?
wrap_i32!(Level<DB>);
impl Level {
    pub fn new(level: i32) -> Result<Self, LevelError> {
        if level < 0 {
            // TODO should this be a panic?
            return Err(LevelError::Negative);
        }

        Ok(Level(level))
    }
}

wrap_i32! {
    #[derive(Default)]
    SlotCapacity<DB>
}
impl From<i32> for SlotCapacity {
    fn from(capacity: i32) -> Self {
        SlotCapacity(capacity)
    }
}
impl SlotCapacity {
    pub fn inner_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}
impl SubAssign for SlotCapacity {
    fn sub_assign(&mut self, other: Self) {
        *self.inner_mut() -= *other.inner();
    }
}
impl Sub for SlotCapacity {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // implicit copy
        let mut store = self;
        store -= other;
        store
    }
}
impl Add for SlotCapacity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // implicit copy
        let mut store = self;
        store += other;
        store
    }
}
impl AddAssign for SlotCapacity {
    fn add_assign(&mut self, other: Self) {
        *self.inner_mut() += *other.inner();
    }
}

wrap_u32!(DecimalPlaces<DB>);
impl From<u32> for DecimalPlaces {
    fn from(capacity: u32) -> Self {
        Self(capacity)
    }
}
wrap_u64!(ItemAmount<DB>);
impl ItemAmount {
    pub fn inner_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}
impl From<u64> for ItemAmount {
    fn from(amount: u64) -> Self {
        Self(amount)
    }
}
impl SubAssign for ItemAmount {
    fn sub_assign(&mut self, other: Self) {
        *self.inner_mut() -= *other.inner();
    }
}
impl Sub for ItemAmount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Implicit copy
        let mut store = self;
        store -= other;
        store
    }
}
impl Add for ItemAmount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Implicit copy
        let mut store = self;
        store += other;
        store
    }
}
impl AddAssign for ItemAmount {
    fn add_assign(&mut self, other: Self) {
        *self.inner_mut() += *other.inner();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Integer)]
pub enum UIKind {
    Message,
    Button,
    Select,
}

impl UIKind {
    fn from_i32(i: i32) -> Option<Self> {
        match i {
            0 => Some(Self::Message),
            1 => Some(Self::Button),
            2 => Some(Self::Select),
            _ => None,
        }
    }

    fn to_i32(self) -> i32 {
        match self {
            Self::Message => 0,
            Self::Button => 1,
            Self::Select => 2,
        }
    }
}

impl_sql_convert!(
    <DB>
    Integer > i32 > UIKind
    |i| {
        UIKind::from_i32(i).ok_or("bad config")?
    }
    |kind| {
        &kind.to_i32()
    }
);
