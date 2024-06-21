import { src, dest, series, watch } from 'gulp';

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

export function copyFiles(cb) {
  return series(assets, manifest, templates)(cb)
}

export function watchCopy() {
  return watch('dist/*', copyFiles)
}

export default watchCopy;
