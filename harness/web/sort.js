// Sorting the board's tables.  A click on a column's head (or its arrow) sorts the rows by that
// column: ascending, then descending, then back to the order they were drawn in.  Blank cells
// ('', '–', '·') go last either way, and rows that tie keep their drawn order.
//
// A cell sorts by its `data-v` if it has one, else an outcome cell (class `cell pass`, …) by
// the outcomes' order, else by its text: as numbers where every cell of the column reads as one
// (with a unit: 420 ms, 1.2 GB, 2.3×, 45%, "12 of 220"), else as words.
//
// A table opts out with `data-nosort`; a row is pinned where it is drawn with `data-fixed`.  A
// table whose heads run down its first column (`data-sort=columns`, a case page's entries) sorts
// its columns instead, by a click on a row's head.  A cell spanning rows (rowspan) is repeated in
// each row it spans, and shown once again wherever the same value runs on.
import { OUTCOMES } from './board.js';

const RANK = Object.fromEntries(OUTCOMES.map((o, i) => [o, i]));
const UNIT = { ms: 1e-3, s: 1, kb: 1e-3, mb: 1, gb: 1e3, '×': 1, '%': 1 };
const NUMBER = /^(<\s*)?(-?\d[\d,]*(?:\.\d+)?(?:e[-+]?\d+)?)\s*(ms|s|kb|mb|gb|×|%)?(?![a-z])/i;
const collator = new Intl.Collator(undefined, { numeric: true, sensitivity: 'base' });

const ARROW = { none: '↕', ascending: '↑', descending: '↓' };

/** The arrow a sortable head carries; `label` names what it sorts by. */
export const sortButton = (label, state = 'none', attrs = '') => `<button type=button class=sort${
  attrs} aria-label='Sort by ${label.replace(/'/g, '&#39;')}'>${ARROW[state]}</button>`;

/** Compares two sort keys (numbers or strings, null for none) in direction `dir` (1 or -1),
 * nulls last whichever the direction. */
export function compare(a, b, dir) {
  if (a == null || b == null) return (a == null) - (b == null);
  const c = typeof a === 'number' && typeof b === 'number' ? a - b : collator.compare(String(a), String(b));
  return dir * c;
}

/** The order the next click on a head sorted `now` gives. */
export const next = (now) => ({ none: 'ascending', ascending: 'descending' }[now] || 'none');

// A cell's raw key: its data-v, its outcome's rank, or its text ('' for none).
function raw(cell) {
  if (!cell) return '';
  if (cell.dataset.v !== undefined) {
    const v = cell.dataset.v;
    return v !== '' && !Number.isNaN(Number(v)) ? Number(v) : v;
  }
  if (cell.classList.contains('cell')) {
    for (const c of cell.classList) if (c in RANK) return c === 'missing' ? '' : RANK[c];
  }
  return cell.textContent.replace(/\s+/g, ' ').trim();
}

// A text as a number, with its unit, or null if it does not read as one.
function number(text) {
  const m = NUMBER.exec(text);
  if (!m) return null;
  let x = Number(m[2].replace(/,/g, '')) * (UNIT[(m[3] || '').toLowerCase()] ?? 1);
  if (m[1]) x /= 2;  // '<1 ms': less than the least it says
  return x;
}

// The keys of a line of cells: numbers where every one that is not blank reads as a number.
function keys(cells) {
  const raws = cells.map(raw).map((r) => (r === '' || r === '–' || r === '·' ? null : r));
  const nums = raws.map((r) => (typeof r === 'string' ? number(r) : r));
  const numeric = raws.every((r, i) => r == null || nums[i] != null);
  return numeric ? nums : raws.map((r) => (r == null ? null : String(r)));
}

