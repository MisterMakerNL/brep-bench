#!/usr/bin/env python3
"""Render the results as a static site (site/):

- board.json: every case and every kernel's outcome of it, in one file (its form is `board`'s);
- the board's pages, harness/web/ copied as they are: the overview (index.html), the explorer of
  every case (explore.html) and a kernel's page (kernel.html?k=NAME), which read board.json;
- a page for each case (cases/<case>.html), written here: its description, its closed forms, its
  drawing, a table of every kernel's entry at a glance (outcome, errors, time and lines of code),
  and the entry picked (#k=NAME in the address picks one): its result in 3D, its time, its notes,
  its measures and its STL file, and its implementation, the code highlighted
  (harness/highlight.py).

Kernels are listed in kernels.json's order, everywhere: the board ranks nothing. Run
harness/draw.py first for the drawings. Standard library only."""
import html
import json
import os
import re
import shutil
import sys
import tomllib
import urllib.parse

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from highlight import highlight, loc  # noqa: E402
from run import FROM_MESH  # noqa: E402
from suite import ROOT, cases as read_cases, kernels as read_kernels, source  # noqa: E402
SITE = os.path.join(ROOT, "site")
REPO = "https://github.com/bmander/brep-bench"
WEB = os.path.join(ROOT, "harness", "web")
OUTCOMES = ["pass", "built", "refused", "wrong", "crashed", "timeout", "missing"]
# an outcome's letter in board.json
CODES = {"pass": "p", "built": "b", "refused": "r", "wrong": "w", "crashed": "c", "timeout": "t",
         "missing": "."}
THREE = "https://cdn.jsdelivr.net/npm/three@0.160.0"
KATEX = "https://cdn.jsdelivr.net/npm/katex@0.16.11/dist"

def head(title, description, depth):
    up = "../" * depth
    return [
        "<!doctype html><html lang=en><head><meta charset=utf-8>",
        "<meta name=viewport content='width=device-width, initial-scale=1'>",
        f"<title>{html.escape(title)}</title>",
        f"<meta name=description content='{html.escape(description)}'>",
        f"<link rel=stylesheet href='{up}style.css'>",
        f"<link rel=icon href='{up}favicon.ico' sizes=32x32>",
        f"<link rel=icon href='{up}favicon.svg' type='image/svg+xml'>",
        f"<link rel=apple-touch-icon href='{up}apple-touch-icon.png'></head><body>",
        f"<nav class=top><a href='{up}index.html'>brep-bench</a>"
        f"<a href='{up}explore.html'>Cases</a><a href='{up}performance.html'>Performance</a>"
        f"<a href='{REPO}'>Source</a></nav>",
    ]


def load():
    registry = {k["name"]: k for k in read_kernels()}
    results = []
    rdir = os.path.join(ROOT, "results")
    for name in sorted(os.listdir(rdir)) if os.path.isdir(rdir) else []:
        if name.endswith(".json"):
            with open(os.path.join(rdir, name)) as f:
                r = json.load(f)
            r.setdefault("kernel", r.get("adapter"))
            # a kernel taken out of kernels.json is off the board, whatever results it left
            # (the Actions cache keeps them)
            if r["kernel"] not in registry:
                continue
            # each case's row by id, and what the kernel's implementations work around, read once
            r["rows"] = {c["case"]: c for c in r["cases"]}
            r["workarounds"] = workarounds(registry.get(r["kernel"], {"name": r["kernel"]}))
            results.append(r)
    # kernels.json's order: the board ranks nothing
    order = {name: i for i, name in enumerate(registry)}
    results.sort(key=lambda r: order[r["kernel"]])
    cases = {c["id"]: c for c in read_cases()}
    drawings = {}
    index = os.path.join(SITE, "drawings", "index.json")
    if os.path.exists(index):
        with open(index) as f:
            drawings = json.load(f)
    return registry, results, cases, drawings


