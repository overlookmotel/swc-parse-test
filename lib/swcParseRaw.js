'use strict';

const {loadBinding} = require('@node-rs/helper');

const bindings = loadBinding(__dirname, 'swc', '@swc/core');

module.exports = function swcParseRaw(src, options, filename) {
	options = options || {syntax: 'ecmascript'};
	options.syntax = options.syntax || 'ecmascript';
	return bindings.parseSync(src, toBuffer(options), filename);
};

function toBuffer(t) {
	return Buffer.from(JSON.stringify(t));
}
