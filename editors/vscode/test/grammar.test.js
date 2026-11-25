const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const tm = require('vscode-textmate');
const onig = require('vscode-oniguruma');
test('new language syntax, Unicode and multiline strings', async () => {
  const data = fs.readFileSync(require.resolve('vscode-oniguruma/release/onig.wasm'));
  await onig.loadWASM(data.buffer.slice(data.byteOffset, data.byteOffset + data.byteLength));
  const registry = new tm.Registry({onigLib: Promise.resolve({createOnigScanner: p => new onig.OnigScanner(p), createOnigString: s => new onig.OnigString(s)}), loadGrammar: async () => tm.parseRawGrammar(fs.readFileSync(path.join(__dirname, '../syntaxes/ush.tmLanguage.json'),'utf8'),'ush.json')});
  const grammar = await registry.loadGrammar('source.ush');
  for (const keyword of ['extern','test','use','for','in','if','as','mut']) {
    const result = grammar.tokenizeLine(`${keyword} x`, tm.INITIAL);
    assert(result.tokens[0].scopes.includes('keyword.control.ush'), keyword);
  }
  let result = grammar.tokenizeLine('let page = """', tm.INITIAL);
  result = grammar.tokenizeLine('😀 fn fake() {', result.ruleStack);
  assert(result.tokens.every(t => t.scopes.includes('string.quoted.triple.ush')));
  result = grammar.tokenizeLine('"""', result.ruleStack);
  result = grammar.tokenizeLine('fn real<T>(x: T) -> T {', result.ruleStack);
  assert(result.tokens.some(t => t.scopes.includes('entity.name.function.ush')));
});
