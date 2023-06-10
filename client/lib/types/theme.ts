// CSS
type Namespaced<T extends string, S extends string> = `${T}-${S}`;

export type CssVar<T extends string = string> = `--${T}`;

export type CssRef<R extends string = string> = `var(${CssVar<R>})`;

type CSSValue = string | number | null;

// Themes

interface Theme {
  [k: string]: Theme | string | true;
}

export type BasicTheme = {
  color: 'primary' | 'secondary' | 'accent';
  layout: 'space';
};

export type SvgTheme = {
  svg: {
    stroke: 'width' | 'color' | 'linecap' | 'dashFormat';
    fill: true;
  };
};

type ThemeVarNames<T extends Theme, K extends keyof T = keyof T> = K extends string
  ? T[K] extends string
    ? Namespaced<K, T[K]>
    : T[K] extends Theme
    ? `${K}-${ThemeVarNames<T[K]>}`
    : T[K] extends true
    ? K
    : never
  : never;

export type ThemeVars<T extends Theme> = `--${ThemeVarNames<T>}`;
export type ThemeRefs<T extends Theme> = `var(${ThemeVars<T>})`;

export type ThemeValues<T extends Theme> = {
  [K in keyof T]: T[K] extends string
    ? { [J in T[K]]: CSSValue }
    : T[K] extends Theme
    ? ThemeValues<T[K]>
    : T[K] extends true
    ? CSSValue
    : never;
};
