import { Jewel, jewel, stock } from "runescape";
import { CHRISTMAS_EVENT_END_TIMESTAMP, Role } from "./constants.js";
import { roleMention } from "./utility.js";

interface Env {
	WEBHOOK_URL: string;
}

export default {
	async scheduled({ cron, scheduledTime }, { WEBHOOK_URL }) {
		const date = new Date(scheduledTime);
		const unix = date.getTime();
		const contents = [];

		if (cron === "0 0 * * *") {
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

		if (cron === "25 19 * * 1" && unix < CHRISTMAS_EVENT_END_TIMESTAMP) {
			// Presents start in 5 minutes.
			const presents = Math.floor((unix + 300_000) / 1_000);

			// Santa arrives in 35 minutes.
			const santa = Math.floor((unix + 3_300_000) / 1_000);

			contents.push(`Collect presents <t:${presents}:R>! ${roleMention(Role.Santa)} will arrive <t:${santa}:R>!`);
		}

		for (const content of contents) {
			await fetch(WEBHOOK_URL, {
				headers: { "Content-Type": "application/json" },
				method: "POST",
				body: JSON.stringify({ allowed_mentions: { parse: ["roles"] }, content }),
			});
		}
	},
} satisfies ExportedHandler<Env>;
