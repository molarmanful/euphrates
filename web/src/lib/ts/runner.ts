import type { EuEnvOpts } from '$lib/wasm/euph'

import { shim } from './shim'

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

  void (async () => {
    const mod = await shim({
      readStdin: len => new Uint8Array(inputIter.take(Number(len))),
      writeStdout({ buffer }) {
        post({ type: 'out', data: buffer }, [buffer])
      },
      writeStderr({ buffer }) {
        post({ type: 'err', data: buffer }, [buffer])
      },
    })

    try {
      mod.runEuph(code, opts)
    } catch {
      void 0
    }
    post({ type: 'done' })
    close()
  })()
})
