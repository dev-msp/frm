// type JsonValue = JsonValue[] | { [key: string]: JsonValue } | string | number | boolean | null;

export const getRange = async (
	{ from = 0, to }: Partial<{ from: number; to: number }> = {},
	n?: number
): Promise<{ codes: [number, number][]; end: string; start: string }> => {
	let path = `/api/from/${from}`;

	if (!isNaN(to ?? NaN)) {
		path += `/to/${to}`;
	}

	const params = new URLSearchParams({ n: (n || 9).toLocaleString() });
	path = `${path}?${params}`;

	const response = await fetch(path, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});
	return response.json() as Promise<{ codes: [number, number][]; end: string; start: string }>;
};