def workarounds(kernel):
    """What each of `kernel`'s implementations works around, by case id: its workarounds.toml
    beside its runner (SPEC.md), or nothing."""
    path = os.path.join(ROOT, "kernels", kernel.get("name", ""), "workarounds.toml")
    if not os.path.exists(path):
        return {}
    with open(path, "rb") as f:
        return tomllib.load(f)


def paragraphs(text):
    """A description's paragraphs as HTML: blank lines part them, `code` is code."""
    out = []
    for para in re.split(r"\n\s*\n", text.strip()):
        h = html.escape(" ".join(para.split()))
        out.append("<p>" + re.sub(r"`([^`]+)`", r"<code>\1</code>", h) + "</p>")
    return "\n".join(out)


def board(registry, results, cases, drawings):
    """Everything the board's pages show, as one JSON object:

    - `kernels`: every kernel in kernels.json, in its order, with what kernels.json says of it
      and, where it has results, its `version` and `run`;
    - `cases`: every case in id order, each `[id, title, [tag], settled, drawn]`, a tag by its
      index in `tags`, `settled` and `drawn` 1 or 0;
    - `results`: by kernel, its outcome of each case in `cases`' order, as a string of one letter
      a case (`codes`), with lists in the same order of what each run cost (null where it has no
      row, or the run was not measured so): its wall-clock `seconds`, its `cpu` seconds, its
      peak memory in megabytes (`mb`) and its wall-clock seconds on one core alone (`one`); and
      of its `why` and its `workaround` (indices in `reasons` and `workarounds`, -1 for none).
      A kernel's `host` is the machine its last run was made on, and its `startup` what starting
      its runner cost there, building nothing (`seconds`, `cpu`, `mb`, and `one` on one core);
      its `binary` is the size of its binary (`bytes`, `files`), and `of` what kernels.json counts
      in it.
    """
    ids = sorted(cases)
    tags = sorted({t for c in cases.values() for t in c["tags"]})
    tag_ix = {t: i for i, t in enumerate(tags)}
    strings = {"reasons": {}, "workarounds": {}}

    def intern(table, text):
        if not text:
            return -1
        return strings[table].setdefault(text, len(strings[table]))

    kernels = []
    out = {}
    ran = {r["kernel"]: r for r in results}
    for name in registry:
        a = registry[name]
        k = {"name": name}
        for field, key in (("kernel", "home"), ("license", "license"), ("backend", "backend"),
                           ("frontend", "frontend"), ("note", "note"),
                           ("reference", "reference")):
            if a.get(field):
                k[key] = a[field]
        r = ran.get(name)
        if r:
            k["version"] = r.get("version") or ""
            k["run"] = r.get("run") or ""
            k["measured"] = r.get("measured") or a.get("measured") or "exact"
            if r.get("host"):
                k["host"] = r["host"]
            if r.get("binary"):
                k["binary"] = r["binary"]
            start = r.get("startup")
            if start:
                k["startup"] = {key: start.get(field) for key, field in COSTS.items()}
            codes, why, worked = [], [], []
            spent = {key: [] for key in COSTS}
            for cid in ids:
                row = r["rows"].get(cid)
                if row is None:
                    codes.append(CODES["missing"])
                    for key in COSTS:
                        spent[key].append(None)
                    why.append(-1)
                    worked.append(-1)
                    continue
                codes.append(CODES[row["outcome"]])
                # a missing case has no run, whatever an older row says of its time
                for key, field in COSTS.items():
                    spent[key].append(None if row["outcome"] == "missing" else row.get(field))
                why.append(intern("reasons", row.get("why")))
                worked.append(intern("workarounds", r["workarounds"].get(cid)))
            out[name] = {"codes": "".join(codes), **spent, "why": why, "workaround": worked}
        elif not a.get("command"):
            k["awaiting"] = True
        kernels.append(k)
    return {
        "repo": REPO,
        "outcomes": {o: CODES[o] for o in OUTCOMES},
        "kernels": kernels,
        "tags": tags,
        "cases": [[cid, cases[cid]["title"], [tag_ix[t] for t in cases[cid]["tags"]],
                   int(cases[cid]["expect"] is not None), int(cid in drawings)] for cid in ids],
        "results": out,
        "reasons": list(strings["reasons"]),
        "workarounds": list(strings["workarounds"]),
    }


