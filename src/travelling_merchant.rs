use chrono::{DateTime, Utc};

use crate::utility::utility::{next_int, runedate};

#[derive(Clone)]
pub enum Item {
    AdvancedPulseCore,
    AnimaCrystal,
    BarrelOfBait,
    BrokenFishingRod,
    CrystalTriskelion,
    DDTokenDaily,
    DDTokenMonthly,
    DDTokenWeekly,
    DeathtouchedDart,
    DragonkinLamp,
    DungeoneeringWildcard,
    GiftForTheReaper,
    GoebieBurialCharm,
    HarmonicDust,
    HornOfHonour,
    LargeGoebieBurialCharm,
    LividPlant,
    MenaphiteGiftOfferingLarge,
    MenaphiteGiftOfferingMedium,
    MenaphiteGiftOfferingSmall,
    MessageInABottle,
    SacredClay,
    ShatteredAnima,
    SilverhawkDown,
    SlayerVIPCoupon,
    SmallGoebieBurialCharm,
    StarvedAncientEffigy,
    Taijitu,
    TangledFishbowl,
    UnfocusedDamageEnhancer,
    UnfocusedRewardEnhancer,
    UnstableAirRune,
}

const SLOT_1_AND_2: [Item; 19] = [
    Item::GiftForTheReaper,
    Item::BrokenFishingRod,
    Item::BarrelOfBait,
    Item::AnimaCrystal,
    Item::SmallGoebieBurialCharm,
    Item::GoebieBurialCharm,
    Item::MenaphiteGiftOfferingSmall,
    Item::MenaphiteGiftOfferingMedium,
    Item::ShatteredAnima,
    Item::DDTokenDaily,
    Item::SacredClay,
    Item::LividPlant,
    Item::SlayerVIPCoupon,
    Item::SilverhawkDown,
    Item::UnstableAirRune,
    Item::AdvancedPulseCore,
    Item::TangledFishbowl,
    Item::UnfocusedDamageEnhancer,
    Item::HornOfHonour,
];

const SLOT_3: [Item; 13] = [
    Item::Taijitu,
    Item::LargeGoebieBurialCharm,
    Item::MenaphiteGiftOfferingLarge,
    Item::DDTokenWeekly,
    Item::DDTokenMonthly,
    Item::DungeoneeringWildcard,
    Item::MessageInABottle,
    Item::CrystalTriskelion,
    Item::StarvedAncientEffigy,
    Item::DeathtouchedDart,
    Item::DragonkinLamp,
    Item::HarmonicDust,
    Item::UnfocusedRewardEnhancer,
];

fn get_slots(runedate: f64, n1: u64, n2: u128) -> u128 {
    let seed = (runedate as u128) * 2u128.pow(32) + (runedate as u64 % n1) as u128;
    next_int(seed, n2, 1)
}

pub fn stock(date: DateTime<Utc>) -> [Item; 3] {
    let runedate = runedate(date);

    let slot_1 = get_slots(runedate, 3, 19) as usize;
    let slot_2 = get_slots(runedate, 8, 19) as usize;
    let slot_3 = get_slots(runedate, 5, 13) as usize;

    let item_1 = SLOT_1_AND_2.get(slot_1).unwrap_or_else(|| {
        panic!("Index out of bounds: {}", slot_1);
    });

    let item_2 = SLOT_1_AND_2.get(slot_2).unwrap_or_else(|| {
        panic!("Index out of bounds: {}", slot_2);
    });

    let item_3 = SLOT_3.get(slot_3).unwrap_or_else(|| {
        panic!("Index out of bounds: {}", slot_3);
    });

    [item_1.clone(), item_2.clone(), item_3.clone()]
}
