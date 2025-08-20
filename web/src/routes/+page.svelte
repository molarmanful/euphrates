<script lang='ts'>
  import { Glue } from '$lib/svelte/glue.svelte'
  import { onMount } from 'svelte'

  const glue = new Glue()

  let code = $state('')

  onMount(async () => {
    await glue.init()
  })

  // eslint-disable-next-line @typescript-eslint/no-unnecessary-type-parameters, @typescript-eslint/no-unused-vars
  const autoScroll = <T>(node: HTMLTextAreaElement, _: T) => ({
    update() {
      requestAnimationFrame(() => {
        node.scroll({
          top: node.scrollHeight,
        })
      })
    },
  })
</script>

<div class='p-4 flex flex-col gap-4 h-screen'>
  <header>
    <button
      class='btn'
      onclick={() => {
        glue.run(code)
      }}
    >
      run
    </button>
  </header>

  <main class='flex flex-1 gap-4 size-full *:flex-1'>
    <textarea
      class='box resize-none'
      bind:value={code}
      placeholder='code goes here...'
    ></textarea>

    <textarea
      disabled
      class='box h-full whitespace-pre-wrap overflow-auto'
      use:autoScroll={glue.out}
    >{glue.out}</textarea>
  </main>
</div>
