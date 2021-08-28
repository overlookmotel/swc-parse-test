/* eslint-disable no-console */

'use strict';

const {sync} = require('./index.js');

console.assert(sync(0) === 100, 'Simple test failed');

console.info('Simple test passed');
