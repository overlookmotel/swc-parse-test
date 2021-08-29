'use strict';

module.exports = function createJs(numLines) {
	let js = '';
	for (let i = 0; i < numLines; i++) {
		js += `const _${i} = ${i};\n`;
	}
	return js;
};
