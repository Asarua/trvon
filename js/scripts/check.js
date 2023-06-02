const { platform, arch, id } = require('./platform')
const { pkgName } = require('./common')

let trvonPath

try {
  trvonPath = require.resolve(id)
} catch {
  throw new Error('Trvon is not support your computer npm package yet. You can use cargo to install it. Try do "cargo install trvon"')
  process.exit(1)
}

module.exports = {
  trvonPath
}
