//! The `magpie` binary. All behaviour lives in the library so it can be tested.

fn main() {
    let args: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let code = magpie_cli::run(&args, &mut stdout.lock(), &mut stderr.lock());
    std::process::exit(code);
}
