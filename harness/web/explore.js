// The explorer: every case, a row each, and each shown kernel's outcome of it, filtered by words,
// category, tag, whether it is settled, a view, and each kernel's outcome.  What it shows is in
// the address, so a view can be linked to.  Only the rows in sight are in the page.
import { load, tally, words, bar, legend, esc, casePage, kernelPage, thumbOf,
         failed, secondsText, megabytesText, solved, option, OUTCOMES } from './board.js';
import { sortButton, compare, next } from './sort.js';

const main = document.getElementById('explore');
load().then(start, (e) => failed(main, e));

// what a kernel's column can be filtered to
const FILTERS = [
  ['', 'any'],
  ['pass', 'pass'],
  ['not-pass', 'not pass or built'],
  ['built', 'built'],
  ['refused', 'refused'],
  ['failed', 'wrong, crashed or timeout'],
  ['wrong', 'wrong'],
  ['crashed', 'crashed'],
  ['timeout', 'timeout'],
  ['implemented', 'implemented'],
  ['missing', 'missing'],
];
const FILTER_LABEL = Object.fromEntries(FILTERS);
function admits(filter, o) {
  switch (filter) {
    case '': return true;
    case 'not-pass': return !solved(o) && o !== 'missing';
    case 'failed': return o === 'wrong' || o === 'crashed' || o === 'timeout';
    case 'implemented': return o !== 'missing';
    default: return o === filter;
  }
}

const VIEWS = [
  ['', 'every case'],
  ['nopass', 'where no kernel passes'],
  ['onepass', 'where one kernel alone passes'],
  ['differ', 'where the shown kernels differ'],
  ['workaround', 'with a workaround'],
  ['elegance', 'with an elegant solution'],
];

