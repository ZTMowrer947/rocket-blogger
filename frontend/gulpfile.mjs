import { src, dest, series, watch } from 'gulp';

function assets() {
  return src(['dist/*.js', 'dist/*.css'])
    .pipe(dest('../public/assets'))
}

function templates() {
  return src(['dist/*.html.tera'])
    .pipe(dest('../templates/generated'))
}

export function copyFiles(cb) {
  return series(assets, templates)(cb)
}

export function watchCopy() {
  return watch('dist/*', copyFiles)
}

export default watchCopy;
