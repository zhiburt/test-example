use std::{
    cell::RefCell,
    io::{Cursor, Result, Write},
    path::Path,
    rc::Rc,
};

use crate::format::{Code, Target};

use cargo::{
    core::{Shell, Workspace},
    ops::{self, CompileFilter, CompileOptions, FilterRule, LibRule, NewOptions, Packages},
    util::command_prelude::CompileMode,
    Config as CargoConfig,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Output {
    pub build: Vec<u8>,
    pub run: Vec<u8>,
}

pub fn run(code: Code) -> Result<Output> {
    let mut out = Output::default();

    let tmp_dir = tempfile::tempdir().unwrap();
    let tmp_path = tmp_dir.as_ref().to_owned().join("rust-example");

    let writer = Writer::default();
    let stdout = Box::new(writer.clone());

    let cfg_default = CargoConfig::default().unwrap();
    let cwd = cfg_default.cwd().to_owned();
    let homedir = cfg_default.home().clone().into_path_unlocked();

    let shell = Shell::from_write(stdout);
    let cfg = CargoConfig::new(shell, cwd, homedir);

    let ws = cargo_new(&code, &tmp_path, &cfg)?;
    cargo_compile(&cfg, &ws)?;

    if matches!(code.target, Target::Run) {
        cargo_run(&cfg, &ws)?;
    }

    tmp_dir.close()?;

    out.build = writer.buf.borrow().get_ref().clone();

    Ok(out)
}

fn cargo_new<'a>(code: &Code, dir: &Path, cfg: &'a CargoConfig) -> Result<Workspace<'a>> {
    let deps = code.include.join("\n");

    let path = dir.to_owned();
    let path_manifest = dir.join("Cargo.toml");
    let file_path = dir.join("src").join("main.rs");
    let pkg_name = Some(String::from("rust-example"));

    let opts = NewOptions::new(None, true, false, path, pkg_name, None, None).unwrap();

    ops::new(&opts, cfg).unwrap();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    file.write_all(code.text.as_bytes())?;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path_manifest)
        .unwrap();

    file.write_all(deps.as_bytes()).unwrap();

    let ws = Workspace::new(&path_manifest, cfg).unwrap();

    ops::resolve_ws(&ws).unwrap();
    ops::generate_lockfile(&ws).unwrap();

    Ok(ws)
}

fn cargo_compile(cfg: &CargoConfig, ws: &Workspace) -> Result<String> {
    let opts = CompileOptions::new(cfg, CompileMode::Build).unwrap();
    ops::compile(ws, &opts).unwrap();

    let _ = ops::compile(ws, &opts).unwrap();

    Ok(String::new())
}

fn cargo_run(cfg: &CargoConfig, ws: &Workspace) -> Result<String> {
    let pkg = ws.current().unwrap().name().to_string();

    let mut opts = CompileOptions::new(cfg, CompileMode::Build).unwrap();

    opts.spec = Packages::Packages(vec![pkg.clone()]);
    opts.filter = CompileFilter::Only {
        all_targets: true,
        lib: LibRule::False,
        bins: FilterRule::Just(vec![pkg]),
        examples: FilterRule::All,
        tests: FilterRule::All,
        benches: FilterRule::All,
    };

    // todo: it does EXEC syscall which replaces a caller
    ops::run(ws, &opts, &[]).unwrap();

    Ok(String::new())
}

#[derive(Debug, Clone, Default)]
struct Writer {
    buf: Rc<RefCell<Cursor<Vec<u8>>>>,
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        RefCell::borrow_mut(&self.buf).get_mut().write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        RefCell::borrow_mut(&self.buf).flush()
    }
}
