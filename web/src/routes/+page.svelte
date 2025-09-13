<script lang='ts'>
  import { replaceState } from '$app/navigation'
  import { page } from '$app/state'
  import { Glue } from '$lib/svelte/glue.svelte'
  import { clsx, compress, decompress } from '$lib/ts/utils'
  import { onMount } from 'svelte'
  import { fade } from 'svelte/transition'

  const glue = new Glue()

  let params = $state({
    header: { value: '', open: false },
    code: { value: '', open: true },
    footer: { value: '', open: false },
    input: { value: '', open: false },
  })

  type ParamKey = keyof typeof params

  const autoScroll = (...[node]: [HTMLTextAreaElement, unknown]) => ({
    update() {
      requestAnimationFrame(() => {
        node.scroll({
          top: node.scrollHeight,
        })
      })
    },
  })

  const setParams = async () => {
    replaceState(
      `?${new URLSearchParams(
        await Promise.all(
          Object.entries(params)
            .filter(([, { value }]) => value)
            .map(async ([k, { value }]) => [k, await compress(value)]),
        ),
      )}`,
      page.state,
    )
  }

  const getParam = async (p: ParamKey) => {
    const b64 = page.url.searchParams.get(p)
    if (b64) {
      params[p].value = await decompress(b64)
      params[p].open = true
    }
  }

  const run = () => {
    glue.run(
      (['header', 'code', 'footer'] as const)
        .map(p => params[p].value)
        .join('\n'),
    )
  }

  onMount(async () => {
    await glue.init()
    await Promise.all(
      Object.keys(params).map(p => getParam(p as ParamKey)),
    )
  })
</script>

<svelte:window
  onkeydown={(e: KeyboardEvent) => {
    if (e.ctrlKey && e.key === 'Enter') run()
  }}
/>

<svelte:head>
  <title>euphrates</title>
</svelte:head>

<div class='flex h-screen flex-col gap-2 p-4'>
  <header>
    <h1 class='mb-3'>euphrates</h1>
    <div class='flex items-start gap-3'>
      <button
        class='btn'
        onclick={run}
      >
        run
      </button>
    </div>
  </header>

  {#snippet paramBox(p: ParamKey)}
    <div
      class={clsx(
        'box',
        params[p].open
          && (p === 'code' ? 'flex-1' : 'h-1/5'),
      )}
    >
      <button
        class='text-left'
        onclick={() => {
          params[p].open = !params[p].open
        }}
      >
        <svg
          viewBox='0 0 100 100'
          class={clsx(
            'inline-block h-2 fill-current transition-transform',
            params[p].open && 'rotate-90',
          )}
        >
          <polygon points='0 0, 0 100, 100 50' />
        </svg>
        {p}
        {#if p === 'code' && params[p].open}
          <span transition:fade={{ duration: 150 }} class='float-right'>
            <code>{params.code.value.length}</code> chars /
            <code>{new TextEncoder().encode(params.code.value).length}</code>
            bytes
          </span>
        {/if}
      </button>
      {#if params[p].open}
        <textarea
          class='whitespace-pre'
          bind:value={params[p].value}
          placeholder='{p} goes here...'
          oninput={setParams}
        ></textarea>
      {/if}
    </div>
  {/snippet}

  <main class='grid size-full grid-cols-2 gap-4'>
    <div class='flex flex-col gap-1'>
      <!-- eslint-disable @typescript-eslint/no-confusing-void-expression -->
      {@render paramBox('header')}
      {@render paramBox('code')}
      {@render paramBox('footer')}
      <!-- eslint-enable @typescript-eslint/no-confusing-void-expression -->
    </div>

    <div class='flex flex-col gap-1'>
      <!-- eslint-disable @typescript-eslint/no-confusing-void-expression -->
      {@render paramBox('input')}
      <!-- eslint-enable @typescript-eslint/no-confusing-void-expression -->

      <div class='box flex-1'>
        <label for='output'>output</label>
        <textarea
          id='output'
          disabled
          class='whitespace-pre-wrap'
          use:autoScroll={glue.out}
        >{glue.out}</textarea>
      </div>
    </div>
  </main>
</div>
