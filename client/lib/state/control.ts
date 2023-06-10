import { isConflict, type Key } from './input';
import type { Reducer } from './reducer';

type Claim =
	| { type: 'keys'; solo: boolean; resources: Key[] }
	| { type: 'group'; solo: boolean; claims: Claim[] };

export type ControlContext = {
	stack: Claim[];
	error: { message: string }[];
};

type Command = { type: 'push_claim'; payload: Claim } | { type: 'pop_claim' };

export const reducer: Reducer<ControlContext, Command> = (s, a) => {
	switch (a.type) {
		case 'push_claim': {
			if (canStack(merge(s.stack), a.payload)) {
				return { ...s, stack: [...s.stack, a.payload] };
			}
			return { ...s, error: [...s.error, { message: "couldn't accept new claim" }] };
		}
		case 'pop_claim': {
			if (s.stack.length > 1) {
				return { ...s, stack: s.stack.slice(0, -1) };
			}
			return { ...s, error: [...s.error, { message: "can't remove last control claim" }] };
		}
		default:
			return s;
	}
};

const merge = (claims: Claim[]): Claim => {
	const singles: (Claim & { type: 'keys' })[] = [];
	const groups: (Claim & { type: 'group' })[] = [];

	for (const claim of claims) {
		if (claim.type === 'keys') {
			singles.push(claim);
		} else {
			groups.push(claim);
		}
	}

	const mergedGroupClaims = groups.flatMap((group) => group.claims);
	const allClaims = [...mergedGroupClaims, ...singles];

	return {
		type: 'group',
		solo: allClaims.some(({ solo }) => solo),
		claims: allClaims
	};
};

const canStack = (base: Claim, newClaim: Claim): boolean => {
	if (newClaim.solo) return true;
	if (base.type === 'keys' && newClaim.type === 'keys') {
		return base.resources.every((aKey) =>
			newClaim.resources.every((bKey) => !isConflict(aKey, bKey))
		);
	} else if (base.type === 'group' && newClaim.type === 'group') {
		return base.claims.every((aGroup) =>
			newClaim.claims.every((bGroup) => canStack(aGroup, bGroup))
		);
	} else {
		const keyClaim = (base.type === 'keys' ? base : newClaim) as Claim & { type: 'key' };
		const groupClaim = (base.type === 'group' ? base : newClaim) as Claim & { type: 'group' };
		return groupClaim.claims.every((group) => canStack(keyClaim, group));
	}
};
