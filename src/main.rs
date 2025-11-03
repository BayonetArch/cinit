use anyhow::Ok;
use simple_term_attr::{LogLevel, StyleAttributes, debug_print, debug_println};
use std::{
    env,
    fs::{DirBuilder, File},
    io::{self, Write, stdout},
    process::{Command, exit},
};

fn run_cmd(cmd: &str) -> anyhow::Result<String> {
    debug_println!(LogLevel::INFO, "Running command {}", cmd.green());
    let out = Command::new("sh").arg("-c").arg(cmd).output()?;

    if out.status.success() {
        let out = String::from_utf8_lossy(&out.stdout);
        return Ok(out.to_string());
    } else {
        debug_println!(LogLevel::ERROR, "Command Failed");

        let out = String::from_utf8_lossy(&out.stderr);

        eprintln!("Reason:\n\n{}", out);
        exit(1);
    }
}

fn setup_makefile(pn: &str) -> anyhow::Result<()> {
    debug_println!(LogLevel::INFO, "Writing makefile contents");

    let makefile_path = format!("./{pn}/Makefile");
    let mut f = File::create(makefile_path)?;

    let makefile_contents = format!(
        r#"CC = gcc
CFLAGS = -Wall -Wextra -ggdb
SOURCE = {pn}.c
TARGET = build/{pn}
HEADER = essen.h

all: $(TARGET)

$(TARGET): $(SOURCE) $(HEADER)
	$(CC) $(CFLAGS) $< -o $@

clean:
	rm -f $(TARGET)

.PHONY: run

run: $(TARGET)
	./$(TARGET)
"#,
    );

    f.write(makefile_contents.as_bytes())?;

    debug_println!(LogLevel::INFO, "Creating build directory");
    let build_dir = format!("./{pn}/build");
    DirBuilder::new().create(build_dir)?;

    Ok(())
}

fn setup_header(pn: &str) -> anyhow::Result<()> {
    let header_link =
        r"https://raw.githubusercontent.com/BayonetArch/essen.h/refs/heads/master/essen.h";

    let cmd = format!("wget {header_link} -O ./{pn}/essen.h");
    run_cmd(&cmd)?;

    Ok(())
}

fn setup_main(pn: &str) -> anyhow::Result<()> {
    debug_println!(LogLevel::INFO, "Writing to '{pn}.c'");

    let file_path = format!("./{pn}/{pn}.c");

    let file_contents = format!(
        r#"/* {pn}.c */

#include "essen.h"

int main(void) {{
    println("Hello,World");

    return 0;
}}"#
    );

    let mut f = File::create(&file_path)?;
    f.write(file_contents.as_bytes())?;

    Ok(())
}

fn test_run(pn: &str) -> anyhow::Result<()> {
    let out = run_cmd(&format!("make --no-print-directory  -C ./{pn} run"))?;

    println!("--------------------------------------------------");
    print!("{out}");
    println!("--------------------------------------------------");
    Ok(())
}

fn parse_args() -> String {
    let argv: Vec<_> = env::args().collect();

    if argv.len() != 2 {
        eprintln!("{}: no arguments provided", "err".red());
        eprintln!("usage: {} <project_name>", &argv[0]);
        exit(1);
    }
    let out = &argv[1];
    out.to_string()
}

fn main() -> anyhow::Result<()> {
    let project_name = parse_args();

    if project_name.len() > 25 {
        debug_println!(LogLevel::ERROR, "Project name is too long");
        exit(1);
    }
    debug_print!(LogLevel::INFO, "Proceed (Y/n)?: ");
    stdout().flush()?;
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;

    if !buf.contains(|x| x == 'Y' || x == 'y' || x == '\n') {
        return Err(anyhow::anyhow!("Exiting.."));
    }

    DirBuilder::new().create(&project_name)?;

    setup_makefile(&project_name)?;
    setup_header(&project_name)?;
    setup_main(&project_name)?;
    test_run(&project_name)?;
    Ok(())
}
