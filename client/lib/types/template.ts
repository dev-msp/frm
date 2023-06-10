export type Contains<T extends string, S extends string> = T extends `${infer _}${S}${infer _}`
  ? T
  : never;
