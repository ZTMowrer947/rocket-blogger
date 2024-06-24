import { src, dest, series, watch } from 'gulp';
import webpackStream from 'webpack-stream';
import webpack from 'webpack';
import webpackConfig from './webpack.config.js';

function build() {
  // Extract webpack config object
  const config = typeof webpackConfig === 'function' ?
    webpackConfig() :
    webpackConfig;

  return src('src/index.js')
    .pipe(webpackStream(config, webpack))
    .pipe(dest('dist/'))
}

// Post-build functions
function assets() {
  return src(['dist/*.js', 'dist/*.css'])
    .pipe(dest('../public/assets'))
}

function templates() {
  return src(['dist/*.html.tera'])
    .pipe(dest('../templates/generated'))
}

function manifest() {
  return src('dist/asset_counts.json')
    .pipe(dest('../templates/generated'))
}

function copyFiles(cb) {
  return series(assets, manifest, templates)(cb)
}

// Build with webpack and copy assets to correct locations
export function buildAndCopy(cb) {
  return series(build, copyFiles)(cb);
}

export function dev() {
  return watch(['src/*', '../templates/*.html.tera'], buildAndCopy)
}

export default dev;
