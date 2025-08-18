import extractorSvelte from '@unocss/extractor-svelte'
import { presetWind4, transformerDirectives, transformerVariantGroup } from 'unocss'

export default {
  presets: [
    presetWind4({ preflights: { reset: true } }),
  ],
  transformers: [transformerDirectives(), transformerVariantGroup()],
  extractors: [extractorSvelte()],
}
