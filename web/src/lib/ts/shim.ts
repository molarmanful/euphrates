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
            // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-return
            new io.streams.InputStream({ blockingRead: readStdin }),
        },
        stdout: {
          getStdout: () =>
            // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-return
            new io.streams.OutputStream({ write: writeStdout }),
        },
        stderr: {
          getStderr: () =>
            // eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-return, @typescript-eslint/no-unsafe-member-access
            new io.streams.OutputStream({ write: writeStderr }),
        },
      },
    }).getImportObject(),
  )
