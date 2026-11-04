//! Finds the case implementations and the geop version for the runner.
//!
//! Every `src/cases/<category>/<name_snake>.rs` is one case, `<category>/<name>` with the name's
//! underscores as hyphens, and must define `pub fn build(b: &mut Bench) -> Result<(), Refusal>`:
//! it writes the case's program into `b`, step by step, each step run as it is added. This writes
//! `$OUT_DIR/cases.rs`: a module per file (a raw identifier, so `box.rs` is one) and `CASES`, the
//! table from case id to its `build`. Adding a case is adding its file.
//!
//! It also sets `KERNEL_VERSION` to the release of geop this runner is built on: the tag its
//! crates come from in `Cargo.toml`.
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let root = Path::new(&manifest);
    let cases = root.join("src/cases");
    println!("cargo:rerun-if-changed={}", cases.display());

    let mut categories: Vec<_> = fs::read_dir(&cases)
        .expect("src/cases reads")
        .map(|e| e.expect("a directory entry").path())
        .filter(|p| p.is_dir())
        .collect();
    categories.sort();

    let (mut modules, mut table) = (String::new(), String::new());
    for dir in &categories {
        println!("cargo:rerun-if-changed={}", dir.display());
        let category = dir.file_name().unwrap().to_str().expect("a UTF-8 category");
        let mut files: Vec<_> = fs::read_dir(dir)
            .expect("a category reads")
            .map(|e| e.expect("a directory entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "rs"))
            .collect();
        files.sort();
        writeln!(modules, "pub mod {category} {{").unwrap();
        for file in &files {
            let name = file.file_stem().unwrap().to_str().expect("a UTF-8 name");
            writeln!(modules, "    #[path = {:?}]\n    pub mod r#{name};", file.display().to_string())
                .unwrap();
            writeln!(
                table,
                "    ({:?}, {category}::r#{name}::build),",
                format!("{category}/{}", name.replace('_', "-"))
            )
            .unwrap();
        }
        writeln!(modules, "}}").unwrap();
    }
    let out = Path::new(&std::env::var("OUT_DIR").unwrap()).join("cases.rs");
    let text = format!(
        "{modules}\n/// Every case id and the function that builds it.\npub const CASES: &[(&str, \
         Build)] = &[\n{table}];\n"
    );
    fs::write(out, text).expect("cases.rs writes");

    println!("cargo:rerun-if-changed=Cargo.toml");
    let manifest = fs::read_to_string(root.join("Cargo.toml")).unwrap_or_default();
    let version = manifest
        .lines()
        .find(|l| l.starts_with("geop-cad-base"))
        .and_then(|l| l.split("tag = \"").nth(1))
        .and_then(|t| t.split('"').next())
        .map_or("unknown", |t| t.trim_start_matches('v'));
    println!("cargo:rustc-env=KERNEL_VERSION={version}");
}
