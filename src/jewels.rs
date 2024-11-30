use crate::utility::functions::{next_int, runedate};
use chrono::{DateTime, Utc};

pub enum Jewels {
    ApmekenAmethyst,
    ScabariteCrystal,
}

pub fn jewel(date: DateTime<Utc>) -> Option<Jewels> {
    let slot = next_int((runedate(date) as u128) * 2u128.pow(32), 5, 1);

    println!("{}", slot);
    match slot {
        0 => Some(Jewels::ScabariteCrystal),
        2 => Some(Jewels::ApmekenAmethyst),
        _ => None,
    }
}
