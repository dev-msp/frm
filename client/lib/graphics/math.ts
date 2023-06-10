import type { Vector } from '$lib/types/graphics';

export const subtract = (from: Vector, to: Vector): Vector => ({
  x: to.x - from.x,
  y: to.y - from.y
});

export const add = (a: Vector, b: Vector): Vector => ({
  x: a.x + b.x,
  y: a.y + b.y
});

export const multiply = ({ x, y }: Vector, n: number): Vector => ({
  x: x * n,
  y: y * n
});
