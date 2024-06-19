import { resolve, dirname } from 'node:path';

const thisDir = dirname(new URL(import.meta.url).pathname);

/** @type {import('@rspack/cli').Configuration} */
export default {
  entry: {
    main: './lib/index.css'
  },
  output: {
    path: resolve(thisDir, '..', 'public', 'assets')
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [{ loader: 'postcss-loader' }],
        type: 'css',
      }
    ]
  }
}
