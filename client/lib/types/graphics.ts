export type Vector = {
  x: number;
  y: number;
};

export type Rect = { [k in 'x' | 'y' | 'width' | 'height']: number };
