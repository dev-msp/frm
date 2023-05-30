import { filter, fromEvent, map, share } from 'rxjs';

export const alpha = 'abcdefghijklmnopqrstuvwxyz';

export type UnknownCommand = {
	type: 'unknown_keypress';
	key: string;
};

export type TraversalCommand =
	| { type: 'zoom'; amount: number }
	| { type: 'shift'; amount: number }
	| { type: 'set'; from?: number; to?: number; n?: number };

export type UiStateCommand = {
	type: 'clear-input';
};

export type AlphaCommand = {
	type: 'alpha';
	key: string;
};

export type DataEvent = {
	type: 'data';
	payload: [number, number][];
};

export type Command = UiStateCommand | AlphaCommand | TraversalCommand | DataEvent | UnknownCommand;

const commandMapping: Partial<Record<string, Command>> = {
	'(': { type: 'shift', amount: -5 },
	')': { type: 'shift', amount: 5 },
	'[': { type: 'shift', amount: -5 },
	']': { type: 'shift', amount: 5 },
	'{': { type: 'shift', amount: -1 },
	'}': { type: 'shift', amount: 1 },
	'-': { type: 'zoom', amount: -0.3 },
	'=': { type: 'zoom', amount: 0.3 },
	Escape: { type: 'clear-input' }
};

const eventToCommand = ({ key }: { key: string }): Command =>
	commandMapping[key] ??
	(/^[a-z]$/i.test(key) ? { type: 'alpha', key } : undefined) ?? {
		type: 'unknown_keypress',
		key
	};

export const commands = fromEvent<KeyboardEvent>(document, 'keydown').pipe(
	filter(({ repeat }) => !repeat),
	map(eventToCommand),
	share()
);

export const alphas = commands.pipe(filter((c): c is AlphaCommand => c.type === 'alpha'));
export const simpleCommands = commands.pipe(
	filter((c): c is TraversalCommand => ['zoom', 'shift', 'set'].includes(c.type))
);
