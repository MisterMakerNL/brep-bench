# The board

The results are published at
**[bmander.github.io/brep-bench](https://bmander.github.io/brep-bench/)**. The board ranks no
kernel above another and lists them in the order of `kernels.json`. It has:

- **An overview:** each kernel's outcomes, each category's outcomes by kernel, and a strip of every
  case's outcome for every kernel.
- **An explorer** of every case, searchable by title, id and tag and filtered by category, tag,
  whether the case is settled (has an expected answer: a closed form or two kernels' agreement; see
  [SPEC.md](../SPEC.md#principles)), and each kernel's outcome. It has views of the cases no kernel
  passes, the cases one kernel alone passes, the cases the kernels shown differ on, and the cases
  with a workaround. Its cells show each outcome, or each run's time or peak memory. Its address
  records its filters, so a view can be linked.
- **A page for each kernel:** its outcomes by category, its refusals grouped by reason, its wrong
  answers, crashes and timeouts, its workarounds grouped by the missing feature, and its times
  and memory.
- **A performance page:** every run's time, CPU time or peak memory as a log-scale dot plot; a
  summary per kernel; kernels compared pairwise, as geometric-mean ratios over the cases both
  pass; medians by category and the slowest cases; multicore use (cores kept busy, speedup over
  one core, efficiency); and each kernel's binary size. Each runner's start-up time is left out
  by default, and a switch counts it again. Results recorded before these measurements show as
  not measured, with the start-up estimated from the kernel's fastest run and marked `*`.
- **A page for each case:** the description; its expected volume and area with their closed
  forms (or, for an unsettled case, a note that it has none yet); a drawing of the expected solid
  (three views and an isometric); a table of every kernel's entry at a glance, a column each (its
  outcome, its errors or measures, its time, CPU time and peak memory, and its lines of code,
  comments left out); and the entry picked, by a click on that table or
  on the buttons under it: its result in a 3D viewer, with any open edges of its mesh marked in red;
  its time; its notes (why it failed, what it works around); its data (its measures against the
  expected ones, which other kernels it agrees with, its faces, its mesh and its STL to download);
  and its code. `#k=NAME` in the address picks a kernel's entry, as the explorer's and the kernel
  pages' links do.
- **`board.json`:** every outcome of every case in one file, which the pages read.

The pages are built by `harness/site.py` from `harness/web/`; see [running the bench](running.md).
