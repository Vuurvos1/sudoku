function round(num: number) {
	return Math.round(num * 100) / 100;
}

export function hexToHSL(hex: string) {
	const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);

	if (!result) {
		return null;
	}

	let r = parseInt(result[1], 16);
	let g = parseInt(result[2], 16);
	let b = parseInt(result[3], 16);
	((r /= 255), (g /= 255), (b /= 255));

	const max = Math.max(r, g, b);
	const min = Math.min(r, g, b);

	let h;
	let s;
	const l = (max + min) / 2;

	if (max == min) {
		h = s = 0; // achromatic
	} else {
		const d = max - min;
		s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
		switch (max) {
			case r:
				h = (g - b) / d + (g < b ? 6 : 0);
				break;
			case g:
				h = (b - r) / d + 2;
				break;
			case b:
				h = (r - g) / d + 4;
				break;
		}
		// @ts-expect-error - h is assigned in the switch statement
		h /= 6;
	}

	// TODO: round h, s, l to 2 decimal places
	const HSL = {
		// @ts-expect-error - h will always be assigned
		h: round(h * 360),
		s: round(s * 100),
		l: round(l * 100)
	};
	return HSL;
}
