/* eslint-disable no-console */

'use strict';

const swcParse = require('@swc/core').parseSync,
	filesize = require('filesize'),
	assert = require('assert');

const createJs = require('../lib/createJs.js'),
	{parseToObject} = require('../lib/addon.js');

const js = createJs(2);
console.log('JS size:', js.length, `(${filesize(js.length)})`);

const astSwc = swcParse(js);

const ast = parseToObject(js);
console.dir(ast, {depth: Infinity});

const astJson = JSON.stringify(ast);
const astJsonSwc = JSON.stringify(astSwc);
assert.equal(astJson, astJsonSwc, 'ASTs not identical');
