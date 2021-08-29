/* eslint-disable no-bitwise */

'use strict';

const {
	types: {
		TYPE_MODULE,
		TYPE_VARIABLE_DECLARATION,
		TYPE_VARIABLE_DECLARATOR,
		TYPE_IDENTIFIER,
		TYPE_NUMERIC_LITERAL
	},
	varKinds: {
		VAR_KIND_VAR,
		VAR_KIND_LET,
		VAR_KIND_CONST
	},
	VAR_KIND_MASK,
	VAR_DECLARE_MASK
} = require('./constants.js');

let buf, pos;

module.exports = function bufferToAst(_buf) {
	buf = _buf;
	pos = 0;
	try {
		return parseNode(buf);
	} catch (err) {
		console.log(`Error at byte ${pos}`); // eslint-disable-line no-console
		throw err;
	}
};

const parsers = Object.assign([], {
	[TYPE_MODULE]: parseModule,
	[TYPE_VARIABLE_DECLARATION]: parseVariableDeclaration,
	[TYPE_VARIABLE_DECLARATOR]: parseVariableDeclarator,
	[TYPE_IDENTIFIER]: parseIdentifier,
	[TYPE_NUMERIC_LITERAL]: parseNumericLiteral
});

function parseNode() {
	// Get type code + span
	const typeCode = buf[pos];
	const span = {
		start: buf.readUInt32LE(pos + 1),
		end: buf.readUInt32LE(pos + 5),
		ctxt: buf[pos + 9]
	};
	pos += 10;

	// Parse to node
	return parsers[typeCode](span);
}

function parseModule(span) {
	return {
		type: 'Module',
		span,
		body: parseNodesArray(buf),
		interpreter: null
	};
}

const varKindCodeToKind = Object.assign([], {
	[VAR_KIND_VAR]: 'var',
	[VAR_KIND_LET]: 'let',
	[VAR_KIND_CONST]: 'const'
});

function parseVariableDeclaration(span) {
	const kindCode = buf[pos++];

	return {
		type: 'VariableDeclaration',
		span,
		kind: varKindCodeToKind[kindCode & VAR_KIND_MASK],
		declare: !!(kindCode & VAR_DECLARE_MASK),
		declarations: parseNodesArray()
	};
}

function parseVariableDeclarator(span) {
	const definite = !!buf[pos++];
	return {
		type: 'VariableDeclarator',
		span,
		id: parseNode(),
		init: parseNode(),
		definite
	};
}

function parseIdentifier(span) {
	const optional = !!buf[pos],
		len = buf.readUInt16LE(pos + 1);
	pos += 3;
	return { // eslint-disable-line no-return-assign
		type: 'Identifier',
		span,
		value: buf.slice(pos, pos += len).toString(),
		optional,
		typeAnnotation: null
	};
}

function parseNumericLiteral(span) {
	const node = {
		type: 'NumericLiteral',
		span,
		value: buf.readUInt32LE(pos)
	};
	pos += 4;
	return node;
}

function parseNodesArray() {
	const len = buf.readUInt32LE(pos);
	pos += 4;
	const nodes = Array(len);
	for (let i = 0; i < len; i++) {
		nodes[i] = parseNode(buf);
	}
	return nodes;
}
