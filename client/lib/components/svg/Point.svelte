<script context="module" lang="ts">
  export type LabelSlot = {
    label?: string;
  };
</script>

<script lang="ts">
  import type { Vector } from '$lib/types/graphics';
  import Text from './Text.svelte';

  interface $$Slots {
    default: LabelSlot;
  }

  export let label: string | undefined = undefined;
  export let labelOffset = 12;
  export let labelAngle = 45;
  export let fill: string | null = 'white';
  export let r = 2;
  export let coord: Vector;

  $: angleRad = (Math.PI * labelAngle) / 180;

  const sq = (n: number) => Math.pow(n, 2);
  let xOffset = labelOffset;
  let yOffset = labelOffset;
  $: {
    const sin = Math.sin(angleRad);
    xOffset = labelOffset * sin;
    yOffset = -labelOffset * Math.sqrt(1 - sq(sin));
  }
</script>

<g style:fill data-type="node" data-node-type="point">
  {#if label}
    <g
      data-type="node"
      data-node-type="point-label"
      transform="translate({coord.x + xOffset}, {coord.y + yOffset})"
      style:dominant-baseline="middle"
    >
      <slot {label}>
        <Text {fill}>{label}</Text>
      </slot>
    </g>
  {/if}
  <circle {r} cx={coord.x} cy={coord.y} />
</g>
