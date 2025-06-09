use std::fs::{ self, File };
use std::io::{ self, Write };
use std::path::Path;
use clap::{ Parser, Subcommand };

#[derive(Parser, Debug)]
#[command(about = "Scaffold a Mercury project.")]
#[command(version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// create new project
    New {
        /// name of project
        project_name: String,
    },
}

fn get_output(module_name: &str) -> String {
    format!(
":- module {}.

:- interface.
:- import_module io.

:- pred main(io::di, io::uo) is det.

:- implementation.

main(!IO) :-
    io.write_string(\"Hello, Mercury!\", !IO).
", module_name)
}

fn scaffold_file(project_name: &str) -> io::Result<()> {
    let dir    = Path::new(project_name);
    let fname  = format!("{}.m", project_name);
    let fname  = dir.join(fname);
    let output = get_output(project_name);

    match fs::create_dir_all(dir) {
        Err(e) => eprintln!("[ERROR] {} in creating directory: {}", line!(), e),
        Ok(_)  =>  println!("[INFO] created a directory: {:?}", dir),
    }

    match File::create(&fname) {
        Err(e) => {
            eprintln!("[ERROR] {} in creating file {:?}: {}", line!(), fname, e);
            Err(e)
        },
        Ok(mut file) => {
            match file.write_all(output.as_bytes()) {
                Err(e) => eprintln!("[ERROR] {} in writing file: {}", line!(), e),
                Ok(_)  =>  println!("[INFO] created and wrote a file: {:?}", fname),
            }

            Ok(())
        },
    }
}

fn run() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::New{ project_name } => {
            scaffold_file(&project_name)?;
            Ok(())
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("[ERROR] {}", e);
        std::process::exit(1);
    }
}