# what a run cost: each measure's name on the board, and the field of a result's row it is read from
COSTS = {"seconds": "seconds", "cpu": "cpu_seconds", "mb": "peak_mb", "one": "one_core_seconds"}


def cost(x, unit):
    """A run's measure for a table cell, or a dash where it was not measured."""
    return "<span class=muted>–</span>" if x is None else f"{x:g}{unit}"


def spent(en, field, unit):
    """An entry's measure of its run for the case page, or nothing where it has not run."""
    return "" if en["outcome"] == "missing" else cost(en["row"].get(field), unit)


def relative(got, want):
    if got is None:
        return ""
    return f"{abs(got - want) / max(abs(want), 1e-12):.1e}"


def case_page(c, registry, results, drawing, prev=None, after=None):
    """`prev` and `after`: the cases before and after this one in its category, or None."""
    e = html.escape
    q = urllib.parse.quote
    cid = c["id"]
    category = cid.rpartition("/")[0]
    depth = cid.count("/") + 1
    up = "../" * depth
    out = head(f"{c['title']} · brep-bench", f"The brep-bench case {cid}: its description, its "
               "drawing, and each kernel's entry: its result in 3D, its measures and its code.",
               depth)
    steps = [f"<a href='{up}explore.html?cat={q(category)}'>{e(category)}</a>"]
    if prev:
        steps.append(f"<a href='{up}cases/{e(prev)}.html' rel=prev>← "
                     f"{e(prev.rpartition('/')[2])}</a>")
    if after:
        steps.append(f"<a href='{up}cases/{e(after)}.html' rel=next>"
                     f"{e(after.rpartition('/')[2])} →</a>")
    out.append(f"<p class=muted>{' · '.join(steps)}</p>")
    tags = ", ".join(f"<a href='{up}explore.html?tag={q(t)}'>{e(t)}</a>" for t in c["tags"])
    out.append(f"<h1>{e(c['title'])}</h1><p class=muted>{e(cid)} · {tags}</p>")
    out.append(f"<div class=description>{paragraphs(c['description'])}</div>")
    exp = c["expect"]
    if exp is None:
        out.append("<p><strong>Unsettled.</strong> This solid has no closed form, and no two kernels "
                   "yet agree on it to 1e-8, so it has no answer to grade against. Below is what each "
                   "kernel builds: <em>built</em> means a watertight, consistently oriented solid, "
                   "whatever its measures; the table says which kernels agree with which.</p>")
    else:
        out.append("<div class=scroll><table><tr><th></th><th>expected</th>"
                   "<th>closed form</th></tr>")
        for key in ("volume", "area"):
            x = exp[key]
            # the closed form typeset by KaTeX, with the case's own words until it loads (and
            # where there is no closed form)
            form = (f"<td class=closed title='{e(x['exact'])}'><span class=tex "
                    f"data-tex='{e(x['tex'])}'>{e(x['exact'])}</span></td>" if x["tex"]
                    else f"<td class='muted why'>{e(x['exact'])}</td>")
            out.append(f"<tr><td>{key}</td><td>{x['value']:.10g}</td>{form}</tr>")
        out.append("</table></div>")


    if drawing:
        d = f"{up}drawings/{e(cid)}"
        sz = drawing["sizes"]
        def fig(view, label, cls=""):
            w, h = sz[view]
            if view == "iso":
                # shown as wide as the right view, so the sheet's columns line up
                w, h = sz["right"][0], round(h * sz["right"][0] / w)
            return (f"<figure class='{cls}'><img class=paper src='{d}/{view}.png' width={w} "
                    f"height={h} alt='{label} view'><figcaption>{label}</figcaption></figure>")
        out.append("<h2>Drawing</h2>")
        out.append("<div class=scroll><div class=sheet>" + fig("top", "Top", "up") +
                   fig("iso", "Isometric", "up") + fig("front", "Front") + fig("right", "Right") +
                   "</div></div>")
        out.append(f"<p class=muted>Third-angle projection, the three views at one scale. Drawn "
                   f"from {e(drawing['from'])}'s result, which passed.</p>")

    # an entry for each kernel with results, in kernels.json's order: its result, its triangles and
    # its code
    entries = []
    for r in results:
        name = r["kernel"]
        row = r["rows"].get(cid)
        stl = os.path.join(ROOT, "results", "meshes", name, cid + ".stl")
        href = mb = None
        if row and os.path.exists(stl):
            dest = os.path.join(SITE, "meshes", name, cid + ".stl")
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            shutil.copyfile(stl, dest)
            href = f"{up}meshes/{e(name)}/{e(cid)}.stl"
            mb = os.path.getsize(stl) / 1e6
        src = source(registry.get(name, {}), cid)
        code = None
        if src and os.path.exists(src):
            with open(src, encoding="utf-8") as f:
                code = f.read()
        o = row["outcome"] if row else "missing"
        wa = r["workarounds"].get(cid)
        entries.append({"name": name, "row": row, "outcome": o, "stl": href, "mb": mb, "wa": wa,
                        "src": src, "code": code, "loc": None if code is None else loc(code, src),
                        "label": "–" if o == "missing" else o + (" †" if wa else ""),
                        "measures": measures(row)})
    # which other results each agrees with
    for en in entries:
        en["friends"] = [o["name"] for o in entries if o is not en
                         and agree(en["measures"], o["measures"])]
    has_mesh = any(en["stl"] for en in entries)

    out.append("<h2 id=results>Results</h2>")
    if not entries:
        out.append("<p class=muted>No kernel has run it yet.</p>")
    else:
        # the entry shown first: a pass with triangles to show, else any result with them, else
        # any result at all (the first such, in kernels.json's order); #k=NAME picks another
        shown = max(entries, key=lambda en: (en["outcome"] == "pass" and bool(en["stl"]),
                                             bool(en["stl"]), bool(en["row"])))

        # every entry at a glance, a column each; a click on any of its cells picks it
        def value(en, i):
            m = en["measures"]
            if m is None:
                return ""
            return relative(m[i], exp[KEYS[i]]["value"]) if exp else f"{m[i]:.6g}"
        rows = [
            ("<th></th>", "th", lambda en: "k", lambda en: e(en["name"])),
            ("<th scope=row>outcome</th>", "td", lambda en: f"cell {en['outcome']}",
             lambda en: e(en["label"])),
            ("<th scope=row title='relative error'>volume err.</th>" if exp
             else "<th scope=row>volume</th>", "td", None, lambda en: value(en, 0)),
            ("<th scope=row title='relative error'>area err.</th>" if exp
             else "<th scope=row>area</th>", "td", None, lambda en: value(en, 1)),
            ("<th scope=row>time</th>", "td", None, lambda en: spent(en, "seconds", " s")),
            ("<th scope=row title='user and system time, over every thread'>CPU time</th>", "td",
             None, lambda en: spent(en, "cpu_seconds", " s")),
            ("<th scope=row title='the most memory the runner held resident at once'>memory</th>",
             "td", None, lambda en: spent(en, "peak_mb", " MB")),
            ("<th scope=row title='lines of code, comments and blank lines left out'>LOC</th>",
             "td", None, lambda en: "" if en["loc"] is None else str(en["loc"])),
        ]
        out.append(f"<div class=scroll><table class=entries data-default='{e(shown['name'])}'>")
        for head_cell, tag, cls, body in rows:
            out.append("<tr>" + head_cell + "".join(
                f"<{tag} class='{cls(en) if cls else ''}' data-k='{e(en['name'])}'>"
                f"<a href='#k={e(q(en['name']))}'>{body(en)}</a></{tag}>" for en in entries) +
                "</tr>")
        out.append("</table></div>")
        out.append("<p class=muted>† works around a missing feature. Pick an entry to see its "
                   "solid, its measures and its code.</p>")

        # the entry picked (the page's script shows it): its solid in 3D, its time, its notes,
        # its data and its code
        out.append("<div class=pick role=group aria-label='Whose entry'>" + "".join(
            f"<button type=button data-k='{e(en['name'])}'>{e(en['name'])} "
            f"<span class=muted>{e(en['label'])}</span></button>" for en in entries) + "</div>")
        if has_mesh:
            out.append("<div id=view hidden><canvas id=viewer></canvas>"
                       "<p class=muted>Drag to turn, scroll or pinch to zoom, right-drag to move. "
                       "Black lines are where the surface turns sharply; red ones, seen through "
                       "the solid, are open edges, where the mesh is not closed.</p></div>")
        out += [entry(en, exp) for en in entries]
        out.append(PICK)

    out.append(f"<p class=muted><a href='{REPO}/blob/main/cases/{e(cid)}.md'>{e(cid)}.md</a>, "
               "the case as written.</p>")
    if has_mesh:
        out.append(VIEWER.replace("{cdn}", THREE))
    if exp and any(exp[k]["tex"] for k in exp):
        out.append(TEX.replace("{cdn}", KATEX))
    out.append("</body></html>")
    path = os.path.join(SITE, "cases", cid + ".html")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write("\n".join(out))


