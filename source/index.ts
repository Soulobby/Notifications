import { setInterval } from "node:timers";
import { Jewel, /* WildernessFlashEvent,*/ jewel, stock /* wildernessFlashEvent */ } from "runescape";
import { request } from "undici";
import { DISCORD_TOKEN, NOTIFICATION_CHANNEL_ID, Role } from "./constants.js";
import { roleMention } from "./utility.js";

if (!DISCORD_TOKEN) throw new Error("No Discord token provided.");
if (!NOTIFICATION_CHANNEL_ID) throw new Error("No notification channel id provided.");

setInterval(async () => {
	const date = new Date();
	const hours = date.getUTCHours();
	const minutes = date.getUTCMinutes();
	const seconds = date.getUTCSeconds();
	const contents = [];
	if (seconds !== 0) return;

	// if (minutes === 55) {
	// 	const nextWildernessFlashEvent = wildernessFlashEvent(1);
	// 	let content = `${roleMention(Role.WildernessFlashEventSpecial)} `;

	// 	switch (nextWildernessFlashEvent) {
	// 		case WildernessFlashEvent.KingBlackDragonRampage:
	// 			content += "The King Black Dragon will rampage";
	// 			break;
	// 		case WildernessFlashEvent.InfernalStar:
	// 			content += "An infernal star will land";
	// 			break;
	// 		case WildernessFlashEvent.EvilBloodwoodTree:
	// 			content += "An evil bloodwood tree will grow";
	// 			break;
	// 		default:
	// 			return;
	// 	}

	// 	contents.push(`${content} <t:${Math.floor((date.getTime() + 300_000) / 1_000)}:R>!`);
	// }

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
			contents.push(`The Travelling Merchant has ${roleMention(Role.MenaphiteGifts)} in stock today!`);
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
