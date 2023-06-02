const { platform, arch } = require('os')
const { pkgName } = require('./common')

const currentPlatform = platform()
const currentArch = currentPlatform === 'darwin' && arch() === 'arm64' ? 'arm64' : 'x64'
const currentPlatformId = `@${pkgName}/${platform}-${arch}`

module.exports = {
  platform: currentPlatform,
  arch: currentArch,
  id: currentPlatformId
}