KEYS = ("volume", "area")


def measures(row):
    """A result's (volume, area, from its mesh), or None where it built no solid to measure."""
    if not row or row["outcome"] not in ("pass", "built", "wrong"):
        return None
    from_mesh = row.get("measured") == "mesh"
    m = row.get("mesh") or {}
    vol = m.get("volume") if from_mesh else row.get("volume")
    area = m.get("area") if from_mesh else row.get("area")
    return None if vol is None or area is None else (vol, area, from_mesh)


def agree(a, b):
    """Whether two results' measures agree: to 1e-8 between exact kernels, to FROM_MESH where
    either was measured from its mesh."""
    if a is None or b is None:
        return False
    tol = FROM_MESH if a[2] or b[2] else 1e-8
    return all(abs(x - y) <= tol * max(abs(x), abs(y), 1e-12) for x, y in zip(a[:2], b[:2]))


# what each outcome means, said where a result gives no reason of its own
MEANS = {
    "pass": "Its volume and area are the expected ones, and its mesh is closed.",
    "built": "A watertight, consistently oriented solid; the case has no settled answer yet to "
             "grade it against.",
    "refused": "The kernel declined to build it.",
    "wrong": "It built a solid, but not the expected one.",
    "crashed": "The runner crashed.",
    "timeout": "The runner ran out of time.",
    "missing": "No result: the kernel has no implementation of this case yet.",
}


