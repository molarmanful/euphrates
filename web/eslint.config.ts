import { molarmanfulLint } from '@molarmanful/fe-tools'

import svelteConfig from './svelte.config'

declare global {
  interface ImportMeta {
    dirname: string
  }
}

export default molarmanfulLint({
  ts: {
    parserOptions: {
      projectService: {
        tsconfigRootDir: import.meta.dirname,
        allowDefaultProject: [
          'eslint.config.ts',
          'svelte.config.ts',
        ],
      },
    },
  },
  svelte: {
    parserOptions: {
      svelteConfig,
    },
  },
}).append({
  settings: {
    'better-tailwindcss': {
      entryPoint: 'src/app.css',
    },
  },
})
