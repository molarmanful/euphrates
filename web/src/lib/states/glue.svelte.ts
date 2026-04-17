import type { RunnerMsg } from '$lib/ts/runner'

import Runner from '$lib/ts/runner?worker'
import { type EuEnvOpts } from '$lib/wasm/euph'

const textDecoder = new TextDecoder()

export class Glue {
  #worker = $state.raw<Worker>()
  #out = $state('')
  get out() {
    return this.#out
  }

  running = $derived(!!this.#worker)

  run(code: string, input: string, opts: EuEnvOpts) {
    this.#stop()
    this.#out = ''

    this.#worker = new Runner()
    this.#worker.addEventListener(
      'message',
      (
        { data }: MessageEvent<
          RunnerMsg
        >,
      ) => {
        switch (data.type) {
          case 'done': {
            this.#out += textDecoder.decode()
            this.#stop()
            break
          }
          case 'out':
          case 'err': {
            this.#out += textDecoder.decode(data.data, { stream: true })
            break
          }
        }
      },
    )
    this.#worker.postMessage($state.snapshot({ code, input, opts }))
  }

  #stop() {
    this.#worker?.terminate()
    this.#worker = void 0
  }

  interrupt() {
    this.#stop()
    this.#out += '\n[interrupted]'
  }
}
