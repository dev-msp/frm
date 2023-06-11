import { defer, filter, fromEvent, map, share } from 'rxjs';
import { browserOnly } from '$lib/stream';
import type { AppEpic } from './state/stream';

export const alpha = 'abcdefghijklmnopqrstuvwxyz0123456789';

export type UnknownCommand = {
	type: 'unmapped_keypress';
	key: string;
};

export type TraversalCommand =
	| { type: 'zoom'; amount: number }
	| { type: 'shift'; unit: 'grid' | 'second' | 'page'; amount: number }
	| { type: 'set'; from?: number; to?: number; n?: number }
	| { type: 'grid'; amount: number };

export type UiStateCommand =
	| {
			type: 'accept-context';
			totalDuration: number;
	  }
	| {
			type: 'clear-input';
	  }
	| {
			type: 'reset';
	  };

export type KeyCommand = {
	type: 'key';
	key: string;
};

export type Command = UiStateCommand | KeyCommand | TraversalCommand | UnknownCommand;

const commandMapping: Partial<Record<string, Command>> = {
	ArrowUp: { type: 'grid', amount: -1 },
	ArrowDown: { type: 'grid', amount: 1 },
	PageUp: { type: 'shift', unit: 'page', amount: -1 },
	PageDown: { type: 'shift', unit: 'page', amount: 1 },
	'(': { type: 'shift', unit: 'second', amount: -5 },
	')': { type: 'shift', unit: 'second', amount: 5 },
	'[': { type: 'shift', unit: 'grid', amount: -1 },
	']': { type: 'shift', unit: 'grid', amount: 1 },
	'{': { type: 'shift', unit: 'second', amount: -1 },
	'}': { type: 'shift', unit: 'second', amount: 1 },
	'-': { type: 'zoom', amount: -60 },
	'=': { type: 'zoom', amount: 60 },
	_: { type: 'zoom', amount: -1 },
	'+': { type: 'zoom', amount: 1 },
	Escape: { type: 'clear-input' }
};

const eventToCommand = (event: KeyboardEvent): Command => {
	const { key } = event;

	const cmd = commandMapping[key];
	if (cmd) {
		event.preventDefault();
		return cmd;
	} else if (/^[a-z0-9]$/i.test(key)) {
		return { type: 'key', key };
	}
	return {
		type: 'unmapped_keypress',
		key
	};
};

export const commands: AppEpic<Command> = () =>
	browserOnly(defer(() => fromEvent<KeyboardEvent>(document, 'keydown'))).pipe(
		filter(({ repeat }) => !repeat),
		map(eventToCommand),
		share()
	);
