// NOTE: These should be removed if/when the WASI shim types are fixed.
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-nocheck
import { instantiate } from '$lib/wasm/euph'
import { cli } from '@bytecodealliance/preview2-shim'
import { WASIShim } from '@bytecodealliance/preview2-shim/instantiation'

export const shim = async ({ readStdin, writeStdout, writeStderr }: {
  readStdin: (len: bigint) => Uint8Array
  writeStdout: (cs: Uint8Array) => void
  writeStderr: (cs: Uint8Array) => void
}) =>
  await instantiate(
    void 0,
    new WASIShim({
      cli: {
        ...cli,
        stdin: {
          ...cli.stdin,
          getStdin: () =>
            new cli.stdin.InputStream({
              blockingRead: readStdin,
            }),
        },
        stdout: {
          ...cli.stdout,
          getStdout: () =>
            new cli.stdout.OutputStream({
              write: writeStdout,
            }),
        },
        stderr: {
          ...cli.stderr,
          getStderr: () =>
            new cli.stderr.OutputStream({
              write: writeStderr,
            }),
        },
      },
    }).getImportObject(),
  )
