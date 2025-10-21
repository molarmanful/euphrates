import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'
import legacy from '@vitejs/plugin-legacy'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [
    tailwindcss(),
    sveltekit(),
    legacy({
      modernPolyfills: ['proposals/iterator-helpers-stage-3-2', 'proposals/array-buffer-base64'],
      renderLegacyChunks: false,
    }),
  ],
})