def entry(en, exp):
    """One kernel's entry for a case, as HTML, hidden: the page's script shows the one picked.
    Its triangles are drawn by the page's one viewer, above it."""
    e = html.escape
    name, row, o = en["name"], en["row"], en["outcome"]
    out = [f"<section class=entry data-k='{e(name)}' data-src='{en['stl'] or ''}' hidden "
           f"aria-label='{e(name)}'>"]
    if not en["stl"]:
        out.append(f"<div class='muted nomesh'>{e(name)} left no triangles to show.</div>")

    out.append("<h3>Time</h3>")
    out.append(f"<p>{spent(en, 'seconds', ' s')}; CPU time {spent(en, 'cpu_seconds', ' s')}; "
               f"peak memory {spent(en, 'peak_mb', ' MB')}</p>" if o != "missing"
               else "<p class=muted>It has not run.</p>")

    out.append("<h3>Notes</h3>")
    why = row.get("why") if row else ""
    out.append(f"<p><span class='badge {o}'>{o}</span> {e(why or MEANS[o])}</p>")
    if en["wa"]:
        out.append(f"<p><strong>Workaround †</strong> {e(en['wa'])}</p>")
    if row and row.get("measured") == "mesh":
        out.append("<p class=muted>It has no exact measures: its volume and area are its "
                   f"triangles', graded to {FROM_MESH:g}.</p>")

    out.append("<h3>Data</h3>")
    facts = []
    if en["measures"]:
        *got, from_mesh = en["measures"]
        whose = "its triangles'" if from_mesh else "measured"
        table = [f"<tr><th></th><th>{whose}</th>"
                 + ("<th>expected</th><th>rel. error</th>" if exp else "") + "</tr>"]
        for key, x in zip(KEYS, got):
            cells = f"<td>{x:.10g}</td>" if exp else f"<td>{x:.12g}</td>"
            if exp:
                want = exp[key]["value"]
                cells += f"<td>{want:.10g}</td><td>{relative(x, want)}</td>"
            table.append(f"<tr><td>{key}</td>{cells}</tr>")
        out.append("<div class=scroll><table>" + "".join(table) + "</table></div>")
        facts.append(("agrees with", e(", ".join(en["friends"])) or
                      "<span class=muted>no other kernel</span>"))
    elif row:
        out.append("<p class=muted>No measures: it built no solid to measure.</p>")
    faces = (row or {}).get("faces")
    if faces:
        facts.append(("faces", e(", ".join(f"{n} {k}" for k, n in faces.items()))))
    m = (row or {}).get("mesh")
    if m:
        bits = [f"{m['triangles']} triangles",
                "closed" if m.get("closed") else
                f"open: {m.get('unmatched_edges', 0)} edges unmatched"]
        if m.get("degenerate"):
            bits.append(f"{m['degenerate']} degenerate")
        facts.append(("mesh", ", ".join(bits)))
    if en["stl"]:
        size = f"{en['mb']:.1f} MB" if en["mb"] >= 0.1 else f"{en['mb'] * 1e3:.0f} kB"
        facts.append(("STL", f"<a href='{en['stl']}' download>download</a> "
                             f"<span class=muted>{size}</span>"))
    if facts:
        out.append("<table class=facts>" + "".join(
            f"<tr><th scope=row>{k}</th><td>{v}</td></tr>" for k, v in facts) + "</table>")
    elif not row:
        out.append("<p class=muted>None.</p>")

    out.append("<h3>Implementation</h3>")
    src, code, n = en["src"], en["code"], en["loc"]
    if code is None:
        out.append("<p class=muted>No implementation yet.</p>")
    else:
        rel = os.path.relpath(src, ROOT)
        out.append(f"<p class=muted><a href='{REPO}/blob/main/{e(rel)}'>{e(rel)}</a> · {n} "
                   f"line{'' if n == 1 else 's'} of code, comments left out</p>"
                   f"<pre class=code>{highlight(code, src)}</pre>")
    out.append("</section>")
    return "\n".join(out)


