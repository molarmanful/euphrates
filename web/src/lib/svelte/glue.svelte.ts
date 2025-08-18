import { shim } from '$lib/ts/shim'
import { instantiate } from '$lib/wasm/euphrates'

const textDecoder = new TextDecoder()

export class Glue {
  #mod: Awaited<ReturnType<typeof instantiate>> | undefined
  #out = $state('')

  async init() {
    this.#mod = await shim({
      writeStdout: cs => {
        this.#out += textDecoder.decode(cs)
      },
      writeStderr: cs => {
        this.#out += textDecoder.decode(cs)
      },
    })
  }

  get out() {
    return this.#out
  }

  run(code: string) {
    if (!this.#mod) throw new Error('glue not initialized')
    this.clrOut()
    this.#mod.run(code)
  }

  clrOut() {
    this.#out = ''
  }
}
