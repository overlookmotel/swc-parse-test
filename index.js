const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'experiment' means native addon name is `experiment`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `experiment.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@overlookmotel/napi-rs-test-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'experiment', '@overlookmotel/napi-rs-test')
