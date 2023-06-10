<script lang="ts">
  import Container, { getParentDim } from '../Container.svelte';
  import G from '../G.svelte';

  type T = $$Generic;

  type DefaultSlot = {
    i: number;
    datum: T;
    row: number;
    col: number;
    width: number;
    height: number;
  };

  interface $$Slots {
    default: DefaultSlot;
    noDatum: Omit<DefaultSlot, 'datum'>;
  }

  export let rows: number;
  export let columns: number;
  export let gutter = 0;
  export let data: T[];

  const dim = getParentDim();
  $: cellWidth = (($dim?.width ?? 0) - gutter * (columns - 1)) / columns;
  $: cellHeight = (($dim?.height ?? 0) - gutter * (rows - 1)) / rows;
</script>

<Container>
  {#each { length: rows * columns } as _, i}
    {@const datum = data[i]}
    {@const [row, col] = [Math.floor(i / columns) % rows, i % columns].map((x) => x)}
    <G
      width={cellWidth}
      height={cellHeight}
      xOffset={col * (cellWidth + gutter)}
      yOffset={row * (cellHeight + gutter)}
    >
      {#if i >= data.length && $$slots.noDatum}
        <slot name="noDatum" {i} {row} {col} width={cellWidth} height={cellHeight} />
      {:else}
        <slot {i} {datum} {row} {col} width={cellWidth} height={cellHeight} />
      {/if}
    </G>
  {/each}
</Container>
