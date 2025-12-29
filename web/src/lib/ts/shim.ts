import { instantiate } from '$lib/wasm/euph'
// @ts-expect-error p2-shim types are broken
import { cli, io } from '@bytecodealliance/preview2-shim'
import { WASIShim } from '@bytecodealliance/preview2-shim/instantiation'

const wasms = import.meta.glob<string>('$lib/wasm/*.wasm', {
  query: '?url',
  import: 'default',
  eager: true,
})

const modules = new Map<string, WebAssembly.Module>()
const loader = async (path: string) => {
  if (!modules.has(path)) {
    modules.set(
      path,
      await WebAssembly.compileStreaming(fetch(wasms[`/src/lib/wasm/${path}`])),
    )
  }
  const res = modules.get(path)
  if (!res) throw new Error(`module ${path} not found`)
  return res
}

// TODO: remove when p2-shim updates
// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
class InputStream extends io.streams.InputStream {
  subscribe() {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-return, @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
    return new io.poll.Pollable()
  }
}

// TODO: remove when p2-shim updates
// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
class OutputStream extends io.streams.OutputStream {
  subscribe() {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-return, @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
    return new io.poll.Pollable()
  }
}

export const shim = async ({ readStdin, writeStdout, writeStderr }: {
  readStdin: (len: bigint) => Uint8Array
  writeStdout: (cs: Uint8Array) => void
  writeStderr: (cs: Uint8Array) => void
}) =>
  await instantiate(
    loader,
    // @ts-expect-error p2-shim types are broken
    new WASIShim({
      // @ts-expect-error p2-shim types are broken
      cli: {
        // @ts-expect-error p2-shim types are broken
        ...cli,
        stdin: {
          getStdin: () =>
            new InputStream(
              // @ts-expect-error p2-shim types are broken
              { blockingRead: readStdin },
            ),
        },
        stdout: {
          getStdout: () =>
            new OutputStream(
              // @ts-expect-error p2-shim types are broken
              { write: writeStdout },
            ),
        },
        stderr: {
          getStderr: () =>
            new OutputStream(
              // @ts-expect-error p2-shim types are broken
              { write: writeStderr },
            ),
        },
      },
    }).getImportObject(),
  )
