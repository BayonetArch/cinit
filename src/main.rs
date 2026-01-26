use simple_term_attr::{LogLevel, StyleAttributes, debug_print, debug_println};
use std::{
    env,
    error::Error,
    fs::{DirBuilder, File},
    io::{self, Write, stdout},
    process::{Command, exit},
};

type Res<T> = Result<T, Box<dyn Error>>;

struct Opts {
    project_name: String,
    git_project: bool,
}

impl Opts {
    fn new() -> Self {
        Self {
            project_name: "None".to_string(),
            git_project: false,
        }
    }
}

fn run_cmd(cmd: &str) -> Res<String> {
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

fn setup_makefile(pn: &str) -> Res<()> {
    debug_println!(LogLevel::INFO, "Writing makefile contents");

    let makefile_path = format!("./{pn}/Makefile");
    let mut f = File::create(makefile_path)?;

    let makefile_contents = format!(
        r#"CC = gcc
CFLAGS = -Wall -Wextra 
LIBS   = 
SOURCE = {pn}.c
TARGET = build/{pn}
HEADER = include/cx.h

all: $(TARGET)

CLANG ?= n
GDB ?= n
RUN_CMD ?= ./$(TARGET)

ifeq ($(GDB), y)
    CFLAGS += -ggdb
	RUN_CMD = gdb ./$(TARGET)	
endif

ifeq ($(CLANG),y)
	CC = clang
	CFLAGS += -fsanitize=address
endif

$(TARGET): $(SOURCE) $(HEADER)
	$(CC) $(LIBS) $(CFLAGS) $< -o $@

clean:
	rm -f $(TARGET)

.PHONY: run all clean

run: $(TARGET)
	$(RUN_CMD)
"#,
    );

    f.write(makefile_contents.as_bytes())?;

    debug_println!(LogLevel::INFO, "Creating build directory");
    let build_dir = format!("./{pn}/build");
    DirBuilder::new().create(build_dir)?;

    Ok(())
}

fn setup_header(pn: &str) -> Res<()> {
    let header_link = r"https://raw.githubusercontent.com/BayonetArch/cx.h/refs/heads/master/cx.h";

    let cmd = format!("wget {header_link} -O ./{pn}/include/cx.h");
    run_cmd(&cmd)?;

    Ok(())
}

fn setup_main(pn: &str) -> Res<()> {
    debug_println!(LogLevel::INFO, "Writing to '{pn}.c'");

    let file_path = format!("./{pn}/{pn}.c");

    let file_contents = format!(
        r#"/* {pn}.c */
#define CX_STRIP_PREFIX
#include "include/cx.h"

int main(void) {{
    println("Hello,World");

    return 0;
}}"#
    );

    let mut f = File::create(&file_path)?;
    f.write(file_contents.as_bytes())?;

    Ok(())
}

fn test_run(pn: &str) -> Res<()> {
    let out = run_cmd(&format!("make --no-print-directory  -C ./{pn} run"))?;

    println!("--------------------------------------------------");
    print!("{out}");
    println!("--------------------------------------------------");
    Ok(())
}

fn usage(program: &str) {
    println!("usage");
    println!("  {} project_name [flags] ...", program);
    println!();
    println!("flags");
    println!("-g   make it an git project");
    println!("-h   print help message");
}

fn parse_args() -> Res<Opts> {
    let mut args = env::args();
    let argc = env::args().count();
    let program = args.next().unwrap();

    if argc < 2 {
        eprintln!("{}: No arguments provided", "Error".red());
        usage(&program);
        return Err("project_name missing".into());
    }

    let mut opts = Opts::new();

    while let Some(flag) = &args.next() {
        match flag.as_str() {
            "-g" => {
                opts.git_project = true;
            }

            "-h" => {
                usage(&program);
                exit(0);
            }

            project_name if !project_name.starts_with("-") => {
                opts.project_name = project_name.to_string();
            }
            _ => {
                eprintln!("Unknown flag");
                exit(1);
            }
        }
    }

    if opts.project_name == "None" {
        usage(&program);
        return Err("project_name missing".into());
    }

    Ok(opts)
}

fn main() -> Res<()> {
    let opts = parse_args()?;
    let project_name = &opts.project_name;

    if project_name.len() > 30 {
        debug_println!(LogLevel::ERROR, "Project name is too long");
        exit(1);
    }
    debug_print!(LogLevel::INFO, "Proceed [Y/n]?: ");
    stdout().flush()?;
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let buf = buf.trim().to_lowercase();

    if !(buf.contains('y')) && !buf.is_empty() {
        return Err("Exiting..".into());
    }

    DirBuilder::new().create(project_name)?;
    DirBuilder::new().create(&format!("{project_name}/include"))?;

    if opts.git_project {
        let cmd = &format!("git init ./{project_name}");
        run_cmd(cmd)?;
    }

    setup_makefile(project_name)?;
    setup_header(project_name)?;
    setup_main(project_name)?;
    test_run(project_name)?;

    Ok(())
}
