import { resolve } from 'node:path';

/** @type {import('@rspack/cli').Configuration} */
export default {
  entry: {
    main: './lib/index.css'
  },
  output: {
    path: resolve(__dirname, '..', 'public', 'assets')
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
