mod jewels;
mod travelling_merchant;
mod utility;
mod wilderness_flash_events;

use anyhow::{Context, Result};
use chrono::{DateTime, Timelike, Utc};
use dotenvy::dotenv;
use serenity::{all::CreateMessage, http::Http};
use std::{env, time::Duration};
use tokio::{spawn, time::interval};
use utility::constants::{
    APMEKEN_AMETHYST, GUTHIXIAN_CACHE, MENAPHITE_GIFTS, NOTIFICATION_CHANNEL_ID, SCABARITE_CRYSTAL,
    WILDERNESS_FLASH_EVENT_SPECIAL,
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
            format!("The <@&{}> is accessible today.", APMEKEN_AMETHYST)
        }
        Some(jewels::Jewels::ScabariteCrystal) => {
            format!("The <@&{}> is accessible today.", SCABARITE_CRYSTAL)
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
                    "The Travelling Merchant has <@&{}> in stock today!",
                    MENAPHITE_GIFTS
                ));
            }
            _ => continue,
        }
    }
    None
}

fn wilderness_flash_event_content(date: DateTime<Utc>) -> Option<String> {
    let content = match wilderness_flash_events::wilderness_flash_event(date) {
        wilderness_flash_events::WildernessFlashEvent::KingBlackDragonRampage => format!(
            "<@&{}> The King Black Dragon will rampage <t:{}:R>!",
            WILDERNESS_FLASH_EVENT_SPECIAL,
            date.timestamp()
        ),
        wilderness_flash_events::WildernessFlashEvent::InfernalStar => format!(
            "<@&{}> An infernal star will land <t:{}:R>!",
            WILDERNESS_FLASH_EVENT_SPECIAL,
            date.timestamp()
        ),
        wilderness_flash_events::WildernessFlashEvent::EvilBloodwoodTree => format!(
            "<@&{}> An evil bloodwood tree will grow <t:{}:R>!",
            WILDERNESS_FLASH_EVENT_SPECIAL,
            date.timestamp()
        ),
        wilderness_flash_events::WildernessFlashEvent::StrykeTheWyrm => format!(
            "<@&{}> The WildyWyrm will burrow to the surface <t:{}:R>!",
            WILDERNESS_FLASH_EVENT_SPECIAL,
            date.timestamp()
        ),
        _ => return None,
    };

    Some(content.to_string())
}

async fn notify(client: Http) -> Result<()> {
    let mut interval = interval(Duration::from_secs(1));

    loop {
        interval.tick().await;
        let now = Utc::now();

        if now.second() != 0 {
            continue;
        }

        let mut content = vec![];

        if now.minute() == 0 {
            let jewel_content = jewel_content(now);
            let travelling_merchant_content = travelling_merchant_content(now);

            if let Some(event_content) = jewel_content {
                content.push(event_content);
            }

            if let Some(event_content) = travelling_merchant_content {
                content.push(event_content);
            }
        }

        if now.minute() == 55 {
            let date = now.clone() + Duration::from_secs(300);

            let guthixian_cache_content = format!(
                "A <@&{}> will open <t:{}:R> with full rewards!",
                GUTHIXIAN_CACHE,
                date.timestamp()
            );

            let wilderness_flash_event_content = wilderness_flash_event_content(date);
            content.push(guthixian_cache_content);

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
