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
	VAR_DECLARE_MASK
} = require('./constants.js');

const varKindToCode = {
	var: VAR_KIND_VAR,
	let: VAR_KIND_LET,
	const: VAR_KIND_CONST
};

class Writer {
	constructor() {
		this.bytes = [];
	}

	ast(node) {
		this.Node(node);
		return this.bytes;
	}

	Node(node) {
		this[node.type](node);
	}

	Module(node) {
		this.bytes.push(TYPE_MODULE);
		this.span(node);
		this.nodesArray(node.body);
	}

	VariableDeclaration(node) {
		const {bytes} = this;
		bytes.push(TYPE_VARIABLE_DECLARATION);
		this.span(node);
		bytes.push(varKindToCode[node.kind] + node.declare * VAR_DECLARE_MASK);
		this.nodesArray(node.declarations);
	}

	VariableDeclarator(node) {
		this.bytes.push(TYPE_VARIABLE_DECLARATOR);
		this.span(node);
		this.bool(node.definite);
		this.Node(node.id);
		this.Node(node.init);
	}

	Identifier(node) {
		const {bytes} = this;
		bytes.push(TYPE_IDENTIFIER);
		this.span(node);
		this.bool(node.optional);

		const {value} = node,
			len = value.length;
		this.int16(len);
		for (let i = 0; i < len; i++) {
			bytes.push(value.charCodeAt(i));
		}
	}

	NumericLiteral(node) {
		this.bytes.push(TYPE_NUMERIC_LITERAL);
		this.span(node);
		this.int32(node.value);
	}

	span(node) {
		const {span} = node;
		this.int32(span.start);
		this.int32(span.end);
		this.int8(span.ctxt);
	}

	nodesArray(nodes) {
		this.int32(nodes.length);
		nodes.forEach(node => this.Node(node));
	}

	bool(b) {
		this.bytes.push(+b);
	}

	int8(n) {
		this.bytes.push(n);
	}

	int16(n) {
		this.int(n, 2);
	}

	int32(n) {
		this.int(n, 4);
	}

	int(num, numBytes) {
		const {bytes} = this;
		for (let i = 0; i < numBytes; i++) {
			const byte = num % 256;
			bytes.push(byte);
			num = (num - byte) / 256;
		}
	}
}

module.exports = function astToBuffer(node) {
	const writer = new Writer();
	return Buffer.from(writer.ast(node));
};
