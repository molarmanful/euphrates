<script lang='ts'>
  import type { Snippet } from 'svelte'

  import '../app.css'

  let { children }: { children: Snippet } = $props()

  let loaded = $state(false)

  $effect(() => {
    loaded = true
  })
</script>

<div class={['transition-opacity', loaded ? 'opacity-100' : 'opacity-[0.01%]']}>
  {@render children()}
</div>
