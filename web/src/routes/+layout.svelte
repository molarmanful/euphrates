<script lang='ts'>
  import '$lib/ts/polyfills'

  import type { Snippet } from 'svelte'

  import '../app.css'

  let { children }: { children: Snippet } = $props()

  let isLoaded = $state(false)

  $effect(() => {
    isLoaded = true
  })
</script>

<div class={['transition-opacity', isLoaded ? 'opacity-100' : 'opacity-[0.01%]']}>
  {@render children()}
</div>
