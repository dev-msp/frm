export type Key = {
	key: KeyLoc;
	modifier: Modifier[];
};

type KeyLoc =
	| {
			type: 'alphanumeric' | 'symbol';
			char: string;
	  }
	| { type: 'command'; name: CommandKey };

enum CommandKey {
	ArrowUp,
	ArrowDown,
	ArrowLeft,
	ArrowRight,
	Return,
	Tab,
	CapsLock,
	Space
}

enum Modifier {
	Fn,
	Shift,
	LeftShift,
	RightShift,
	Command,
	LeftCommand,
	RightCommand,
	Option,
	LeftOption,
	RightOption,
	Control,
	LeftControl,
	RightControl
}

export const isConflict = (keyA: Key, keyB: Key): boolean => {
	if (keyA.key.type !== keyB.key.type) {
		return false;
	}
	if (keyA.key.type === 'alphanumeric' && keyB.key.type === 'alphanumeric') {
		return keyA.key.char.toLowerCase() === keyB.key.char.toLowerCase();
	} else if (keyA.key.type === 'command' && keyB.key.type === 'command') {
		return keyA.key.name === keyB.key.name;
	} else {
		throw new Error('impossible?!');
	}
};

export const getKeyValue = (event: KeyboardEvent): Key => {
	const { key, shiftKey, ctrlKey, altKey, metaKey } = event;
	const modifierKeys: Modifier[] = [];

	if (shiftKey) {
		modifierKeys.push(Modifier.Shift);
	}
	if (ctrlKey) {
		modifierKeys.push(Modifier.Control);
	}
	if (altKey) {
		modifierKeys.push(Modifier.Option);
	}
	if (metaKey) {
		modifierKeys.push(Modifier.Command);
	}

	if (key.length === 1) {
		const keyLoc: KeyLoc = {
			type: /[a-zA-Z0-9]/.test(key) ? 'alphanumeric' : 'symbol',
			char: key
		};

		return { key: keyLoc, modifier: modifierKeys };
	}

	switch (key) {
		case 'ArrowUp':
			return {
				key: { type: 'command', name: CommandKey.ArrowUp },
				modifier: modifierKeys
			};
		case 'ArrowDown':
			return {
				key: { type: 'command', name: CommandKey.ArrowDown },
				modifier: modifierKeys
			};
		case 'ArrowLeft':
			return {
				key: { type: 'command', name: CommandKey.ArrowLeft },
				modifier: modifierKeys
			};
		case 'ArrowRight':
			return {
				key: { type: 'command', name: CommandKey.ArrowRight },
				modifier: modifierKeys
			};
		case 'Return':
			return {
				key: { type: 'command', name: CommandKey.Return },
				modifier: modifierKeys
			};
		case 'Tab':
			return { key: { type: 'command', name: CommandKey.Tab }, modifier: modifierKeys };

		case 'Space':
			return {
				key: { type: 'command', name: CommandKey.Space },
				modifier: modifierKeys
			};

		case 'CapsLock':
			return {
				key: { type: 'command', name: CommandKey.CapsLock },
				modifier: modifierKeys
			};
		default:
			throw new Error(`Unhandled key: ${key}`);
	}
};

export const formatKey = (key: Key): string => {
	const keyModifiers = key.modifier.join(' + ');
	const keyName = getKeyDisplayName(key.key);

	if (keyModifiers.length === 0) {
		return keyName;
	} else {
		return `${keyModifiers} + ${keyName}`;
	}
};

function getKeyDisplayName(key: KeyLoc): string {
	switch (key.type) {
		case 'alphanumeric':
			return key.char;
		case 'symbol':
			return key.char;
		case 'command':
			return CommandKey[key.name];
	}
}
