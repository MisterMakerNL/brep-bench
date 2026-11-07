"""What the harness reads: the cases (cases/**/*.md), the kernels (kernels.json), and where a kernel's
implementation of a case lives; and of a family (a case with `[params]`, SPEC.md), its variations,
drawn from SEED, each with its expected measures evaluated from the case's formulas. Standard
library only (tomllib: Python 3.11 or later)."""
import ast
import json
import math
import os
import random
import tomllib

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def read_case(path):
    """A case file: its TOML header (between `+++` lines) and the description after it."""
    with open(path, encoding="utf-8") as f:
        text = f.read()
    if not text.startswith("+++"):
        raise ValueError(f"{path}: no +++ header")
    _, header, body = text.split("+++", 2)
    meta = tomllib.loads(header)
    cid = os.path.relpath(path, os.path.join(ROOT, "cases"))[:-len(".md")].replace(os.sep, "/")
    if "params" in meta:
        return read_family(cid, meta, body, path)
    return {
        "id": cid,
        "title": meta.get("title", cid),
        "tags": meta.get("tags", []),
        # an unsettled case (`settled = false`) has no answer yet: what each kernel builds is shown
        # `tex` is the closed form in LaTeX, for the case's page; absent where there is none
        "expect": ({k: {"value": float(meta[k]["value"]), "exact": meta[k]["exact"],
                        "tex": meta[k].get("tex")}
                    for k in ("volume", "area")}
                   if meta.get("settled", True) else None),
        "description": body.strip(),
        "path": path,
        "family": None,
    }


def cases(prefix=""):
    """Every case whose id starts with `prefix`, in id order."""
    out = []
    for dirpath, _, files in os.walk(os.path.join(ROOT, "cases")):
        for f in files:
            if f.endswith(".md"):
                c = read_case(os.path.join(dirpath, f))
                if c["id"].startswith(prefix):
                    out.append(c)
    return sorted(out, key=lambda c: c["id"])


def kernels():
    with open(os.path.join(ROOT, "kernels.json")) as f:
        return json.load(f)["kernels"]


def source(kernel, cid):
    """Where `kernel`'s implementation of case `cid` is, from its `source` pattern, or None."""
    pattern = kernel.get("source")
    if not pattern:
        return None
    category, _, name = cid.rpartition("/")
    return os.path.join(ROOT, pattern.format(case=cid, category=category, name=name,
                                             name_snake=name.replace("-", "_")))


# A family (SPEC.md, "A family") is one case stated over parameters: `[params]` says how each is
# drawn, `[let]` names quantities of them, and `[volume]` and `[area]` give each measure as a
# `formula` of both. The harness draws its variations from SEED, the same every run, and grades
# each against its formulas evaluated there. SEED changes only with a version of SPEC.md.
SEED = "brep-bench 1.10"
# how many variations a family has, unless its `[family]` says
VARIATIONS = 20
# the decimal places a real parameter is rounded to, unless its `[family]` says
DIGITS = 3
# how many draws a variation may take to meet the family's `require`
DRAWS = 1000

# what a formula may call or name: arithmetic, these functions and constants, and the parameters
# and quantities named before it. Angles in a case are in degrees: sind, cosd and tand take them.
FUNCTIONS = {
    "sqrt": math.sqrt, "cbrt": math.cbrt, "exp": math.exp, "log": math.log, "abs": abs,
    "min": min, "max": max, "hypot": math.hypot, "floor": math.floor, "ceil": math.ceil,
    "sin": math.sin, "cos": math.cos, "tan": math.tan, "asin": math.asin, "acos": math.acos,
    "atan": math.atan, "atan2": math.atan2, "radians": math.radians, "degrees": math.degrees,
    "sind": lambda a: math.sin(math.radians(a)), "cosd": lambda a: math.cos(math.radians(a)),
    "tand": lambda a: math.tan(math.radians(a)),
}
CONSTANTS = {"pi": math.pi}
OPERATORS = (ast.Expression, ast.BinOp, ast.UnaryOp, ast.Call, ast.Name, ast.Load, ast.Constant,
             ast.Add, ast.Sub, ast.Mult, ast.Div, ast.Pow, ast.USub, ast.UAdd, ast.Mod,
             ast.FloorDiv, ast.Compare, ast.Lt, ast.LtE, ast.Gt, ast.GtE, ast.BoolOp, ast.And,
             ast.Or)


