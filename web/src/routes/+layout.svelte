<script lang='ts'>
  import type { Snippet } from 'svelte'

  import 'uno.css'
  import '../app.css'

  let { children }: { children: Snippet } = $props()
</script>

{@render children()}
