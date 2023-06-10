import {
	animationFrameScheduler,
	fromEvent,
	mergeAll,
	of,
	map,
	Observable,
	Subscription,
	Subject,
	debounceTime,
	EMPTY,
	concatMap
} from 'rxjs';
import { browser } from '$app/environment';

import type { Action } from 'svelte/action';
import type { Rect, Vector } from '$lib/types/graphics';

let canvas: HTMLCanvasElement;

if (browser) {
	canvas = document.createElement('canvas');
}

const getFont = async (elem: Element): Promise<TextMetrics | undefined> => {
	const font = getComputedStyle(elem).getPropertyValue('font');
	const text = elem.textContent;
	const ctx = canvas.getContext('2d');
	if (!ctx || !text) {
		return;
	}
	ctx.font = font;
	return ctx.measureText(text);
};

type DebugPayload = {
	vec: Vector;
	nodeRect: Rect;
	glyphRect: Rect;
};

const getCenterOffset = async (node: SVGGraphicsElement): Promise<DebugPayload> => {
	const measurement = await getFont(node);
	if (!measurement) {
		throw new Error("didn't get a measurement");
	}
	const {
		fontBoundingBoxAscent,
		actualBoundingBoxDescent,
		actualBoundingBoxAscent,
		actualBoundingBoxLeft,
		actualBoundingBoxRight
	} = measurement;
	const nodebox = node.getBBox();

	const actualHeight = Math.abs(actualBoundingBoxAscent) + Math.abs(actualBoundingBoxDescent);
	const actualWidth = Math.abs(actualBoundingBoxLeft) + Math.abs(actualBoundingBoxRight);

	const ratio = nodebox.width / actualWidth;

	const fontActualYDelta = ratio * (fontBoundingBoxAscent - actualBoundingBoxAscent);

	const { x, y, width, height } = nodebox;

	return {
		vec: {
			x: x + actualWidth / 2,
			y: y + fontActualYDelta + actualHeight / 2
		},
		nodeRect: { x, y, width, height },
		glyphRect: {
			x: x,
			y: y + fontActualYDelta,
			width: actualWidth,
			height: actualHeight
		}
	};
};

export type OffsetParams = { center?: boolean; onChange?: (bb: DebugPayload) => void };
const defaultParams: OffsetParams = {};

// wait until fonts are ready
// on every mutation event (arg: node)
// on every window resize
const manual = new Subject<void>();
const resizes$ = browser ? fromEvent(window, 'resize') : EMPTY;
const refreshEvents: Observable<void> = of(manual, resizes$).pipe(
	mergeAll(),
	map(() => undefined),
	debounceTime(50, animationFrameScheduler)
);

export const offsetBoundingBox: Action<SVGGraphicsElement, OffsetParams> = (
	node,
	params = defaultParams
) => {
	const { center, onChange } = params;
	const stream = refreshEvents.pipe(concatMap(() => getCenterOffset(node)));
	let subscription: Subscription | undefined;

	if (center) {
		subscription = stream.subscribe({
			next: (debugPayload) => onChange?.(debugPayload),
			error: (err) => console.error(err)
		});
		manual.next();
	}

	return {
		update(newParams) {
			const { center, onChange: newOnChange } = newParams;
			subscription?.unsubscribe();
			if (center) {
				subscription = stream.subscribe({
					next: (x) => newOnChange?.(x),
					error: (err) => console.error(err)
				});
			}
		},
		destroy() {
			subscription?.unsubscribe();
		}
	};
};
