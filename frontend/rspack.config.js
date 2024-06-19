/** @type {import('@rspack/cli').Configuration} */
export default {
  entry: {
    main: './lib/index.css'
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
