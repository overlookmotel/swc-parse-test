/* eslint-disable no-console */

'use strict';

const swcParse = require('@swc/core').parseSync,
	filesize = require('filesize'),
	assert = require('assert');

const createJs = require('../lib/createJs.js'),
	{parseToBuffer} = require('../lib/addon.js'),
	bufferToAst = require('../lib/buffer/bufferToAst.js'),
	astToBuffer = require('../lib/buffer/astToBuffer.js');

const js = createJs(2);
console.log('JS size:', js.length, `(${filesize(js.length)})`);

const buf = parseToBuffer(js);

const astSwc = swcParse(js);
const bufSwc = astToBuffer(astSwc);
assert.equal(buf.length, bufSwc.length, 'Buffers not identical length');

const ast = bufferToAst(buf);
console.dir(ast, {depth: Infinity});

const astJson = JSON.stringify(ast);
const astJsonSwc = JSON.stringify(astSwc);
assert.equal(astJson, astJsonSwc, 'ASTs not identical');
