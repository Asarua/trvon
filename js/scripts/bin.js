#!/usr/bin/env node
const { trvonPath } = require('./check')

if (trvonPath) {
  require('child_process').spawnSync(
    trvonPath,
    [process.argv.slice(2)],
    {
      stdio: 'inherit'
    }
  )
}
