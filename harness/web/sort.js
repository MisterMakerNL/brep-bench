// Sorting the board's tables.  A click on a column's head (or its arrow) sorts the rows by that
// column: ascending, then descending, then back to the order they were drawn in.  Blank cells
// ('', '–', '·') go last either way, and rows that tie keep their drawn order.
//
// A cell sorts by its `data-v` if it has one (a number cell is drawn with its value there, its
// text being rounded and in units), else an outcome cell (class `cell pass`, …) by the outcomes'
// order, else by its text: as numbers where every cell of the column is one, else as words.
//
// A table opts out with `data-nosort`; a row is pinned where it is drawn with `data-fixed`.  A
// table whose heads run down its first column (`data-sort=columns`, a case page's entries) sorts
// its columns instead, by a click on a row's head.
import { OUTCOMES, esc } from './board.js';

const RANK = Object.fromEntries(OUTCOMES.map((o, i) => [o, i]));
const collator = new Intl.Collator(undefined, { numeric: true, sensitivity: 'base' });
const ARROW = { none: '↕', ascending: '↑', descending: '↓' };

/** An outcome's sort key: its place in the outcomes' order, or null (blank) for missing. */
export const rank = (o) => (o === 'missing' ? null : RANK[o]);

/** The arrow a sortable head carries; `label` names what it sorts by. */
export const sortButton = (label, state = 'none', attrs = '') =>
  `<button type=button class=sort${attrs} aria-label='Sort by ${esc(label)}'>${ARROW[state]}</button>`;

/** Compares two sort keys (numbers or strings, null for none) in direction `dir` (1 or -1),
 * nulls last whichever the direction. */
export function compare(a, b, dir) {
  if (a == null || b == null) return (a == null) - (b == null);
  const c = typeof a === 'number' && typeof b === 'number' ? a - b : collator.compare(String(a), String(b));
  return dir * c;
}

/** The order the next click on a head sorted `now` gives. */
export const next = (now) => ({ none: 'ascending', ascending: 'descending' }[now] || 'none');

// A cell's key: its data-v, its outcome's rank, or its text; null where blank.
function raw(cell) {
  if (!cell) return null;
  let v = cell.dataset.v;
  if (v === undefined && cell.classList.contains('cell')) {
    const o = [...cell.classList].find((c) => c in RANK);
    if (o) return rank(o);
  }
  v ??= cell.textContent.replace(/\s+/g, ' ').trim();
  return v === '' || v === '–' || v === '·' ? null : v;
}

// The keys of a line of cells: numbers where every one that is not blank is a number.
function keys(cells) {
  const raws = cells.map(raw);
  const numeric = raws.every((r) => r == null || typeof r === 'number' || !Number.isNaN(Number(r)));
  return raws.map((r) => (r == null ? null : numeric ? Number(r) : String(r)));
}

// A row's cells by column; a cell spanning columns (an overview row's "awaiting an
// implementation") is in none of them, a blank in each.
function byColumn(row) {
  const out = [];
  for (const cell of row.cells) {
    if (cell.colSpan === 1) out[out.length] = cell;
    else out.length += cell.colSpan;
  }
  return out;
}

/** Makes every table under `root` sortable, but those with data-nosort and those already so. */
export function sortable(root = document) {
  for (const table of root.querySelectorAll('table:not([data-nosort]):not([data-sorting])')) {
    if (table.dataset.sort === 'columns') columns(table);
    else rows(table);
  }
}

// Wires each head to sort by its line; `apply(i, state)` sorts by line i in that order.
function heads(table, cells, apply) {
  cells.forEach((th, i) => {
    if (!th || !th.textContent.trim()) return;
    th.classList.add('sortable');
    th.insertAdjacentHTML('beforeend', ` ${sortButton(th.textContent.replace(/\s+/g, ' ').trim())}`);
    const button = th.lastElementChild;
    const click = () => {
      const state = next(th.getAttribute('aria-sort') || 'none');
      for (const other of table.querySelectorAll('th[aria-sort]')) {
        other.removeAttribute('aria-sort');
        other.querySelector('button.sort').textContent = ARROW.none;
      }
      if (state !== 'none') th.setAttribute('aria-sort', state);
      button.textContent = ARROW[state];
      apply(i, state);
    };
    button.addEventListener('click', click);
    // the head itself, but not a link or a control in it
    th.addEventListener('click', (event) => {
      if (!event.target.closest('a, button, select, input')) click();
    });
  });
  table.dataset.sorting = '';
}

// A table of rows under a row of heads: its rows sorted, its pinned ones kept first.
function rows(table) {
  const all = [...table.rows];
  const head = all.findIndex((row) => [...row.cells].every((c) => c.tagName === 'TH' && c.scope !== 'row'));
  if (head < 0) return;
  const drawn = all.slice(head + 1).map((row, r) => ({ row, r, cells: byColumn(row),
                                                      fixed: row.hasAttribute('data-fixed') }));
  if (drawn.filter((x) => !x.fixed).length < 2) return;
  const memo = new Map();
  heads(table, byColumn(all[head]), (c, state) => {
    if (!memo.has(c)) memo.set(c, keys(drawn.map((x) => x.cells[c])));
    const k = memo.get(c);
    const dir = state === 'descending' ? -1 : 1;
    const order = state === 'none' ? drawn : [...drawn].sort((a, b) => compare(k[a.r], k[b.r], dir));
    drawn[0].row.parentElement.append(...order.filter((x) => x.fixed).map((x) => x.row),
                                      ...order.filter((x) => !x.fixed).map((x) => x.row));
  });
}

// A table whose heads run down its first column: its columns sorted, the first kept in place.
function columns(table) {
  const cells = [...table.rows].map((r) => [...r.cells]);
  const n = Math.min(...cells.map((r) => r.length));
  if (n < 3) return;
  const drawn = Array.from({ length: n - 1 }, (_, j) => j + 1);
  heads(table, cells.map((r) => (r[0].tagName === 'TH' ? r[0] : null)), (r, state) => {
    const k = keys(cells[r]);
    const dir = state === 'descending' ? -1 : 1;
    const order = state === 'none' ? drawn : [...drawn].sort((a, b) => compare(k[a], k[b], dir));
    cells.forEach((row, i) => table.rows[i].append(...order.map((j) => row[j])));
  });
}
