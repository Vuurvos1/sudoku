export enum NoteStyles {
	Grid,
	Row
}

export const settings = $state({
	noteStyle: NoteStyles.Grid,
	showTimer: false
});
