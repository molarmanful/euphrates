// NOTE: These should be removed if/when the WASI shim types are fixed.
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-nocheck
import { instantiate } from '$lib/wasm/euph'
import { cli } from '@bytecodealliance/preview2-shim'
import { WASIShim } from '@bytecodealliance/preview2-shim/instantiation'

const wasms = import.meta.glob('$lib/wasm/*.wasm', {
  query: '?url',
  import: 'default',
  eager: true,
})

const modules = new Map<string, WebAssembly.Module>()
const loader = async (path: string) => {
  if (!modules.has(path))
    modules.set(path, await WebAssembly.compileStreaming(fetch(wasms[`/src/lib/wasm/${path}`])))
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
    new WASIShim({
      cli: {
        ...cli,
        stdin: { getStdin: () => new cli.stdin.InputStream({ blockingRead: readStdin }) },
        stdout: { getStdout: () => new cli.stdout.OutputStream({ write: writeStdout }) },
        stderr: { getStderr: () => new cli.stderr.OutputStream({ write: writeStderr }) },
      },
    }).getImportObject(),
  )
