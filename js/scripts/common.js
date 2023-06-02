const path = require('path')

const basePath = path.resolve(__dirname, '../..')
const commonPkgPath = path.resolve(basePath, 'package.json')
const commonPkg = require(commonPkgPath)
const pkgName = commonPkg.name
const trvonPlatformBase = path.resolve(__dirname, `../@${pkgName}`)

module.exports = {
  trvonPlatformBase,
  commonPkg,
  commonPkgPath,
  pkgName,
  basePath
}