function start(b) {
  const byName = Object.fromEntries(b.ran.map((k) => [k.name, k]));
  const hay = b.cases.map((c) => `${c.id} ${c.title} ${c.tags.join(' ')}`.toLowerCase());
  const tagCount = {};
  for (const c of b.cases) for (const t of c.tags) tagCount[t] = (tagCount[t] || 0) + 1;

  // the state, read from the address
  const params = new URLSearchParams(location.search);
  const state = {
    q: params.get('q') || '',
    cats: new Set(params.getAll('cat')),
    tags: new Set(params.getAll('tag')),
    settled: params.get('settled') || '',
    view: params.get('view') || '',
    // what a cell shows: its outcome, or what the run cost (its colour is the outcome's)
    cells: ['time', 'memory'].includes(params.get('cells')) ? params.get('cells') : '',
    shown: params.get('show')
      ? params.get('show').split(',').filter((n) => byName[n])
      : b.ran.map((k) => k.name),
    filters: new Map(),
    // the column the rows are sorted by ('case', or a kernel's name; '' for the cases' order),
    // and which way
    sort: '',
    order: 'ascending',
  };
  {
    const s = params.get('sort') || '';
    const name = s.replace(/^-/, '');
    if (name === 'case' || byName[name]) {
      state.sort = name;
      state.order = s.startsWith('-') ? 'descending' : 'ascending';
    }
  }
  for (const [key, value] of params) {
    if (key.startsWith('f.') && byName[key.slice(2)] && FILTER_LABEL[value] !== undefined) {
      state.filters.set(key.slice(2), value);
    }
  }

  function address() {
    const p = new URLSearchParams();
    if (state.q) p.set('q', state.q);
    for (const c of state.cats) p.append('cat', c);
    for (const t of state.tags) p.append('tag', t);
    if (state.settled) p.set('settled', state.settled);
    if (state.view) p.set('view', state.view);
    if (state.cells) p.set('cells', state.cells);
    if (state.sort) p.set('sort', `${state.order === 'descending' ? '-' : ''}${state.sort}`);
    if (state.shown.length !== b.ran.length) p.set('show', state.shown.join(','));
    for (const [k, f] of state.filters) if (f && state.shown.includes(k)) p.set(`f.${k}`, f);
    const s = p.toString();
    history.replaceState(null, '', s ? `?${s}` : location.pathname);
  }

  main.innerHTML = `
    <div class=controls>
      <input type=search placeholder='Search titles, ids and tags' aria-label='Search'
        value='${esc(state.q)}'>
      <select data-add=cat aria-label='Add a category'><option value=''>category…</option>
        ${b.categories.map((c) => option(c.name, `${c.name} (${c.cases.length})`)).join('')}</select>
      <select data-add=tag aria-label='Add a tag'><option value=''>tag…</option>
        ${Object.keys(tagCount).sort().map((t) => option(t, `${t} (${tagCount[t]})`)).join('')}
      </select>
      <select name=settled aria-label='Settled or not'>
        ${option('', 'settled or not', state.settled)}${option('yes', 'settled', state.settled)}
        ${option('no', 'unsettled', state.settled)}</select>
      <select name=view aria-label='View'>
        ${VIEWS.map(([v, l]) => option(v, l, state.view)).join('')}</select>
      <select name=cells aria-label='What a cell shows'>
        ${option('', 'cells: outcome', state.cells)}${option('time', 'cells: time', state.cells)}
        ${option('memory', 'cells: memory', state.cells)}
      </select>
    </div>
    <div class=columns role=group aria-label='Kernels shown'><span class=muted>Kernels:</span>
      ${b.ran.map((k) => `<button type=button class=chip data-kernel='${esc(k.name)}'
        aria-pressed=${state.shown.includes(k.name)}>${esc(k.name)}</button>`).join('')}</div>
    <div class=active></div>
    <p class=found></p>
    <div class=table-frame><table class=explore><thead></thead><tbody></tbody></table></div>
    <p class=legend>${legend()}</p>
    <p class=muted>A cell's outcome links to the case's results; point at it for why, or how long
      it took (<em>cells: time</em> and <em>cells: memory</em> show what each run cost, coloured
      by outcome;
      <a href='performance.html'>Performance</a> compares them). † marks a result reached by working around a feature the kernel lacks. Choose
      kernels above to compare only them: <em>where the shown kernels differ</em> then lists the
      cases they do not agree on.</p>`;

  const search = main.querySelector('input[type=search]');
  const active = main.querySelector('.active');
  const found = main.querySelector('.found');
  const frame = main.querySelector('.table-frame');
  const thead = main.querySelector('thead');
  const tbody = main.querySelector('tbody');

  let matched = [];
  let shown = [];

  function filter() {
    shown = state.shown.map((n) => byName[n]);
    const terms = state.q.toLowerCase().split(/\s+/).filter(Boolean);
    const kernelFilters = shown.map((k) => [k, state.filters.get(k.name) || ''])
      .filter(([, f]) => f);
    matched = b.cases.filter((c) => {
      const i = c.i;
      if (state.cats.size && !state.cats.has(c.category)) return false;
      for (const t of state.tags) if (!c.tags.includes(t)) return false;
      if (state.settled === 'yes' && !c.settled) return false;
      if (state.settled === 'no' && c.settled) return false;
      for (const w of terms) if (!hay[i].includes(w)) return false;
      for (const [k, f] of kernelFilters) if (!admits(f, k.outcomes[i])) return false;
      switch (state.view) {
        case 'nopass': return !b.ran.some((k) => solved(k.outcomes[i]));
        case 'onepass': return b.ran.filter((k) => solved(k.outcomes[i])).length === 1;
        case 'differ': {
          const seen = new Set(shown.map((k) => k.outcomes[i]).filter((o) => o !== 'missing'));
          return seen.size > 1;
        }
        case 'workaround': return shown.some((k) => k.workaround[i]);
        case 'elegance': return shown.some((k) => k.elegance[i]);
        default: return true;
      }
    }).map((c) => c.i);
    sort();
  }

  // the matched cases in the order of the column sorted by: a case by its title, a kernel's
  // column by what its cells show (an outcome in the outcomes' order, or what the run cost),
  // its blanks last; ties keep the cases' order
  function sort() {
    if (state.sort && state.sort !== 'case' && !state.shown.includes(state.sort)) state.sort = '';
    if (!state.sort) return;
    const k = byName[state.sort];
    const key = state.sort === 'case' ? (i) => b.cases[i].title
      : (i) => {
        if (k.outcomes[i] === 'missing') return null;
        if (state.cells === 'time') return k.seconds[i];
        if (state.cells === 'memory') return k.mb[i];
        return OUTCOMES.indexOf(k.outcomes[i]);
      };
    const keys = new Map(matched.map((i) => [i, key(i)]));
    const dir = state.order === 'descending' ? -1 : 1;
    matched.sort((x, y) => compare(keys.get(x), keys.get(y), dir));
  }

  function chips() {
    const out = [];
    for (const c of state.cats) out.push(['cat', c, `category: ${c}`]);
    for (const t of state.tags) out.push(['tag', t, `tag: ${t}`]);
    for (const k of state.shown) {
      const f = state.filters.get(k);
      if (f) out.push(['filter', k, `${k}: ${FILTER_LABEL[f]}`]);
    }
    active.innerHTML = out.map(([kind, value, label]) => `<button type=button class=chip
      data-remove='${kind}' data-value='${esc(value)}' title='Remove'>${esc(label)}</button>`)
      .join('') + (out.length || state.q || state.settled || state.view
      ? `<a href='explore.html' class=muted>clear everything</a>` : '');
    active.hidden = !out.length && !state.q && !state.settled && !state.view;
  }

  function head() {
    const select = (k) => `<select data-filter='${esc(k.name)}' aria-label='${esc(k.name)}: outcome'>
      ${FILTERS.map(([v, l]) => option(v, l, state.filters.get(k.name) || '')).join('')}</select>`;
    const tallies = shown.map((k) => {
      const t = tally(k, matched);
      return `<th class=k title='${esc(words(t))}'>${bar(t)}${t.pass + t.built} of ${matched.length}
        </th>`;
    }).join('');
    const order = (key) => (state.sort === key ? state.order : 'none');
    const sorted = (key) => (state.sort === key ? ` aria-sort=${state.order}` : '');
    const button = (key, label) => sortButton(label, order(key), ` data-sort='${esc(key)}'`);
    thead.innerHTML = `<tr><th class=case${sorted('case')}>case ${
      button('case', 'case')}</th>${shown.map((k) => `<th class=k${sorted(k.name)}>
      <a href='${kernelPage(k.name)}'>${esc(k.name)}</a> ${button(k.name, k.name)}<br>${
      select(k)}</th>`).join('')}</tr>
      <tr class=tally><th class='case muted'>passed or built</th>${tallies}</tr>`;
    frame.style.setProperty('--head', `${thead.rows[0].offsetHeight}px`);
  }

  // the rows in sight, and a spacer above and below for the rest
  let rowHeight = 48;
  let drawn = '';
  function body(force) {
    const top = frame.scrollTop - thead.offsetHeight;
    const first = Math.max(0, Math.floor(top / rowHeight) - 10);
    const last = Math.min(matched.length, Math.ceil((top + frame.clientHeight) / rowHeight) + 10);
    const key = `${first}:${last}`;
    if (!force && key === drawn) return;
    drawn = key;
    const span = shown.length + 1;
    const spacer = (rows) => (rows ? `<tr class=spacer aria-hidden=true><td colspan=${span}
      style='height:${rows * rowHeight}px'></td></tr>` : '');
    const rows = [];
    for (let j = first; j < last; j++) rows.push(row(b.cases[matched[j]]));
    tbody.innerHTML = spacer(first) + rows.join('') + spacer(matched.length - last);
    const one = tbody.querySelector('tr:not(.spacer)');
    if (one) {
      const h = one.getBoundingClientRect().height;
      if (h && Math.abs(h - rowHeight) > 0.5) { rowHeight = h; body(true); }
    }
  }

  function row(c) {
    const page = casePage(c.id);
    const thumb = c.drawn
      ? `<img class=thumb src='${thumbOf(c.id)}' alt='' loading=lazy>` : '<span class=thumb></span>';
    const cells = shown.map((k) => {
      const o = k.outcomes[c.i];
      if (o === 'missing') return `<td class='cell missing' title='${esc(k.name)}: no implementation yet'>–</td>`;
      const wa = k.workaround[c.i];
      const neat = k.elegance[c.i];
      const seconds = k.seconds[c.i];
      const mb = k.mb[c.i];
      const tip = [`${k.name}: ${o}`, wa && `workaround: ${wa}`, neat && `elegant: ${neat}`,
                   k.why[c.i],
                   seconds != null && secondsText(seconds),
                   mb != null && megabytesText(mb)].filter(Boolean).join(' · ');
      const shows = state.cells === 'time' ? secondsText(seconds)
        : state.cells === 'memory' ? megabytesText(mb) : o;
      return `<td class='cell ${o}' title='${esc(tip)}'><a href='${casePage(c.id, k.name)}'>${shows}${
        wa ? ' <span class=wa>†</span>' : ''}${neat ? ' <span class=wa>✦</span>' : ''}</a></td>`;
    }).join('');
    return `<tr><td class=case><div class=cc>${thumb}<div><a class=t href='${page}'
      title='${esc(c.title)}'>${esc(c.title)}</a><small>${esc(c.id)}${
      c.settled ? '' : ' · unsettled'}</small></div></div></td>${cells}</tr>`;
  }

  function update() {
    filter();
    chips();
    head();
    found.textContent = `${matched.length} of ${b.cases.length} cases`;
    drawn = '';
    body(true);
    address();
  }

  let typing = 0;
  search.addEventListener('input', () => {
    clearTimeout(typing);
    typing = setTimeout(() => { state.q = search.value.trim(); frame.scrollTop = 0; update(); }, 120);
  });
  main.addEventListener('change', (event) => {
    const el = event.target;
    if (el.dataset.add === 'cat' && el.value) state.cats.add(el.value);
    else if (el.dataset.add === 'tag' && el.value) state.tags.add(el.value);
    else if (el.name === 'settled') state.settled = el.value;
    else if (el.name === 'view') state.view = el.value;
    else if (el.name === 'cells') state.cells = el.value;
    else if (el.dataset.filter) state.filters.set(el.dataset.filter, el.value);
    else return;
    if (el.dataset.add) el.value = '';
    frame.scrollTop = 0;
    update();
  });
  main.addEventListener('click', (event) => {
    const el = event.target.closest('button');
    if (!el) return;
    if (el.dataset.kernel) {
      const name = el.dataset.kernel;
      const on = state.shown.includes(name);
      if (on && state.shown.length === 1) return;
      state.shown = on ? state.shown.filter((n) => n !== name)
        : b.ran.map((k) => k.name).filter((n) => n === name || state.shown.includes(n));
      el.setAttribute('aria-pressed', String(!on));
    } else if (el.dataset.sort) {
      const now = state.sort === el.dataset.sort ? state.order : 'none';
      const then = next(now);
      state.sort = then === 'none' ? '' : el.dataset.sort;
      state.order = then === 'none' ? 'ascending' : then;
      frame.scrollTop = 0;
    } else if (el.dataset.remove === 'cat') state.cats.delete(el.dataset.value);
    else if (el.dataset.remove === 'tag') state.tags.delete(el.dataset.value);
    else if (el.dataset.remove === 'filter') state.filters.delete(el.dataset.value);
    else return;
    update();
  });
  frame.addEventListener('scroll', () => requestAnimationFrame(() => body(false)));
  new ResizeObserver(() => body(false)).observe(frame);
  update();
}
