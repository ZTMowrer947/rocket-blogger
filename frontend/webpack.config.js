import { resolve } from 'node:path';

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
  }
};

export default () => {
  return config;
};
