#[macro_use]
extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions("clapex", Shell::Bash, "./");
    app.gen_completions("clapex", Shell::Zsh, "./");
    app.gen_completions("clapex", Shell::PowerShell, "./");
}