def evaluate(formula, names):
    """`formula` (a number, or a string of arithmetic) evaluated with `names` bound: Python's
    arithmetic and comparisons, FUNCTIONS and CONSTANTS, nothing else."""
    if isinstance(formula, (int, float)):
        return formula
    tree = ast.parse(str(formula), mode="eval")
    for node in ast.walk(tree):
        if not isinstance(node, OPERATORS):
            raise ValueError(f"{formula!r}: {type(node).__name__} is not allowed in a formula")
        if isinstance(node, ast.Call) and not (isinstance(node.func, ast.Name)
                                               and node.func.id in FUNCTIONS):
            raise ValueError(f"{formula!r}: only {', '.join(FUNCTIONS)} may be called")
        if isinstance(node, ast.Constant) and not isinstance(node.value, (int, float)):
            raise ValueError(f"{formula!r}: only numbers are allowed")
    scope = {**FUNCTIONS, **CONSTANTS, **names}
    return eval(compile(tree, "<formula>", "eval"), {"__builtins__": {}}, scope)


def draw(spec, names, rng, digits):
    """One parameter drawn by its `spec`: `[low, high]`, a real uniform between them, rounded to
    `digits` places; `{int = [low, high]}`, an integer, both ends included; `{choice = [...]}`, one
    of those. Bounds may be formulas of the parameters drawn before it."""
    if isinstance(spec, list):
        low, high = (evaluate(b, names) for b in spec)
        return round(rng.uniform(low, high), digits)
    if "int" in spec:
        low, high = (evaluate(b, names) for b in spec["int"])
        return rng.randint(math.ceil(low), math.floor(high))
    if "choice" in spec:
        return rng.choice(spec["choice"])
    raise ValueError(f"a parameter is [low, high], {{int = [...]}} or {{choice = [...]}}: {spec}")


def variation(cid, meta, i):
    """Family `cid`'s variation `i`: its parameters, drawn from SEED, the case and `i`, until they
    meet the family's `require`; and the quantities its `[let]` names, by name."""
    family = meta.get("family", {})
    digits = family.get("digits", DIGITS)
    rng = random.Random(f"{SEED}/{cid}/{i}")
    for _ in range(DRAWS):
        params = {}
        for name, spec in meta["params"].items():
            params[name] = draw(spec, params, rng, digits)
        names = dict(params)
        for name, formula in meta.get("let", {}).items():
            names[name] = evaluate(formula, names)
        if all(evaluate(r, names) for r in family.get("require", [])):
            return params, names
    raise ValueError(f"{cid}: variation {i} met its `require` in none of {DRAWS} draws")


def read_family(cid, meta, body, path):
    """A family: its parameters' specs, and each variation's parameters and expected measures."""
    if not all("formula" in meta.get(k, {}) for k in ("volume", "area")):
        raise ValueError(f"{path}: a family gives its volume and area as a `formula`")
    count = meta.get("family", {}).get("variations", VARIATIONS)
    variations = []
    for i in range(count):
        params, names = variation(cid, meta, i)
        variations.append({"params": params,
                           "expect": {k: float(evaluate(meta[k]["formula"], names))
                                      for k in ("volume", "area")}})
    return {
        "id": cid,
        "title": meta.get("title", cid),
        "tags": meta.get("tags", []),
        "expect": {k: {"value": None, "formula": meta[k]["formula"], "exact": meta[k]["exact"],
                       "tex": meta[k].get("tex")} for k in ("volume", "area")},
        "description": body.strip(),
        "path": path,
        "family": {"params": meta["params"], "let": meta.get("let", {}),
                   "require": meta.get("family", {}).get("require", []),
                   "variations": variations},
    }
