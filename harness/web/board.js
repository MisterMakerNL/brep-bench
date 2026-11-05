// What the board's pages share: board.json read into cases and kernels, and the pieces they draw.
// board.json's form is site.py's `board`.  Kernels keep kernels.json's order everywhere: the
// board ranks nothing.

export const OUTCOMES = ['pass', 'built', 'refused', 'wrong', 'crashed', 'timeout', 'missing'];

// The colour groups of the bars and the strip: wrong, crashed and timeout share one red there,
// and are counted apart in words.
export const GROUPS = [
  { key: 'pass', label: 'pass', of: ['pass'] },
  { key: 'built', label: 'built (unsettled, sound)', of: ['built'] },
  { key: 'refused', label: 'refused', of: ['refused'] },
  { key: 'failed', label: 'wrong, crashed or timed out', of: ['wrong', 'crashed', 'timeout'] },
  { key: 'missing', label: 'missing (no implementation yet)', of: ['missing'] },
];
const GROUP = {};
for (const g of GROUPS) for (const o of g.of) GROUP[o] = g.key;
export const groupOf = (outcome) => GROUP[outcome];

export const esc = (s) => String(s).replace(/[&<>"']/g, (c) => (
  { '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c]));

// a case's page, showing `kernel`'s entry if one is named
export const casePage = (id, kernel) => `cases/${id}.html${
  kernel ? `#k=${encodeURIComponent(kernel)}` : ''}`;
export const thumbOf = (id) => `drawings/${id}/iso.png`;
export const kernelPage = (name) => `kernel.html?k=${encodeURIComponent(name)}`;
export const categoryOf = (id) => id.slice(0, id.lastIndexOf('/'));
// whether an outcome is a sound solid: passed, or built where no answer is settled
export const solved = (o) => o === 'pass' || o === 'built';

/** The value a fraction `q` of the way up `sorted` (ascending), or null for an empty list. */
export function quantile(sorted, q) {
  return sorted.length ? sorted[Math.max(0, Math.ceil(q * sorted.length) - 1)] : null;
}

/** A select's option, picked when its value is `chosen`. */
export const option = (value, label, chosen) =>
  `<option value='${esc(value)}'${value === chosen ? ' selected' : ''}>${esc(label)}</option>`;

/** Places a shown tooltip beside the pointer, kept inside the window. */
export function placeTip(tip, event) {
  const pad = 14;
  tip.style.left = `${Math.min(event.clientX + pad, innerWidth - tip.offsetWidth - 8)}px`;
  tip.style.top = `${event.clientY + pad + tip.offsetHeight > innerHeight
    ? event.clientY - tip.offsetHeight - pad : event.clientY + pad}px`;
}

// Below this many seconds a run is mostly the starting of its runner, so how many cores it kept
// busy, and what they bought it, are not reckoned from it.
export const PARALLEL_FROM = 0.1;

/** board.json, read: `cases` (in id order, each with its index `i`), `categories` (in order,
 * each with its cases' indices), `kernels` (every one; those with results carry `outcomes`,
 * `seconds`, `cpu`, `mb`, `built`, `builtCpu`, `builtOne`, `cores`, `speedup`, `why` and
 * `workaround`, one entry a case (a measure null where there was no run), and `start`) and
 * `ran` (those with results).
 *
 * `start` is what starting the kernel's runner costs, building nothing: as the harness measured
 * it (`measured`), or for results from before it did, the kernel's fastest run standing in for
 * it.  `built`, `builtCpu` and `builtOne` are a run's time, CPU time and time on one core less
 * the start's: the kernel's own.
 * `cores` is how many cores a run kept busy on average while building (its CPU time over its
 * time, the start's left out of both) and `speedup` its time on one core over its time on all of
 * them (likewise), both null for a run shorter than PARALLEL_FROM so reckoned. */
export async function load() {
  const response = await fetch('board.json');
  if (!response.ok) throw new Error(`board.json: ${response.status}`);
  const b = await response.json();
  const names = Object.fromEntries(Object.entries(b.outcomes).map(([o, c]) => [c, o]));
  const cases = b.cases.map(([id, title, tags, settled, drawn], i) => ({
    i, id, title, tags: tags.map((t) => b.tags[t]), settled: !!settled, drawn: !!drawn,
    category: categoryOf(id),
  }));
  const categories = [];
  for (const c of cases) {
    let last = categories[categories.length - 1];
    if (!last || last.name !== c.category) categories.push(last = { name: c.category, cases: [] });
    last.cases.push(c.i);
  }
  const kernels = b.kernels.map((k) => {
    const r = b.results[k.name];
    if (!r) return k;
    const outcomes = Array.from(r.codes, (c) => names[c]);
    const { seconds, cpu, mb } = r;
    const least = (list) => {
      const xs = list.filter((x) => x != null);
      return xs.length ? Math.min(...xs) : null;
    };
    const start = k.startup ? { ...k.startup, measured: true }
      : { seconds: least(seconds), cpu: least(cpu), mb: least(mb), one: null,
          measured: false };
    const less = (list, by) => list.map((x) => (x == null ? null : Math.max(0, x - (by || 0))));
    const built = less(seconds, start.seconds);
    const builtCpu = less(cpu, start.cpu);
    const builtOne = less(r.one, start.one ?? start.seconds);
    const long = (i) => built[i] != null && built[i] >= PARALLEL_FROM;
    return {
      ...k,
      outcomes,
      seconds,
      cpu,
      mb,
      start,
      built,
      builtCpu,
      builtOne,
      cores: builtCpu.map((c, i) => (c != null && long(i) ? c / built[i] : null)),
      speedup: builtOne.map((t, i) => (t != null && long(i) ? t / built[i] : null)),
      why: r.why.map((i) => (i < 0 ? '' : b.reasons[i])),
      workaround: r.workaround.map((i) => (i < 0 ? '' : b.workarounds[i])),
    };
  });
  return { repo: b.repo, cases, categories, kernels, ran: kernels.filter((k) => k.outcomes),
           tags: b.tags };
}

/** A time in seconds, as a table shows it: 3 ms, 420 ms, 4.21 s, 12.3 s. */
export function secondsText(x) {
  if (x == null) return '–';
  if (x < 0.001) return '<1 ms';
  if (x < 1) return `${Math.round(x * 1000)} ms`;
  return x < 10 ? `${x.toFixed(2)} s` : `${x.toFixed(1)} s`;
}

/** Megabytes, as a table shows them: 12 MB, 245 MB, 1.24 GB. */
export function megabytesText(x) {
  if (x == null) return '–';
  if (x < 10) return `${x.toFixed(1)} MB`;
  return x < 1000 ? `${Math.round(x)} MB` : `${(x / 1000).toFixed(2)} GB`;
}

/** How many of the cases at `indices` (every case if omitted) kernel `k` has of each outcome. */
export function tally(k, indices) {
  const t = Object.fromEntries(OUTCOMES.map((o) => [o, 0]));
  if (indices) for (const i of indices) t[k.outcomes[i]]++;
  else for (const o of k.outcomes) t[o]++;
  return t;
}

/** A tally in words, outcome by outcome, leaving out the ones that are none. */
export const words = (t) => OUTCOMES.filter((o) => t[o]).map((o) => `${t[o]} ${o}`).join(', ');

/** A tally as a bar: a segment a colour group, as long as its count. */
export function bar(t) {
  const parts = GROUPS.map((g) => [g, g.of.reduce((n, o) => n + t[o], 0)]).filter(([, n]) => n);
  const spans = parts.map(([g, n]) =>
    `<span class=${g.key} style='flex-grow:${n}' title='${n} ${esc(g.label)}'></span>`).join('');
  return `<div class=bar role=img aria-label='${esc(words(t))}'>${spans}</div>`;
}

/** The key to the bars' colours. */
export const legend = () => GROUPS.map((g) =>
  `<span class=key><span class=${g.key}></span>${esc(g.label)}</span>`).join('');

/** A mark's colour for an outcome, read from the stylesheet (it changes with the theme). */
export function markColours() {
  const style = getComputedStyle(document.documentElement);
  return Object.fromEntries(GROUPS.map((g) => [g.key, style.getPropertyValue(`--m-${g.key}`).trim()]));
}

/** Says on the page that the board could not be read. */
export function failed(where, error) {
  where.innerHTML = `<p>The board could not be read: ${esc(error.message)}. Its pages read
    <code>board.json</code> beside them, so they must be served over HTTP (for one made locally,
    <code>python3 -m http.server -d site</code>).</p>`;
}
