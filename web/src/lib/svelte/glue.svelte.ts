import { shim } from '$lib/ts/shim'
import { instantiate } from '$lib/wasm/euph'

import 'core-js/proposals/iterator-helpers-stage-3-2'

const textEncoder = new TextEncoder()
const textDecoder = new TextDecoder()

export class Glue {
  #mod?: Awaited<ReturnType<typeof instantiate>>
  #in = new Uint8Array().values()
  #out = $state('')

  get out() {
    return this.#out
  }

  async run(code: string, input: string) {
    this.#in = textEncoder.encode(input).values()
    this.#out = ''

    this.#mod = await shim({
      readStdin: len => new Uint8Array(this.#in.take(Number(len))),
      writeStdout: cs => {
        this.#out += textDecoder.decode(cs)
      },
      writeStderr: cs => {
        this.#out += textDecoder.decode(cs)
      },
    })

    this.#mod.runEuph(code)
  }
}
