use chrono::{DateTime, Datelike, Timelike, Utc};
use std::time::Duration;

const INITIAL_RUNEDATE_TIMESTAMP: u64 = 1014768000;
const MULTI: u128 = 0x5deece66d;
const MASK: u128 = 281_474_976_710_656;
const C0: u128 = 0xe66d;
const C1: u128 = 0xdeec;
const C2: u128 = 0x0005;
const CHUNK: u128 = 65536;

pub fn runedate(date: DateTime<Utc>) -> f64 {
    ((((date - Duration::from_secs(INITIAL_RUNEDATE_TIMESTAMP)).timestamp() as f64) / 86400.0)
        * 100.0)
        .floor()
        / 100.0
}

fn multiply_avoid_limit(seed: u128) -> u128 {
    let s0 = seed % CHUNK;
    let s1 = (seed / CHUNK) % CHUNK;
    let s2 = seed / (CHUNK * CHUNK);
    let mut carry = 11;
    let mut r0 = s0 * C0 + carry;
    carry = r0 / CHUNK;
    r0 %= CHUNK;
    let mut r1 = s1 * C0 + s0 * C1 + carry;
    carry = r1 / CHUNK;
    r1 %= CHUNK;
    let mut r2 = s2 * C0 + s1 * C1 + s0 * C2 + carry;
    r2 %= CHUNK;
    r2 * CHUNK * CHUNK + r1 * CHUNK + r0
}

pub fn next_int(seed: u128, no: u128, repeats: u32) -> u128 {
    let mut computed_seed = (seed ^ MULTI) % MASK;

    for _ in 0..repeats {
        computed_seed = multiply_avoid_limit(computed_seed);
    }

    computed_seed >>= 17;
    computed_seed % no
}

pub fn is_christmas_2025_event(now: DateTime<Utc>) -> bool {
    (now.year() == 2025
        && now.month() == 12
        && (now.day() > 1
            || (now.day() == 1 && now.hour() > 11)
            || (now.day() == 1 && now.hour() == 11 && now.minute() >= 15)))
        || (now.year() == 2026 && now.month() == 1 && now.day() < 5)
}
