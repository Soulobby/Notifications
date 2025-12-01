mod guthixian_cache;
mod jewels;
mod travelling_merchant;
mod utility;
mod wilderness_flash_events;
use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Timelike, Utc, Weekday};
use dotenvy::dotenv;
use serenity::{
    all::{CreateMessage, Mentionable},
    http::Http,
};
use std::{env, time::Duration};
use tokio::{spawn, time::sleep};
use utility::constants::{
    APMEKEN_AMETHYST, CLAWDIA, GUTHIXIAN_CACHE, HAPPY_HOUR, MENAPHITE_GIFTS,
    NOTIFICATION_CHANNEL_ID, SANTA, SCABARITE_CRYSTAL, SNOWVERLOAD, WILDERNESS_FLASH_EVENT_SPECIAL,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN").context("Error retrieving DISCORD_TOKEN.")?;
    let client = Http::new(&discord_token);

    let notifier = spawn(async move {
        if let Err(error) = notify(client).await {
            eprintln!("Error in notifying: {:?}", error);
        }
    });

    tokio::join!(notifier).0?;
    Ok(())
}

fn jewel_content(date: DateTime<Utc>) -> Option<String> {
    let content = match jewels::jewel(date) {
        Some(jewels::Jewels::ApmekenAmethyst) => {
            format!("The {} is accessible today.", APMEKEN_AMETHYST.mention())
        }
        Some(jewels::Jewels::ScabariteCrystal) => {
            format!("The {} is accessible today.", SCABARITE_CRYSTAL.mention())
        }
        _ => return None,
    };

    Some(content.to_string())
}

fn travelling_merchant_content(date: DateTime<Utc>) -> Option<String> {
    for item in travelling_merchant::stock(date) {
        match item {
            travelling_merchant::Item::MenaphiteGiftOfferingLarge
            | travelling_merchant::Item::MenaphiteGiftOfferingMedium
            | travelling_merchant::Item::MenaphiteGiftOfferingSmall => {
                return Some(format!(
                    "The Travelling Merchant has {} in stock today!",
                    MENAPHITE_GIFTS.mention()
                ));
            }
            _ => continue,
        }
    }
    None
}

fn guthixian_cache_content(date: DateTime<Utc>) -> Option<String> {
    if guthixian_cache::guthixian_cache(date) {
        Some(format!(
            "A {} will open <t:{}:R> with full rewards!",
            GUTHIXIAN_CACHE.mention(),
            date.timestamp()
        ))
    } else {
        None
    }
}

fn wilderness_flash_event_content(date: DateTime<Utc>) -> Option<String> {
    let content = match wilderness_flash_events::wilderness_flash_event(date) {
        wilderness_flash_events::WildernessFlashEvent::KingBlackDragonRampage => format!(
            "{} The King Black Dragon will rampage <t:{}:R>!",
            WILDERNESS_FLASH_EVENT_SPECIAL.mention(),
            date.timestamp()
        ),
        wilderness_flash_events::WildernessFlashEvent::InfernalStar => format!(
            "{} An infernal star will land <t:{}:R>!",
            WILDERNESS_FLASH_EVENT_SPECIAL.mention(),
            date.timestamp()
        ),
        wilderness_flash_events::WildernessFlashEvent::EvilBloodwoodTree => format!(
            "{} An evil bloodwood tree will grow <t:{}:R>!",
            WILDERNESS_FLASH_EVENT_SPECIAL.mention(),
            date.timestamp()
        ),
        wilderness_flash_events::WildernessFlashEvent::StrykeTheWyrm => format!(
            "{} The WildyWyrm will burrow to the surface <t:{}:R>!",
            WILDERNESS_FLASH_EVENT_SPECIAL.mention(),
            date.timestamp()
        ),
        _ => return None,
    };

    Some(content.to_string())
}

async fn notify(client: Http) -> Result<()> {
    loop {
        sleep(Duration::from_millis(
            60000 - (Utc::now().timestamp_millis() % 60000) as u64,
        ))
        .await;

        let now = Utc::now();
        let mut content = vec![];

        if now.hour() == 0 && now.minute() == 0 {
            let jewel_content = jewel_content(now);
            let travelling_merchant_content = travelling_merchant_content(now);

            if let Some(event_content) = jewel_content {
                content.push(event_content);
            }

            if let Some(event_content) = travelling_merchant_content {
                content.push(event_content);
            }
        }

        if now.year() == 2025
            && ((now.month() == 6 && now.day() >= 30 && now.hour() >= 10)
                || (now.month() == 7 && now.day() < 28))
            && now.minute() == 40
        {
            let clawdia_timestamp_start = now + Duration::from_secs(300);

            content.push(format!(
                "{} spawns <t:{}:R>!",
                CLAWDIA.mention(),
                clawdia_timestamp_start.timestamp(),
            ));
        }

        if now.year() == 2025
            && ((now.month() == 6 && now.day() >= 30 && now.hour() >= 10)
                || (now.month() == 7 && now.day() < 28))
            && ((now.hour() == 0 && now.minute() == 55)
                || (now.hour() == 8 && now.minute() == 55)
                || (now.hour() == 13 && now.minute() == 55)
                || (now.hour() == 16 && now.minute() == 25)
                || (now.hour() == 20 && now.minute() == 55))
        {
            let happy_hourtimestamp_start = now + Duration::from_secs(300);

            content.push(format!(
                "{} starts <t:{}:R>!",
                HAPPY_HOUR.mention(),
                happy_hourtimestamp_start.timestamp(),
            ));
        }

        if now.weekday() == Weekday::Sun
            && now.hour() == 19
            && now.minute() == 50
            && ((now.year() == 2024 && now.month() == 12)
                || ((now.year() == 2025
                    && now.month() == 12
                    && (now.day() > 1 || (now.day() == 1 && now.hour() >= 11)))
                    || (now.year() == 2026 && now.month() == 1 && now.day() < 5)))
        {
            let santa_timestamp_start = now + Duration::from_secs(600);

            content.push(format!(
                "{} will arrive <t:{}:R>!",
                SANTA.mention(),
                santa_timestamp_start.timestamp(),
            ));
        }

        if now.minute() == 40
            && ((now.year() == 2025
                && now.month() == 12
                && (now.day() > 1 || (now.day() == 1 && now.hour() >= 11)))
                || (now.year() == 2026 && now.month() == 1 && now.day() < 5))
        {
            let snowverload_timestamp_start = now + Duration::from_secs(300);

            content.push(format!(
                "{} spawns <t:{}:R>!",
                SNOWVERLOAD.mention(),
                snowverload_timestamp_start.timestamp(),
            ));
        }

        if now.minute() == 55 {
            let date = now + Duration::from_secs(300);
            let guthixian_cache_content = guthixian_cache_content(date);

            if let Some(event_content) = guthixian_cache_content {
                content.push(event_content);
            }

            let wilderness_flash_event_content = wilderness_flash_event_content(date);

            if let Some(event_content) = wilderness_flash_event_content {
                content.push(event_content);
            }
        }

        for event in content {
            client
                .send_message(
                    NOTIFICATION_CHANNEL_ID,
                    vec![],
                    &CreateMessage::new().content(event),
                )
                .await?;
        }
    }
}
