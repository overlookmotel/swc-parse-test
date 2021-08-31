'use strict';

module.exports = function parseJson(json) {
	let pos = 0;

	const eaters = {
		Module: eatModule,
		VariableDeclaration: eatVariableDeclaration,
		VariableDeclarator: eatVariableDeclarator,
		Identifier: eatIdentifier,
		NumericLiteral: eatNumericLiteral
	};

	return eatNode();

	function eatNode() {
		pos += 9; // `{"type":"`
		const type = eatString();
		pos += 18; // `","span":{"start":`
		const start = eatNumber(',');
		pos += 7; // `,"end":`
		const end = eatNumber(',');
		pos += 8; // `,"ctxt":`
		const ctxt = eatNumber('}');
		pos += 3; // `},"`

		const node = {type, span: {start, end, ctxt}};
		eaters[type](node);
		pos++; // `}`
		return node;
	}

	function eatString() {
		return eatToChar('"');
	}

	function eatNumber(endChar) {
		return +eatToChar(endChar);
	}

	function eatBoolean() {
		if (json[pos] === 't') {
			pos += 4;
			return true;
		}
		pos += 5;
		return false;
	}

	function eatToChar(endChar) {
		const endPos = json.indexOf(endChar, pos);
		const val = json.slice(pos, endPos);
		pos = endPos;
		return val;
	}

	function eatElements() {
		const arr = [];
		while (true) { // eslint-disable-line no-constant-condition
			arr.push(eatNode());
			if (json[pos] === ']') break;
			pos++; // `,`
		}
		pos++; // ']'
		return arr;
	}

	function eatModule(node) {
		pos += 7; // `body":[`
		node.body = eatElements();
		node.interpreter = null;
		pos += 19; // `,"interpreter":null`
	}

	function eatVariableDeclaration(node) {
		pos += 7; // `kind":"`
		node.kind = eatString();
		pos += 12; // `","declare":`
		node.declare = eatBoolean();
		pos += 17; // `,"declarations":[`

		node.declarations = eatElements();
	}

	function eatVariableDeclarator(node) {
		pos += 4; // `id":`
		node.id = eatNode();
		pos += 8; // `,"init":`
		node.init = eatNode();
		pos += 12; // `,"definite":`
		node.definite = eatBoolean();
	}

	function eatIdentifier(node) {
		pos += 8; // `value":"`
		node.value = eatString();
		pos += 13; // `","optional":`
		node.optional = eatBoolean();
		node.typeAnnotation = null;
		pos += 22; // `,"typeAnnotation":null`
	}

	function eatNumericLiteral(node) {
		pos += 7; // `value":"`
		node.value = eatNumber('}');
	}
};
