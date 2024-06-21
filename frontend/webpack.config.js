import { resolve } from 'node:path';

const isProduction = process.env.NODE_ENV === 'production';

/** @type {import('webpack').Configuration} */
const config = {
  entry: resolve('src', 'index.js'),
  mode: isProduction ? 'production' : 'development',
  experiments: {
    outputModule: true,
  },
  devtool: isProduction ? false : 'eval',
  output: {
    module: true,
    path: resolve('dist'),
    filename: '[name].[contenthash].js'
  }
};

export default () => {
  return config;
};