# shows the entry picked, by a click on the table of entries or the buttons, or the address's
# #k=NAME, else the table's default; marks it picked (aria-current) and tells the viewer its
# triangles (an `entry` event, its detail their path, '' for none)
PICK = """<script>
(() => {
  const entries = [...document.querySelectorAll('section.entry')];
  const marks = [...document.querySelectorAll('[data-k]:not(section)')];
  const view = document.getElementById('view');
  function pick(k) {
    const entry = entries.find((x) => x.dataset.k === k);
    if (!entry) return false;
    for (const x of entries) x.hidden = x !== entry;
    for (const m of marks) m.setAttribute('aria-current', String(m.dataset.k === k));
    if (view) view.hidden = !entry.dataset.src;
    document.dispatchEvent(new CustomEvent('entry', { detail: entry.dataset.src }));
    return true;
  }
  const asked = () => new URLSearchParams(location.hash.slice(1)).get('k');
  for (const m of marks) {
    m.addEventListener('click', (event) => {
      event.preventDefault();
      history.replaceState(null, '', '#k=' + encodeURIComponent(m.dataset.k));
      pick(m.dataset.k);
    });
  }
  addEventListener('hashchange', () => pick(asked()));
  // arriving for one kernel's result (from the explorer, say): show it, and scroll to it
  if (pick(asked())) document.getElementById('results').scrollIntoView();
  else pick(document.querySelector('table.entries').dataset.default);
})();
</script>"""


