'use strict';

const pathJoin = require('path').join;
const {loadBinding} = require('@node-rs/helper');

const {
	parseToBuffer, parseToObject
} = loadBinding(pathJoin(__dirname, '..'), 'experiment', '@overlookmotel/swc-parse-test');

const bufferToAst = require('./buffer/bufferToAst.js');

function parseSync(js) {
	const buf = parseToBuffer(js);
	return bufferToAst(buf);
}

module.exports = {
	parseSync,
	parseToBuffer,
	parseToObject
};
