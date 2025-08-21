<script lang='ts'>
  import type { Snippet } from 'svelte'

  import { clsx } from '$lib/ts/utils'

  import 'uno.css'
  import '../app.css'

  let { children }: { children: Snippet } = $props()

  let loaded = $state(false)

  $effect(() => {
    loaded = true
  })
</script>

<div class={clsx('transition-opacity', loaded ? 'opacity-100' : 'opacity-0.01')}>
  {@render children()}
</div>
