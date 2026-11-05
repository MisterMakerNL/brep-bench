# Running the bench

## Locally

```bash
python3 harness/build.py           # every kernel's runner (cargo, pip, dotnet)
python3 harness/run.py             # every kernel, every case: results/*.json
python3 harness/draw.py            # each case's drawing: site/drawings/
python3 harness/site.py            # board.json, the board's pages and a page per case: site/
python3 -m http.server -d site     # the board reads board.json, so serve it to open it
```

`harness/build.py NAME` builds one kernel's runner, and `harness/run.py --kernel NAME --case
boolean/` runs part of the suite; `NAME` is the kernel's `name` in `kernels.json` (e.g. `occt`). A
run narrowed by `--case` is only a check and is not recorded; a run of every case, even for one
kernel, writes `results/<kernel>.json`.

Each run's time, CPU time (user and system, over every thread) and peak memory are recorded
beside its outcome; none of them is graded. Runners are started by `harness/measure.py`, a
small process that holds almost no memory, because on Linux a child's peak resident set starts
at its parent's. Before its cases, each runner is given the case id `harness/start`, which names
no case, five times; the median is the kernel's start-up, which the board subtracts from its
times. A CI run of only some of a kernel's cases keeps its last start-up and binary size. `--one-core` runs each case a kernel builds a second time, pinned to one core, for its
speedup on all of them (Linux only). Each result file also records its machine and the size of
the kernel's binary, as `binary` in `kernels.json` names it.

CI does not run on pull requests, so run a change locally before proposing it.

The harness uses only Python's standard library (3.11 or later). The runners need their own
toolchains: Rust, Python's pip and, for Aetheris, the .NET 10 SDK.

## On GitHub

A push to `main` reruns only the cases its changes affect, on top of the last run's results.
`harness/changed.py` decides:

- a case: every kernel runs it;
- a kernel's implementation of a case: that kernel runs that case;
- anything else under `kernels/<kernel>/` (except its `workarounds.toml`), or how its entry in
  `kernels.json` builds or runs it: that kernel runs every case;
- a kernel newly added, or one with no results yet that the push touches: it runs every case;
- a change of which kernel is the reference: every case is drawn again;
- a deleted case: its rows are dropped from the results;
- how a run is made, measured or graded (`harness/run.py`, `measure.py`, `stl.py`, `suite.py`,
  `build.py`): the whole suite runs;
- `harness/draw.py`: every case is drawn again;
- anything else (the site, the docs, a kernel's display-only fields, such as its license):
  nothing runs, and the site is only rebuilt.

With no last results to build on, a push runs the whole suite. The site is published after every push.

The whole suite also runs nightly, and on a manual dispatch unless its `full` input is unchecked. CI
allows 120 s per case, not the 60 s default, so a timeout on the board means more than 120 s. CI
runs with `--one-core`, which roughly doubles the time spent running cases.

## Layout

```
cases/<category>/<name>.md         a case: its closed forms (TOML header) and its description
kernels.json                       the kernels: how to build and run each, where its cases live
kernels/<kernel>/                  a kernel's runner and its implementation of each case
kernels/<kernel>/workarounds.toml  the cases it builds by working around a feature it lacks
harness/                           build, run and grade, draw, and render the site
harness/web/                       the board's pages, which read the site's board.json, and its icons
results/<kernel>.json              the last results
docs/                              these notes
```
