import { error } from '@sveltejs/kit';
import { techniques } from '#lib/learn.ts';
import type { EntryGenerator, PageLoad } from './$types';

export const entries: EntryGenerator = () => techniques.map(({ slug }) => ({ slug }));

export const load: PageLoad = ({ params }) => {
	const technique = techniques.find((t) => t.slug === params.slug);
	if (!technique) error(404, 'Unknown technique');
	return { technique };
};
