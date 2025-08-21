import extractorSvelte from '@unocss/extractor-svelte'
import { presetWebFonts, presetWind4, transformerDirectives, transformerVariantGroup } from 'unocss'

export default {
  presets: [
    presetWind4({ preflights: { reset: true } }),
    presetWebFonts({
      fonts: {
        sans1: [{ name: 'Satoshi', weights: [400, 700], provider: 'fontshare' }],
        mono: [{ name: 'Atkinson Hyperlegible Mono', provider: 'google' }],
      },
    }),
  ],
  transformers: [transformerDirectives(), transformerVariantGroup()],
  extractors: [extractorSvelte()],
}
