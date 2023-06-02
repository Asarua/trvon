const { supports } = require('./support')
const { execSync } = require('child_process')
const { basePath, trvonPlatformBase, commonPkg, pkgName, commonPkgPath } = require('./common')
const { platform, arch } = require('./platform')
const chalk = require('chalk')
const fs = require('fs')
const path = require('path')

fs.rmSync(trvonPlatformBase, { recursive: true, force: true })

const optionalDependencies = commonPkg.optionalDependencies || {}

let buildPlatforms = supports
const [currentPlatform] = process.argv.slice(2, 3)
if (currentPlatform) {
  const platforms = supports.filter(({ platform }) => currentPlatform === platform)
  if (platforms.length) {
    buildPlatforms = platforms
  } else {
    console.error(`Fail build to ${chalk.red(currentPlatform)}, You should use ${supports.map(({ platform }) => chalk.green(platform)).join(' / ')}`)
    process.exit(1)
  }
}

for (const { platform, target } of buildPlatforms) {
  const buildCmd = `cargo build --release --target ${target}`
  try {
    console.log(chalk.gray(`start build ${chalk.green(target)}...`))
    execSync(`rustup target add ${target}`, { stdio: 'inherit' })
    execSync(buildCmd, { stdio: 'inherit' })
  } catch(e) {
    console.error(`\n\n${chalk.bold.red(target)} Compilation failed! Please check the compilation settings or try manual compilation\n\n ${chalk.green(buildCmd)}\n`)
    process.exit(1)
  } finally {
    const [targetPlatform, targetArch] = platform.split('-')

    let releaseFileName = pkgName
    if (targetPlatform === 'win32') {
      releaseFileName += '.exe'
    }

    try {
      const releasePath = path.join(basePath, `target/${target}/release/${releaseFileName}`)
      if (fs.existsSync(releasePath)) {
        const { dir, main } = genPlatformDir(targetPlatform, targetArch)
        const toPath = path.join(dir, main)
        execSync(`cp ${releasePath} ${toPath}`)
        console.log(`\n${chalk.green('Success')} copy ${releasePath} to ${toPath}\n`)
      }
    } catch {
      console.error(chalk.red('Copy release pkg fail.'))
    }
  }
}

fs.writeFileSync(
  commonPkgPath,
  JSON.stringify({
    ...commonPkg,
    bin: {
      [pkgName]: './js/scripts/bin.js'
    },
    optionalDependencies
  }, null, 2),
  'utf-8'
)

function genPlatformDir(targetPlatform, targetArch) {
  const dir = path.join(trvonPlatformBase, `/${targetPlatform}-${targetArch}`)
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true })
  }

  const targetPkg = generatePkgJson(targetPlatform, targetArch, commonPkg.version)
  fs.writeFileSync(
    path.join(dir, '/package.json'),
    JSON.stringify(
      targetPkg,
      null,
      2
    ),
    'utf-8'
  )
  fs.writeFileSync(
    path.join(dir, '/README.md'),
      `# Trvon

This is the ${targetPlatform} ${targetArch} binary for trvon. See https://github.com/Asarua/trvon for details.`
  )

  return {
    dir,
    main: targetPkg.main
  }
}

function generatePkgJson(targetPlatform, targetArch, version) {
  const main = `trvon${targetPlatform === 'win32' ? '.exe' : ''}`
  const targetName = `@${pkgName}/${targetPlatform}-${targetArch}`

  optionalDependencies[targetName] = version

  return {
    name: targetName,
    version,
    description: "TRVON -- The rust version of nrm",
    main,
    repository: {
      type: "git",
      url: "git+https://github.com/Asarua/trvon.git"
    },
    keywords: [],
    author: "Asarua",
    license: "MIT",
    bugs: {
      url: "https://github.com/Asarua/trvon/issues"
    },
    homepage: "https://github.com/Asarua/trvon#readme",
    os: [targetPlatform],
    cpu: [targetArch],
    engines: {
      node: '>=14'
    },
    files: [
      "README.md",
      main
    ]
  }
}
