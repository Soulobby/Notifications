use chrono::{DateTime, Utc};

const INITIAL_TIMESTAMP: i64 = 1707134400;

pub enum WildernessFlashEvent {
    KingBlackDragonRampage,
    ForgottenSoldiers,
    SurprisingSeedlings,
    HellhoundPack,
    InfernalStar,
    LostSouls,
    RamokeeIncursion,
    DisplacedEnergy,
    EvilBloodwoodTree,
    SpiderSwarm,
    UnnaturalOutcrop,
    StrykeTheWyrm,
    DemonStragglers,
    ButterflySwarm,
}

pub fn wilderness_flash_event(date: DateTime<Utc>) -> WildernessFlashEvent {
    let timestamp = date.timestamp();

    if timestamp < INITIAL_TIMESTAMP {
        panic!("Date is before events began.");
    }

    let hours_elapsed = (timestamp - INITIAL_TIMESTAMP) / 3600;

    match hours_elapsed % 14 {
        0 => WildernessFlashEvent::KingBlackDragonRampage,
        1 => WildernessFlashEvent::ForgottenSoldiers,
        2 => WildernessFlashEvent::SurprisingSeedlings,
        3 => WildernessFlashEvent::HellhoundPack,
        4 => WildernessFlashEvent::InfernalStar,
        5 => WildernessFlashEvent::LostSouls,
        6 => WildernessFlashEvent::RamokeeIncursion,
        7 => WildernessFlashEvent::DisplacedEnergy,
        8 => WildernessFlashEvent::EvilBloodwoodTree,
        9 => WildernessFlashEvent::SpiderSwarm,
        10 => WildernessFlashEvent::UnnaturalOutcrop,
        11 => WildernessFlashEvent::StrykeTheWyrm,
        12 => WildernessFlashEvent::DemonStragglers,
        13 => WildernessFlashEvent::ButterflySwarm,
        _ => unreachable!(),
    }
}
