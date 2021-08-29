'use strict';

const b = require('benny'); // eslint-disable-line import/no-extraneous-dependencies

const swcParse = require('@swc/core').parseSync,
	babelParse = require('@babel/parser').parse,
	filesize = require('filesize'),
	createJs = require('../lib/createJs.js'),
	swcParseRaw = require('../lib/swcParseRaw.js'),
	parseJson = require('../lib/parseJson.js'),
	{
		parseSync: experimentParse,
		parseToBuffer,
		parseToObject
	} = require('../lib/addon.js');

async function run(numLines) {
	const js = createJs(numLines);

	await b.suite(
		`${numLines} lines (${filesize(js.length)})`,

		b.add('swc', () => {
			swcParse(js);
		}),

		b.add('swc (without deserialization)', () => {
			swcParseRaw(js);
		}),

		b.add('experiment 1 - swc with custom JSON parser', () => {
			parseJson(swcParseRaw(js));
		}),

		b.add('experiment 2 - buffer', () => {
			experimentParse(js);
		}),

		b.add('experiment 2 - buffer (without deserialization)', () => {
			parseToBuffer(js);
		}),

		b.add('experiment 3 - object', () => {
			parseToObject(js);
		}),

		b.add('babel', () => {
			babelParse(js);
		}),

		b.cycle(),
		b.complete(),

		b.save({
			file: `${numLines} lines`,
			folder: __dirname,
			details: true,
			format: 'chart.html'
		})
	);
}

(async () => {
	await run(100);
	await run(1000);
	await run(10000);
})().catch((e) => {
	console.log('ERROR:', e); // eslint-disable-line no-console
});
