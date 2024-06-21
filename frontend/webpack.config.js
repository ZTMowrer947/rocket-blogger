import { resolve } from 'node:path';

import MiniCssExtractPlugin from 'mini-css-extract-plugin';

const isProduction = process.env.NODE_ENV === 'production';

/** @type {import('webpack').Configuration} */
const config = {
  entry: resolve('src', 'index.js'),
  mode: isProduction ? 'production' : 'development',
  experiments: {
    outputModule: true,
  },
  devtool: isProduction ? false : 'eval-source-map',
  output: {
    module: true,
    path: resolve('dist'),
    filename: isProduction ? '[name].[contenthash].js' : '[name].bundle.js',
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          isProduction ? MiniCssExtractPlugin.loader : 'style-loader',
          'css-loader',
          'postcss-loader',
        ]
      }
    ]
  },
  plugins: []
};

export default () => {
  if (isProduction) {
    config.plugins.push(new MiniCssExtractPlugin({
      filename: '[name].[contenthash].css'
    }));
  }

  return config;
};
