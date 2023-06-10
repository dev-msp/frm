import { Observable, Subject, Subscription, map, shareReplay, startWith } from 'rxjs';

import { labeledLog } from '$lib/stream';
import { identity, isNil } from 'lodash';
import { runEpicDefault, type Epic, type RunEpicConfig } from './stream';
import type { Reducer } from './reducer';

export class Store<S, A extends { type: string }> {
	private state: S;
	private reducer: Reducer<S, A>;

	private state$: Observable<S>;
	private actions$ = new Subject<A>();
	private rootEpicSubscription?: Subscription;

	constructor(reducer: Reducer<S, A>, initialState: S) {
		this.state = initialState;
		this.state$ = this.actions$.pipe(
			startWith(undefined),
			map(() => this.getState())
		);
		this.reducer = reducer;
	}

	getState(): S {
		return this.state;
	}

	private setState(s: S) {
		this.state = s;
	}

	dispatch(a: A & { type: string }) {
		// console.group(`Dispatching action ${a.type}`);
		// console.log('Current state', JSON.stringify(this.state, null, 2));
		// console.log('Action', JSON.stringify(a, null, 2));
		this.setState(this.reducer(this.state, a));
		this.actions$.next(a);
		// console.log('New state', JSON.stringify(this.state, null, 2));
		// console.groupEnd();
	}

	consumeEpic(epic: Epic<S, A, A>) {
		if (this.rootEpicSubscription) {
			throw new Error('expected only a single root epic');
		}
		this.rootEpicSubscription = epic(
			this.actions$.pipe(labeledLog('all actions', (s) => JSON.stringify(s, null, 2))),
			this.state$.pipe(labeledLog('all state', (s) => JSON.stringify(s, null, 2)))
		).subscribe((action) => {
			console.log('going to dispatch', JSON.stringify(action, null, 2));
			this.dispatch(action);
		});
	}

	runEpic<T>(
		epic: Epic<S, A, T>,
		initial?: T,
		config: RunEpicConfig = runEpicDefault
	): Observable<T> {
		const source = epic(this.actions$.asObservable(), this.state$).pipe(
			isNil(initial) ? identity : startWith(initial)
		);
		return config.share
			? source.pipe(
					config.label ? labeledLog(config.label) : identity,
					shareReplay(config.replayCount ?? 0)
			  )
			: source;
	}
}