// The cells of each row by column, each spanned column given its cell (rowspans repeated).
function grid(rows) {
  const out = rows.map(() => []);
  rows.forEach((row, r) => {
    let col = 0;
    for (const cell of row.cells) {
      while (out[r][col]) col++;
      for (let dr = 0; dr < cell.rowSpan && r + dr < rows.length; dr++) {
        for (let dc = 0; dc < cell.colSpan; dc++) out[r + dr][col + dc] = cell;
      }
      col += cell.colSpan;
    }
  });
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
  let current = null;
  cells.forEach((th, i) => {
    if (!th || !th.textContent.trim()) return;
    th.classList.add('sortable');
    const label = th.textContent.replace(/\s+/g, ' ').trim();
    th.insertAdjacentHTML('beforeend', ` ${sortButton(label)}`);
    const button = th.lastElementChild;
    const click = () => {
      const state = next(current === th ? th.getAttribute('aria-sort') || 'none' : 'none');
      if (current && current !== th) {
        current.removeAttribute('aria-sort');
        current.querySelector('button.sort').textContent = ARROW.none;
      }
      current = state === 'none' ? null : th;
      if (state === 'none') th.removeAttribute('aria-sort');
      else th.setAttribute('aria-sort', state);
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

// A table of rows under a row of heads: its rows sorted.
function rows(table) {
  const all = [...table.rows];
  const head = all.findIndex((row) => [...row.cells].every((c) => c.tagName === 'TH' && c.scope !== 'row'));
  if (head < 0) return;
  const body = all.slice(head + 1);
  if (body.filter((r) => !r.hasAttribute('data-fixed')).length < 2) return;
  // a cell spanning rows repeated in each, marked to show once where its value runs on
  const spanned = new Set();
  const cells = grid(body);
  body.forEach((row, r) => {
    cells[r].forEach((cell, c) => {
      if (!cell || cell.rowSpan === 1) return;
      spanned.add(c);
      if (cell.parentElement !== row) {
        const copy = cell.cloneNode(true);
        copy.rowSpan = 1;
        const after = cells[r].slice(0, c).reverse().find((x) => x && x.parentElement === row);
        if (after) after.after(copy);
        else row.prepend(copy);
        cells[r][c] = copy;
      }
    });
  });
  for (const r of cells) for (const c of spanned) r[c].rowSpan = 1;
  const columns = cells[0] ? Math.max(...cells.map((r) => r.length)) : 0;
  const line = (c) => keys(cells.map((r) => (r[c] && r[c].colSpan === 1 ? r[c] : null)));
  const memo = new Map();
  const drawn = body.map((row, r) => ({ row, r }));
  const runs = () => {
    for (const c of spanned) {
      const order = [...table.rows].slice(head + 1);
      const text = order.map((row) => cells[body.indexOf(row)][c].textContent.trim());
      order.forEach((row, j) => {
        const cell = cells[body.indexOf(row)][c];
        cell.classList.toggle('same', j > 0 && text[j] === text[j - 1]);
        cell.classList.toggle('runs', j + 1 < order.length && text[j] === text[j + 1]);
      });
    }
  };
  runs();
  const headCells = grid([all[head]])[0];
  heads(table, headCells.slice(0, columns).map((th, i) => (th && headCells.indexOf(th) === i ? th : null)),
    (c, state) => {
      if (!memo.has(c)) memo.set(c, line(c));
      const k = memo.get(c);
      const dir = state === 'descending' ? -1 : 1;
      const order = state === 'none' ? drawn
        : [...drawn].sort((a, b) => compare(k[a.r], k[b.r], dir));
      const parent = body[0].parentElement;
      const fixed = order.filter((x) => x.row.hasAttribute('data-fixed'));
      for (const { row } of [...fixed, ...order.filter((x) => !fixed.includes(x))]) parent.append(row);
      runs();
    });
}

// A table whose heads run down its first column: its columns sorted, the first kept in place.
function columns(table) {
  const all = [...table.rows];
  const n = Math.min(...all.map((r) => r.cells.length));
  if (n < 3) return;
  const drawn = Array.from({ length: n - 1 }, (_, j) => j + 1);
  const cells = all.map((r) => [...r.cells]);
  heads(table, all.map((r) => (r.cells[0].tagName === 'TH' ? r.cells[0] : null)), (r, state) => {
    const k = [null, ...keys(drawn.map((j) => cells[r][j]))];
    const dir = state === 'descending' ? -1 : 1;
    const order = state === 'none' ? drawn : [...drawn].sort((a, b) => compare(k[a], k[b], dir));
    all.forEach((row, i) => { for (const j of order) row.append(cells[i][j]); });
  });
}
