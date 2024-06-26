import { setInterval } from "node:timers";
import {
	Jewel,
	WildernessFlashEvent,
	jewel,
	guthixianCache as rGuthixianCache,
	wildernessFlashEvent as rWildernessFlashEvent,
	stock,
} from "runescape";
import { request } from "undici";
import { DISCORD_TOKEN, NOTIFICATION_CHANNEL_ID, Role } from "./constants.js";
import { roleMention } from "./utility.js";

if (!DISCORD_TOKEN) {
	throw new Error("No Discord token provided.");
}

if (!NOTIFICATION_CHANNEL_ID) {
	throw new Error("No notification channel id provided.");
}

function guthixianCache(contents: string[], timestamp: string) {
	if (rGuthixianCache(1)) {
		contents.push(
			`A ${roleMention(Role.GuthixianCache)} will open ${timestamp} with full rewards!`,
		);
	}

	return contents;
}

function wildernessFlashEvent(contents: string[], timestamp: string) {
	const nextWildernessFlashEvent = rWildernessFlashEvent(1);
	let wildernessFlashEventContent = `${roleMention(Role.WildernessFlashEventSpecial)} `;

	switch (nextWildernessFlashEvent) {
		case WildernessFlashEvent.KingBlackDragonRampage:
			wildernessFlashEventContent += "The King Black Dragon will rampage";
			break;
		case WildernessFlashEvent.InfernalStar:
			wildernessFlashEventContent += "An infernal star will land";
			break;
		case WildernessFlashEvent.EvilBloodwoodTree:
			wildernessFlashEventContent += "An evil bloodwood tree will grow";
			break;
		case WildernessFlashEvent.StrykeTheWyrm:
			wildernessFlashEventContent += "The WildyWyrm will burrow to the surface";
			break;
		default:
			return contents;
	}

	contents.push(`${wildernessFlashEventContent} ${timestamp}!`);
	return contents;
}

setInterval(async () => {
	const date = new Date();
	const hours = date.getUTCHours();
	const minutes = date.getUTCMinutes();
	const seconds = date.getUTCSeconds();
	let contents: string[] = [];

	if (seconds !== 0) {
		return;
	}

	if (minutes === 55) {
		// In 5 minutes.
		const timestamp = `<t:${Math.floor((date.getTime() + 300_000) / 1_000)}:R>`;

		contents = guthixianCache(contents, timestamp);
		contents = wildernessFlashEvent(contents, timestamp);
	}

	if (hours === 0 && minutes === 0) {
		const currentJewel = jewel();

		if (currentJewel) {
			contents.push(
				`The ${roleMention(
					currentJewel === Jewel.ApmekenAmethyst ? Role.ApmekenAmethyst : Role.ScabariteCrystal,
				)} is accessible today.`,
			);
		}

		if (stock().some((slot) => slot.includes("Menaphite"))) {
			contents.push(
				`The Travelling Merchant has ${roleMention(Role.MenaphiteGifts)} in stock today!`,
			);
		}
	}

	for (const content of contents) {
		await request(`https://discord.com/api/v10/channels/${NOTIFICATION_CHANNEL_ID}/messages`, {
			headers: { authorization: `Bot ${DISCORD_TOKEN}`, "Content-Type": "application/json" },
			method: "POST",
			body: JSON.stringify({ allowed_mentions: { parse: ["roles"] }, content }),
		});
	}
}, 1_000);