TEX = """<link rel=stylesheet href='{cdn}/katex.min.css'>
<script src='{cdn}/katex.min.js'></script>
<script>
if (window.katex) {
  for (const el of document.querySelectorAll('.tex')) {
    katex.render(el.dataset.tex, el, { displayMode: true, throwOnError: false });
  }
}
</script>
"""

VIEWER = """<script type=importmap>
{"imports": {"three": "{cdn}/build/three.module.js", "three/addons/": "{cdn}/examples/jsm/"}}
</script>
<script type=module>
import * as THREE from 'three';
import { STLLoader } from 'three/addons/loaders/STLLoader.js';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

const canvas = document.getElementById('viewer');
const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(30, 1, 0.01, 1e6);
camera.up.set(0, 0, 1);
const controls = new OrbitControls(camera, canvas);
controls.enableDamping = true;
scene.add(new THREE.HemisphereLight(0xffffff, 0x8890a0, 2.2));
const sun = new THREE.DirectionalLight(0xffffff, 1.6);
scene.add(sun);
const dark = matchMedia('(prefers-color-scheme: dark)');
let shown = null;
const loader = new STLLoader();
const cache = new Map();

function resize() {
  const w = canvas.clientWidth, h = canvas.clientHeight;
  if (!w || !h) return;  // hidden: the entry picked left no triangles
  renderer.setSize(w, h, false);
  camera.aspect = w / h;
  camera.updateProjectionMatrix();
}

// The edges a single triangle uses (vertices joined where their coordinates are equal, as the
// harness joins them): where the mesh is not closed. Drawn red, thick, over everything.
function openEdges(geometry) {
  const p = geometry.attributes.position.array;
  const ids = new Map();
  const id = (k) => {
    const key = p[k] + ',' + p[k + 1] + ',' + p[k + 2];
    let v = ids.get(key);
    if (v === undefined) { v = ids.size; ids.set(key, v); }
    return v;
  };
  const uses = new Map();
  const ends = new Map();
  for (let t = 0; t < p.length; t += 9) {
    const v = [id(t), id(t + 3), id(t + 6)];
    for (let j = 0; j < 3; j++) {
      const a = v[j], b = v[(j + 1) % 3];
      if (a === b) continue;
      const key = a + '>' + b;
      uses.set(key, (uses.get(key) || 0) + 1);
      ends.set(key, [t + 3 * j, t + 3 * ((j + 1) % 3)]);
    }
  }
  const out = [];
  for (const [key, n] of uses) {
    const [a, b] = key.split('>');
    if (n === 1 && uses.get(b + '>' + a) === 1) continue;
    const [i, j] = ends.get(key);
    out.push(p[i], p[i + 1], p[i + 2], p[j], p[j + 1], p[j + 2]);
  }
  if (!out.length) return null;
  const g = new THREE.BufferGeometry();
  g.setAttribute('position', new THREE.Float32BufferAttribute(out, 3));
  const m = new THREE.LineBasicMaterial({ color: 0xe0362c, depthTest: false });
  const lines = new THREE.LineSegments(g, m);
  lines.renderOrder = 2;
  // and a dot at each, a fixed size on the screen: an open edge is often a sliver too thin to see
  const dots = new THREE.Points(g, new THREE.PointsMaterial({
    color: 0xe0362c, size: 7, sizeAttenuation: false, depthTest: false }));
  dots.renderOrder = 3;
  const both = new THREE.Group();
  both.add(lines, dots);
  return both;
}

// each solid, built once: its surface, its creases and its open edges
const built = new Map();
function build(src) {
  if (!built.has(src)) {
    built.set(src, new Promise((ok, no) => loader.load(src, ok, undefined, no)).then((geometry) => {
      geometry.computeVertexNormals();
      geometry.computeBoundingSphere();
      const solid = new THREE.Mesh(geometry, new THREE.MeshStandardMaterial({
        color: 0xdfe3ea, roughness: 0.75, metalness: 0.0, flatShading: false,
        polygonOffset: true, polygonOffsetFactor: 1, polygonOffsetUnits: 1 }));
      const lines = new THREE.LineSegments(new THREE.EdgesGeometry(geometry, 28),
        new THREE.LineBasicMaterial({ color: dark.matches ? 0x0d0d10 : 0x22252a }));
      const group = new THREE.Group();
      group.add(solid, lines);
      const open = openEdges(geometry);
      if (open) group.add(open);
      group.userData.sphere = geometry.boundingSphere;
      return group;
    }));
  }
  return built.get(src);
}

let wanted = null;
function show(src, fit) {
  wanted = src;
  build(src).then((group) => {
    if (src !== wanted) return;  // another entry was picked while this one loaded
    if (shown) scene.remove(shown);
    shown = group;
    scene.add(shown);
    if (fit) {
      const { center, radius } = group.userData.sphere;
      const away = new THREE.Vector3(1, -1, 0.9).normalize()
        .multiplyScalar(radius / Math.sin(THREE.MathUtils.degToRad(15)) * 1.05);
      camera.position.copy(center).add(away);
      camera.near = radius / 100; camera.far = radius * 100;
      camera.updateProjectionMatrix();
      controls.target.copy(center);
    }
  });
}

function frame() {
  controls.update();
  sun.position.copy(camera.position);
  renderer.render(scene, camera);
}

// the entry picked (the page's other script's `entry` event, its detail the path of its
// triangles): the camera is fitted to the first solid shown, and stays put as others are picked,
// so they can be compared; nothing is drawn while the viewer is hidden
let fitted = false;
function picked(src) {
  renderer.setAnimationLoop(src ? frame : null);
  if (!src) return;
  show(src, !fitted);
  fitted = true;
}
document.addEventListener('entry', (event) => picked(event.detail));
new ResizeObserver(resize).observe(canvas);
resize();
// this module runs after the other script has picked the first entry
picked(document.querySelector('section.entry:not([hidden])')?.dataset.src);
</script>"""


def main():
    registry, results, cases, drawings = load()
    for k in registry.values():
        for cid in sorted(set(workarounds(k)) - set(cases)):
            print(f"kernels/{k['name']}/workarounds.toml: no case {cid}", file=sys.stderr)
    os.makedirs(SITE, exist_ok=True)
    for name in sorted(os.listdir(WEB)):
        shutil.copyfile(os.path.join(WEB, name), os.path.join(SITE, name))
    with open(os.path.join(SITE, "board.json"), "w") as f:
        json.dump(board(registry, results, cases, drawings), f, separators=(",", ":"),
                  ensure_ascii=False)
    ids = sorted(cases)
    for i, cid in enumerate(ids):
        category = cid.rpartition("/")[0]
        prev = ids[i - 1] if i and ids[i - 1].rpartition("/")[0] == category else None
        after = (ids[i + 1] if i + 1 < len(ids) and ids[i + 1].rpartition("/")[0] == category
                 else None)
        case_page(cases[cid], registry, results, drawings.get(cid), prev, after)
    print(f"site/: the board, board.json and {len(cases)} case pages written")


if __name__ == "__main__":
    main()
