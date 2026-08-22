import { $init, type EuEnvOpts, runEuph } from '$lib/wasm/euph'
import { cli } from '@bytecodealliance/preview2-shim'

export type RunnerMsg =
  | { type: 'done' }
  | { type: 'out' | 'err'; data: ArrayBufferLike }

const textEncoder = new TextEncoder()

const post = (data: RunnerMsg, transfer?: Transferable[]) => {
  postMessage(data, { transfer })
}

addEventListener('message', (
  { data: { code, input, opts } }: MessageEvent<{
    code: string
    input: string
    opts: EuEnvOpts
  }>,
) => {
  const inputIter = textEncoder.encode(input).values()

  cli._setStdin({
    blockingRead: len => new Uint8Array(inputIter.take(Number(len))),
  })
  cli._setStdout({
    write({ buffer }) {
      post({ type: 'out', data: buffer }, [buffer])
    },
  })
  cli._setStderr({
    write({ buffer }) {
      post({ type: 'err', data: buffer }, [buffer])
    },
  })

  void (async () => {
    try {
      await $init
      runEuph(code, opts)
    } catch (error) {
      console.error(error)
    }
    post({ type: 'done' })
    close()
  })()
})
