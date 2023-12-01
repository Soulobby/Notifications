export const Role = {
	ApmekenAmethyst: "583288933773869066",
	ScabariteCrystal: "583289583911960589",
	MenaphiteGifts: "615200253087449091",
	Santa: "1180053189807067186",
} as const satisfies Readonly<Record<string, `${bigint}`>>;

export const CHRISTMAS_EVENT_END_TIMESTAMP = Date.UTC(2_024, 0, 8);
