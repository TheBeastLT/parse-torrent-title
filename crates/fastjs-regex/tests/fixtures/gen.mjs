import { readFileSync, writeFileSync } from 'node:fs';

const dir = new URL('.', import.meta.url);
const { crafted, strings, patterns } = JSON.parse(readFileSync(new URL('js_parity_input.json', dir), 'utf8'));
const SURROGATE = 'surrogate';

const isHigh = code => code >= 0xd800 && code <= 0xdbff;
const isLow = code => code >= 0xdc00 && code <= 0xdfff;
const hasLoneSurrogate = text => {
  for (let i = 0; i < text.length; i++) {
    const code = text.charCodeAt(i);
    if (isHigh(code) && isLow(text.charCodeAt(i + 1))) i++;
    else if (isHigh(code) || isLow(code)) return true;
  }
  return false;
};
const insidePair = (text, index) => index > 0 && isLow(text.charCodeAt(index)) && isHigh(text.charCodeAt(index - 1));
const splitsSurrogates = value => typeof value === 'string' ? hasLoneSurrogate(value)
  : Array.isArray(value) ? value.some(splitsSurrogates)
  : value !== null && typeof value === 'object' && Object.values(value).some(splitsSurrogates);

const reset = regex => {
  regex.lastIndex = 0;
  return regex;
};
const withG = flags => flags.includes('g') ? flags : `${flags}g`;
const execResult = (text, m) => {
  if (!m) return null;
  const result = [m.index, ...Array.from(m, group => group ?? null)];
  if (m.groups) result.push(Object.fromEntries(Object.entries(m.groups).map(([name, value]) => [name, value ?? null])));
  return insidePair(text, m.index) || splitsSurrogates(result) ? SURROGATE : result;
};
const replacer = (match, ...args) => {
  const named = typeof args.at(-1) === 'object';
  const groups = args.slice(0, named ? -3 : -2);
  const offset = args.at(named ? -3 : -2);
  return `<${[match, ...groups.map(group => group ?? '~'), offset].join('|')}>`;
};
const sparse = (texts, compute, isDefault) => {
  const out = {};
  texts.forEach((text, index) => {
    const value = compute(text);
    if (!isDefault(value, text)) out[index] = splitsSurrogates(value) ? SURROGATE : value;
  });
  return out;
};
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

const results = patterns.map(pattern => {
  const { source, flags } = pattern;
  let regex;
  try {
    regex = new RegExp(source, flags);
  } catch (error) {
    return { ...pattern, error: error.message };
  }
  const global = new RegExp(source, withG(flags));
  const core = pattern.origin === 'core';
  const texts = core ? strings.slice(0, crafted) : strings;
  const exec = text => {
    regex.lastIndex = 0;
    return execResult(text, regex.exec(text));
  };
  const all = text => {
    global.lastIndex = 0;
    return [...text.matchAll(global)].map(m => execResult(text, m));
  };
  for (const text of texts) {
    regex.lastIndex = 0;
    const found = exec(text);
    regex.lastIndex = 0;
    if (regex.test(text) !== (found !== null)) throw new Error(`test and exec disagree for /${source}/${flags}`);
    if (found !== SURROGATE && text.search(regex) !== (found?.[0] ?? -1)) throw new Error(`search for /${source}/${flags}`);
    const texts = text.match(global);
    if (!same(texts ?? [], [...text.matchAll(global)].map(m => m[0]))) throw new Error(`match /${source}/${flags}`);
  }
  const execAt = [];
  texts.forEach((text, index) => {
    for (const lastIndex of [1, 3, 7, text.length]) {
      if (lastIndex > text.length || insidePair(text, lastIndex)) continue;
      global.lastIndex = lastIndex;
      execAt.push([index, lastIndex, execResult(text, global.exec(text))]);
    }
  });
  return {
    ...pattern,
    texts: texts.length,
    exec: sparse(texts, exec, value => value === null),
    all: sparse(texts, all, value => value.length === 0),
    replace: pattern.replacements.map(replacement =>
      sparse(texts, text => text.replace(reset(regex), replacement), (v, t) => v === t)),
    replaceAll: pattern.replacementsAll.map(replacement =>
      sparse(texts, text => text.replaceAll(global, replacement), (v, t) => v === t)),
    replaceFn: core ? sparse(texts, text => text.replace(reset(regex), replacer), (v, t) => v === t) : {},
    replaceAllFn: core ? sparse(texts, text => text.replaceAll(global, replacer), (v, t) => v === t) : {},
    split: sparse(texts, text => text.split(regex), (v, t) => same(v, [t])),
    splitLimit: sparse(texts, text => text.split(regex, 2), (v, t) => same(v, [t])),
    execAt: core ? execAt : [],
  };
});

writeFileSync(new URL('js_parity.json', dir), `${JSON.stringify({ node: process.version, strings, patterns: results })}\n`);
console.log(`${results.length} patterns, ${strings.length} strings, node ${process.version}`);
