<script lang='ts'>
  import { replaceState } from '$app/navigation'
  import { page } from '$app/state'
  import { Glue } from '$lib/svelte/glue.svelte'
  import { compress, decompress } from '$lib/ts/utils'
  import { onMount } from 'svelte'

  const glue = new Glue()

  let stdin = $state('')
  let code = $state('')

  const autoScroll = (...[node]: [HTMLTextAreaElement, unknown]) => ({
    update() {
      requestAnimationFrame(() => {
        node.scroll({
          top: node.scrollHeight,
        })
      })
    },
  })

  const processKV = async (
    k: string,
    v: string,
  ): Promise<[[string, string]] | []> => v ? [[k, await compress(v)]] : []

  const setParams = async () => {
    const kvs = await Promise.all([
      processKV('code', code),
      processKV('stdin', stdin),
    ])
    const params = new URLSearchParams(kvs.flat())
    replaceState(
      `?${params}`,
      page.state,
    )
  }

  const getParam = async (p: string) => {
    const b64 = page.url.searchParams.get(p)
    return b64 ? await decompress(b64) : ''
  }

  onMount(async () => {
    await glue.init()
    ;[stdin, code] = await Promise.all([
      getParam('stdin'),
      getParam('code'),
    ])
  })
</script>

<svelte:window
  onkeydown={(e: KeyboardEvent) => {
    if (e.ctrlKey && e.key === 'Enter') glue.run(code)
  }}
/>

<svelte:head>
  <title>euphrates</title>
</svelte:head>

<div class='p-4 flex flex-col gap-4 h-screen'>
  <header>
    <h1 class='mb-3'>euphrates</h1>
    <div class='flex gap-3 items-start'>
      <button
        class='btn'
        onclick={() => {
          glue.run(code)
        }}
      >
        run
      </button>
    </div>
  </header>

  <main class='flex flex-1 gap-4 size-full *:flex-1'>
    <textarea
      class='box resize-none'
      bind:value={code}
      placeholder='code goes here...'
      oninput={setParams}
    ></textarea>

    <textarea
      disabled
      class='box h-full whitespace-pre-wrap overflow-auto'
      use:autoScroll={glue.out}
    >{glue.out}</textarea>
  </main>
</div>
