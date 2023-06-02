const supports = [
  { platform: 'darwin-arm64', target: 'aarch64-apple-darwin' },
  { platform: 'darwin-x64', target: 'x86_64-apple-darwin' },
  { platform: 'win32-x64', target: 'x86_64-pc-windows-gnu' },
  { platform: 'linux-x64', target: 'x86_64-unknown-linux-gnu' }
]

module.exports = {
  supports
}
