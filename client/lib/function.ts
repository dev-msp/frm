export const identity = <T>(x: T): T => x;

export function isFunction(functionToCheck: unknown): functionToCheck is CallableFunction {
	return !!functionToCheck && {}.toString.call(functionToCheck) === '[object Function]';
}

export const noop = (): void => undefined;

export const always =
	<T>(arg: T): (() => T) =>
	() =>
		arg;
