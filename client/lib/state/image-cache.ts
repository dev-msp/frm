import { sortBy, sortedUniq } from 'lodash';
import {
	Subject,
	type Observer,
	type Unsubscribable,
	Observable,
	filter,
	map,
	distinctUntilChanged,
	defer,
	fromEvent,
	of,
	mergeMap,
	concat,
	EMPTY,
	take,
	bufferTime,
	concatMap
} from 'rxjs';

export type ImageState = { status: 'loading' } | { status: 'error' } | { status: 'ready' };

export type ImageEvent =
	| { type: 'request'; timestamp: number }
	| { type: 'loaded'; timestamp: number }
	| { type: 'error'; timestamp: number }
	| { type: 'substitute'; timestamp: number; substitution: number };

class ImageCache extends Map<number, ImageState> {
	readonly subject: Subject<ImageEvent> = new Subject();

	subscribe(observer: Observer<ImageEvent>): Unsubscribable {
		return this.subject.subscribe(observer);
	}

	run() {
		this.subject
			.pipe(
				filter((event): event is ImageEvent & { type: 'request' } => event.type === 'request'),
				bufferTime(1000),
				concatMap((xs) =>
					sortedUniq(
						sortBy(xs, ({ timestamp }) => {
							const sub = this.substituteFor(timestamp);
							return Math.abs(sub ? sub - timestamp : 0);
						})
					)
				),
				mergeMap(({ timestamp }) => {
					const image = new Image();
					image.src = `/api/frame/${timestamp}`;
					image.loading = 'eager';
					return fromEvent(image, 'load').pipe(
						take(1),
						map(() => timestamp)
					);
				}, 10)
			)
			.subscribe({
				next: (timestamp) => {
					this.markReady(timestamp);
					this.subject.next({ type: 'loaded', timestamp });
				},
				error: (e) => {
					console.error(e);
				}
			});
	}

	get$(timestamp: number): Observable<number | undefined> {
		const kickItOff: Observable<ImageEvent> = defer(() => {
			if (this.get(timestamp)?.status === 'ready') {
				return of({ type: 'loaded', timestamp } as ImageEvent);
			}
			const sub = this.substituteFor(timestamp);
			this.subject.next({ type: 'request', timestamp });
			return concat(
				sub ? of({ type: 'substitute', timestamp, substitution: sub } as ImageEvent) : EMPTY,
				this.subject
			);
		});

		return kickItOff.pipe(
			filter((e) => e.timestamp === timestamp),
			map((e) =>
				e.type === 'substitute' ? e.substitution : e.type === 'loaded' ? e.timestamp : undefined
			),
			distinctUntilChanged()
		);
	}

	set(timestamp: number, image: ImageState) {
		super.set(timestamp, image);
		if (image.status === 'ready') {
			this.subject.next({ type: 'loaded', timestamp });
		} else if (image.status === 'error') {
			this.subject.next({ type: 'error', timestamp });
		}
		return this;
	}

	markReady(key: number) {
		this.set(key, { status: 'ready' });
	}

	private substituteFor(key: number): number | undefined {
		let sub: number | undefined;
		for (const [i, { status }] of super.entries()) {
			const delta = Math.abs(key - i);
			if (status !== 'ready' || delta > 90e3) {
				continue;
			}
			if (delta < Math.abs(key - (sub ?? 0))) {
				sub = i;
			}
		}
		return sub;
	}
}

export const imageCache = new ImageCache();
imageCache.run();
