use clap::Shell;
use std::fs;

include!("src/app.rs");

fn main() {
    let var = std::env::var_os("SHELL_COMPLETIONS_DIR").or(std::env::var_os("OUT_DIR"));

    let outdir = match var {
        None => return,
        Some(outdir) => outdir,
    };

    fs::create_dir_all(&outdir).unwrap();

    let mut app = build_app();
    app.gen_completions("mf_attendance", Shell::Bash, &outdir);
    app.gen_completions("mf_attendance", Shell::Fish, &outdir);
    app.gen_completions("mf_attendance", Shell::Zsh, &outdir);
    app.gen_completions("mf_attendance", Shell::PowerShell, &outdir);
}
