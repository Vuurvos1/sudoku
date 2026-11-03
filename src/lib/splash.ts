// the launch splash lives in app.html, so it shows before the app has loaded

/** where app.html finds the theme's colors, to color the splash before the settings are in */
const SPLASH_THEME_KEY = 'splash-theme';

let hidden = false;

/**
 * Takes down the launch splash. With a target, the splash logo moves onto it while the
 * background fades, so the logo lands in its place on the page; otherwise it all fades.
 */
export function hideSplash(target: HTMLElement | null, { animate = true } = {}) {
	const splash = document.getElementById('splash');
	if (hidden || !splash) return;
	hidden = true;

	if (!animate) {
		splash.remove();
		return;
	}

	const options = {
		duration: 500,
		easing: 'cubic-bezier(0.2, 0, 0, 1)',
		fill: 'forwards'
	} as const;
	const logo = splash.querySelector('svg')!;
	// the name stays behind and fades as the logo sets off
	splash.querySelector('p')?.animate({ opacity: 0 }, { ...options, duration: 150 });

	if (target) {
		const from = logo.getBoundingClientRect();
		const to = target.getBoundingClientRect();
		const move = `translate(${to.left - from.left}px, ${to.top - from.top}px)`;
		logo.animate({ transform: `${move} scale(${to.width / from.width})` }, options);
		// the page's own logo stays hidden until the splash logo is on top of it
		target.style.visibility = 'hidden';
	} else {
		logo.animate({ opacity: 0 }, options);
	}

	// the page shows a beat after the logo sets off, so the logo doesn't cross the title
	const reveal = { ...options, delay: 150, duration: 350 };
	splash.animate({ backgroundColor: 'transparent' }, reveal).finished.then(() => {
		splash.remove();
		if (target) target.style.visibility = '';
	});
}

/** remembers the theme's colors for the next launch's splash */
export function saveSplashTheme({ background, text }: { background: string; text: string }) {
	try {
		localStorage.setItem(SPLASH_THEME_KEY, JSON.stringify({ background, text }));
	} catch {
		// the splash falls back to the dark theme
	}
}
