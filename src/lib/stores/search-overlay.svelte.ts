let _open = $state(false);

export const searchOverlay = {
	get open() {
		return _open;
	},
	show() {
		_open = true;
	},
	toggle() {
		_open = !_open;
	},
	close() {
		_open = false;
	},
};
