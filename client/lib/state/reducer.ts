import { mapObject } from '../iteration';
import { identity } from '../function';

export type Val<T extends Record<string | number | symbol, any>> = T[keyof T];

export type JsonData = null | boolean | number | string | JsonData[] | { [k: string]: JsonData };

export type JsonCompatible<T> = {
	[P in keyof T]: T[P] extends JsonData
		? T[P]
		: Pick<T, P> extends Required<Pick<T, P>>
		? never
		: T[P] extends (() => any) | undefined
		? never
		: JsonCompatible<T[P]>;
};

export type Message<D extends JsonData = JsonData> = {
	sender: string;
	data: D;
};

export interface TypedResponse<T = any> extends Response {
	json<P = T>(): Promise<P>;
}

// Reducers

export type PartReducer<S, A extends Action, F> = (state: S, action: A, fullState: F) => S;

export type Reducer<S, A extends Action> = (state: S, action: A) => S;

export interface Action {
	type: string;
}

export const reducePartByType = <
	State,
	AM extends Record<ActionType, Action>,
	FullState = State,
	ActionType extends string | symbol = keyof AM & (string | symbol)
>(
	typeMapping: Partial<{ [key in ActionType]: PartReducer<State, AM[key], FullState> }>,
	defaultReducer: Reducer<State, AM[ActionType]> = identity
) => {
	return (state: State, action: AM[ActionType], fullState: FullState): State =>
		typeMapping?.[action.type as ActionType]?.(state, action, fullState) ??
		defaultReducer(state, action);
};

export const reduceByType = <
	State,
	AM extends Record<ActionType, Action>,
	ActionType extends string | symbol = keyof AM & (string | symbol)
>(
	typeMapping: Partial<{ [key in ActionType]: Reducer<State, AM[key]> }>,
	defaultReducer: Reducer<State, AM[ActionType]> = identity
) => {
	return (state: State, action: AM[ActionType]): State =>
		typeMapping?.[action.type as ActionType]?.(state, action) ?? defaultReducer(state, action);
};

export const reduceByKey = <
	State extends Record<string | symbol, any>,
	AM extends Record<any, Action>
>(keyMapping: { [key in keyof State]: PartReducer<State[key], Val<AM>, State> }) => {
	return (fullState: State, action: Val<AM>): State =>
		mapObject(
			fullState,
			(keyState: State[keyof State], key: keyof State) =>
				keyMapping?.[key]?.(keyState, action, fullState) ?? keyState
		) as State;
};
