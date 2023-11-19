import { Jewel, jewel, stock } from "runescape";
import { Role } from "./constants.js";
import { roleMention } from "./utility.js";

interface Env {
	WEBHOOK_URL: string;
}

export default {
	async fetch() {
		return new Response("", { status: 444, statusText: "Soul drained." });
	},
	async scheduled(_, { WEBHOOK_URL }) {
		const currentJewel = jewel();
		const contents = [];

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

		for (const content of contents) {
			await fetch(WEBHOOK_URL, {
				headers: { "Content-Type": "application/json" },
				method: "POST",
				body: JSON.stringify({ allowed_mentions: { parse: ["roles"] }, content }),
			});
		}
	},
} satisfies ExportedHandler<Env>;
