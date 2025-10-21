import { compress as fflCompress, decompress as fflDecompress, strFromU8, strToU8 } from 'fflate'

import 'proposals/array-buffer-base64'

export const compress = async (a: string) =>
  await new Promise<string>((resolve, reject) => {
    fflCompress(strToU8(a), { level: 9, mem: 12 }, (err, res) => {
      if (err) reject(err)
      else resolve(res.toBase64({ alphabet: 'base64url' }))
    })
  })

export const decompress = async (a: string) =>
  await new Promise<string>((resolve, reject) => {
    fflDecompress(Uint8Array.fromBase64(a, { alphabet: 'base64url' }), {}, (err, res) => {
      if (err) reject(err)
      else resolve(strFromU8(res))
    })
  })
