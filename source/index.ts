import { Jewel, jewel, stock } from "runescape";
import { Role } from "./constants.js";
import { roleMention } from "./utility.js";

interface Env {
	WEBHOOK_URL: string;
}

export default {
	async scheduled({ scheduledTime }, { WEBHOOK_URL }) {
		const date = new Date(scheduledTime);
		const contents = [];

		if (date.getUTCHours() === 0) {
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

		if (date.getUTCHours() === 20) {
			// Santa leaves 2 hours after their arrival.
			const leave = Math.floor((date.getTime() + 7_200_000) / 1_000);
			contents.push(`${roleMention(Role.Santa)} has arrived and will leave <t:${leave}:R>!`);
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
