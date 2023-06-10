export const formatTimecode = (ms: number, useHours = false) => {
	const totalSeconds = Math.floor(ms / 1000);
	const remMs = ms - totalSeconds * 1000;

	const hours = Math.floor(totalSeconds / 3600);
	const minutes = Math.floor((totalSeconds - hours * 3600) / 60);
	const seconds = totalSeconds - hours * 3600 - minutes * 60;
	const f = new Intl.NumberFormat('en-US', { minimumIntegerDigits: 2, maximumFractionDigits: 2 });
	return [hours, minutes, (seconds * 1000 + remMs) / 1000]
		.slice(useHours ? 0 : 1)
		.map((n) => f.format(n))
		.join(':');
};
