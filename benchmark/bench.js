'use strict';

const b = require('benny'); // eslint-disable-line import/no-extraneous-dependencies

const {sync} = require('../index.js');

function add(a) {
	return a + 100;
}

async function run() {
	await b.suite(
		'Add 100',

		b.add('Native a + 100', () => {
			sync(10);
		}),

		b.add('JavaScript a + 100', () => {
			add(10);
		}),

		b.cycle(),
		b.complete()
	);
}

run().catch((e) => {
	console.error(e); // eslint-disable-line no-console
});
