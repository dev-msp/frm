<script lang="ts">
  import { writable } from 'svelte/store';
  import Debug from '../Debug.svelte';
  import Text from './Text.svelte';
  import { getDepth, getParentDim, getRootDim, setParentDim, incDepth } from './Container.svelte';
  import Line from './Line.svelte';

  export let id: string | null = null;
  export let className: string | null = null;

  // layout/origin props
  export let center = false;
  export let centerOrigin = false;
  export let xOffset = 0;
  export let yOffset = 0;
  export let rotation = 0;

  // desired width and height
  export let width: number | undefined = undefined;
  export let height: number | undefined = undefined;

  // top-level ancestor dimensions
  const dim = getParentDim();
  const root = getRootDim();
  const depth = getDepth();
  incDepth();

  $: selfWidth = width || ($dim?.width ?? 0);
  $: selfHeight = height || ($dim?.height ?? 0);

  const newDim = writable({ width: selfWidth, height: selfHeight });
  $: {
    if (selfWidth && selfHeight) {
      $newDim = { width: selfWidth, height: selfHeight };
    }
  }

  setParentDim(newDim);

  $: totalXOffset = xOffset - (center ? selfWidth / 2 : 0);
  $: totalYOffset = yOffset - (center ? selfHeight / 2 : 0);
  $: originX = centerOrigin ? ($dim?.width ?? 0) / 2 : 0;
  $: originY = centerOrigin ? ($dim?.height ?? 0) / 2 : 0;

  const formatTag = (tagName: string, props: { [k: string]: unknown }) => {
    const kvs = Object.entries(props)
      .filter(([, v]) => !['null', 'undefined'].includes(typeof v))
      .map(([k, v]) => `${k}=${JSON.stringify(v)}`)
      .join(' ');
    return `<${tagName} ${kvs} />`;
  };
</script>

<g
  {id}
  class={className}
  transform="translate({originX + totalXOffset}, {originY + totalYOffset}) rotate({rotation})"
>
  <slot width={selfWidth} height={selfHeight} />
</g>

<Debug label="G">
  {@const isTall = Math.abs(($root?.height ?? 0) - selfHeight) < 40}
  {@const baseline = 'hanging'}
  {@const colors = ['red', 'yellow', 'limegreen', 'cyan']}
  {@const color = colors[depth % colors.length]}

  <g
    id="{id ?? 'g'}_DEBUG"
    style:font-family="monospace"
    style:font-size="calc(max(9pt, min(0.8rem, 12pt)))"
    style:stroke-dasharray="8 4"
    style:opacity="0.5"
  >
    {#if Math.abs(totalXOffset) + Math.abs(totalYOffset) > 0}
      {@const a = { x: xOffset, y: yOffset }}
      {@const b = { x: totalXOffset, y: totalYOffset }}
      <g style:font-size="80%">
        <Line
          {color}
          style="dashed"
          r={2}
          endpoints
          a={{ ...a, label: 'x' }}
          b={{ ...b, label: "x'" }}
        >
          <Text slot="endpoint" let:label>
            {label}
          </Text>
        </Line>
      </g>
    {/if}
    <g transform="translate({totalXOffset}, {totalYOffset})">
      <rect width={selfWidth} height={selfHeight} fill="none" stroke={color} />
      <g transform="translate({isTall ? 10 : 0}, {10 + (isTall ? 0 : selfHeight)})">
        <Text {baseline} y="-100%" fill={color}>
          <tspan x="0" y="0">
            {formatTag('G', { id, depth })}
          </tspan>
          <!-- <tspan x="0" y="1.3em" fill="white"> -->
          <!--   ({totalXOffset},{totalYOffset}) {selfWidth}x{selfHeight} -->
          <!-- </tspan> -->
        </Text>
      </g>
    </g>
  </g>
</Debug>
