import { error } from '@sveltejs/kit';
import { rules } from '$lib/learn';
import type { EntryGenerator, PageLoad } from './$types';

export const entries: EntryGenerator = () => rules.map(({ slug }) => ({ slug }));

export const load: PageLoad = ({ params }) => {
	const rule = rules.find((r) => r.slug === params.slug);
	if (!rule) error(404, 'Unknown rule');
	return { rule };
};
