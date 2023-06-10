import type { Command, KeyCommand } from '$lib/command';
import type { Range } from './range';
import type { Reducer } from './reducer';

export type Bisect = {
	direction: 'forward' | 'backward' | 'initial';
	original: Range;
	current: Range;
};

export type BisectCommand = { type: 'bisect'; action: 'left' | 'right' | 'commit' | 'reset' };

export const initial = (duration: number): Bisect => {
	const rng: Range = { from: 0, to: duration, totalDuration: duration };
	return {
		direction: 'initial',
		original: rng,
		current: rng
	};
};

const translateCmd = (cmd: KeyCommand): BisectCommand | undefined => {
	switch (cmd.key) {
		case 'h':
			return { type: 'bisect', action: 'left' };
		case 'l':
			return { type: 'bisect', action: 'right' };
		case 'r':
			return { type: 'bisect', action: 'reset' };
		case 'space':
			return { type: 'bisect', action: 'left' };
	}
};

export const reducer: Reducer<Bisect, Command> = (s, a): Bisect => {
	const cmd = a.type === 'key' ? translateCmd(a) : undefined;
	if (!cmd) {
		console.log('bisect rejected', cmd);
		return s;
	}

	const { current } = s;
	const midpoint = Math.round(current.from + (current.to - current.from) / 2);
	switch (cmd.action) {
		case 'left': {
			return {
				...s,
				direction: 'backward',
				current: {
					...current,
					from: current.from,
					to: midpoint
				}
			};
		}

		case 'right': {
			return {
				...s,
				direction: 'forward',
				current: {
					...current,
					from: midpoint,
					to: current.to
				}
			};
		}

		case 'reset': {
			return {
				...s,
				direction: 'initial',
				current: s.original
			};
		}

		default:
			return s;
	}
};
