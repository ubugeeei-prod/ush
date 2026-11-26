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
  for (const keyword of ['extern','test','use','for','in','if','as','mut','defer','spawn','status_of','allow_fail']) {
    const result = grammar.tokenizeLine(`${keyword} x`, tm.INITIAL);
    assert(result.tokens[0].scopes.includes('keyword.control.ush'), keyword);
  }
  for (const [source, word, scope] of [
    ['echo Hello $name --output=${name}/file', 'Hello', 'string.unquoted.ush'],
    ['echo Hello $name --output=${name}/file', '$name', 'variable.other.ush'],
    ['echo Hello $name --output=${name}/file', '${name}', 'variable.other.ush'],
    ['let output = greet bare', 'bare', 'string.unquoted.ush'],
    ['let os = match uname -s {', 'match', 'keyword.control.ush'],
    ['let os = match uname -s {', '-s', 'string.unquoted.ush'],
    ['echo hello | tr a-z A-Z', 'a-z', 'string.unquoted.ush'],
    ['echo (greet $name)', '$name', 'variable.other.ush'],
  ]) {
    const at = source.indexOf(word);
    const tokens = grammar.tokenizeLine(source, tm.INITIAL).tokens;
    assert(tokens.some(t => t.startIndex <= at && t.endIndex > at && t.scopes.includes(scope)), source);
  }
  let result = grammar.tokenizeLine('let page = """', tm.INITIAL);
  result = grammar.tokenizeLine('😀 fn fake() {', result.ruleStack);
  assert(result.tokens.every(t => t.scopes.includes('string.quoted.triple.ush')));
  result = grammar.tokenizeLine('"""', result.ruleStack);
  result = grammar.tokenizeLine('fn real<T>(x: T) -> T {', result.ruleStack);
  assert(result.tokens.some(t => t.scopes.includes('entity.name.function.ush')));
});
test('interpolating strings, templates and glob patterns', async () => {
  const data = fs.readFileSync(require.resolve('vscode-oniguruma/release/onig.wasm'));
  await onig.loadWASM(data.buffer.slice(data.byteOffset, data.byteOffset + data.byteLength)).catch(() => {});
  const registry = new tm.Registry({onigLib: Promise.resolve({createOnigScanner: p => new onig.OnigScanner(p), createOnigString: s => new onig.OnigString(s)}), loadGrammar: async () => tm.parseRawGrammar(fs.readFileSync(path.join(__dirname, '../syntaxes/ush.tmLanguage.json'),'utf8'),'ush.json')});
  const grammar = await registry.loadGrammar('source.ush');
  const line = 'let s = $"a $name ${xs.join(", ")} \\${raw}" + "${plain}"';
  const tokens = grammar.tokenizeLine(line, tm.INITIAL).tokens;
  const scopeOf = text => tokens.find(t => line.slice(t.startIndex, t.endIndex) === text).scopes;
  assert(scopeOf('$name').includes('variable.other.ush'));
  assert(scopeOf('${').includes('punctuation.section.interpolation.begin.ush'));
  assert(scopeOf('\\$').includes('constant.character.escape.ush'));
  assert(!tokens.some(t => line.slice(t.startIndex, t.endIndex) === 'plain' && t.scopes.includes('meta.interpolation.ush')));
  let result = grammar.tokenizeLine('let t = $"""', tm.INITIAL);
  result = grammar.tokenizeLine('  ## ${title} $env::HOME', result.ruleStack);
  assert(result.tokens.some(t => t.scopes.includes('meta.interpolation.ush')));
  assert(result.tokens.some(t => t.scopes.includes('variable.other.ush')));
  result = grammar.tokenizeLine('"""', result.ruleStack);
  result = grammar.tokenizeLine('glob "*.ts" => print x', result.ruleStack);
  assert(result.tokens[0].scopes.includes('keyword.control.ush'));
});
