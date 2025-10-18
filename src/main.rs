use crate::attrs::Attrs;
use anyhow::Ok;
use std::{
    env,
    fs::{DirBuilder, File},
    io::Write,
    process::{Command, exit},
};

mod attrs;

#[allow(dead_code)]
enum DebugLevel {
    INFO,
    WARN,
    ERROR,
}

macro_rules! debug_print {
    ($l:expr,$($fmt:tt)*) => {
        match $l {
            DebugLevel::INFO => println!("[{}]: {}","I".grey(),format!($($fmt)*)),
            DebugLevel::WARN => println!("[{}]: {}","W".yellow_bold(),format!($($fmt)*)),
            DebugLevel::ERROR => println!("[{}]: {}","E".red_bold(),format!($($fmt)*)),
        }
    };
}

fn run_cmd(cmd: &str) -> anyhow::Result<String> {
    debug_print!(DebugLevel::INFO, "running command '{}'", cmd.green());

    let out = Command::new("sh").arg("-c").arg(cmd).output()?;

    if out.status.success() {
        let out = String::from_utf8_lossy(&out.stdout);
        return Ok(out.to_string());
    } else {
        debug_print!(DebugLevel::ERROR, "command Failed");

        let out = String::from_utf8_lossy(&out.stderr);

        eprintln!("Reason:\n'{}'", out.to_string().as_str().underline());
        exit(1);
    }
}

fn setup_makefile(pn: &str) -> anyhow::Result<()> {
    debug_print!(DebugLevel::INFO, "Writing makefile contents");

    let makefile_path = format!("./{pn}/Makefile");
    let mut f = File::create(makefile_path)?;

    let makefile_contents = format!(
        r#"CC = gcc
CFLAGS = -Wall -Wextra -ggdb
SOURCE = {pn}.c
TARGET = ./build/{pn}
HEADER = essentials.h

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

    debug_print!(DebugLevel::INFO, "Creating build directory");
    let build_dir = format!("./{pn}/build");
    DirBuilder::new().create(build_dir)?;

    Ok(())
}

fn setup_header(pn: &str) -> anyhow::Result<()> {
    let header_link =
        r"https://raw.githubusercontent.com/BayonetArch/essen.h/refs/heads/master/essentials.h";

    let cmd = format!("wget {header_link} -O ./{pn}/essentials.h");
    run_cmd(&cmd)?;

    Ok(())
}

fn setup_main(pn: &str) -> anyhow::Result<()> {
    debug_print!(DebugLevel::INFO, "writing to '{pn}.c'");

    let file_path = format!("./{pn}/{pn}.c");

    let file_contents = format!(
        r#"/* {pn}.c */

#include "essentials.h"

int main(void) {{
    printf("Hello,World\n");

    return 0;
}}"#
    );

    let mut f = File::create(&file_path)?;
    f.write(file_contents.as_bytes())?;

    Ok(())
}

fn test_run(pn: &str) -> anyhow::Result<()> {
    let out = run_cmd(&format!("make --no-print-directory  -C ./{pn} run"))?;

    print!("{out}");

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

    if project_name.len() > 15 {
        debug_print!(DebugLevel::ERROR, "Project name is too long");
        exit(1);
    }

    DirBuilder::new().create(&project_name)?;

    setup_makefile(&project_name)?;
    setup_header(&project_name)?;
    setup_main(&project_name)?;
    test_run(&project_name)?;

    Ok(())
}
