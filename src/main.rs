use std::fs::File;
use std::io::{BufRead, BufReader};

use semver::{Version, VersionReq};

use clap::{CommandFactory, Parser, Subcommand};

/// Print, filter, sort lines that match a semantic version (https://semver.org).
///
/// Print lines that match a semantic version to standard output. If FILE is '-', read lines from standard input.
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print lines that match a semver version.
    #[clap(alias = "m")]
    Match {
        /// Sort lines
        #[clap(short, long)]
        sort: bool,

        /// Sort lines in reversed order
        #[clap(short, long)]
        reverse: bool,

        /// Removes repeated versions (implies --sort)
        #[clap(short, long)]
        uniq: bool,

        /// Filter versions according to expression.
        #[clap(short, long, num_args(1), value_name("EXPR"), value_hint = clap::ValueHint::Other)]
        filter: Option<String>,

        /// Files to process, if '-' read standard input
        #[clap(name = "FILE", required = true, num_args = 1.., value_hint = clap::ValueHint::FilePath)]
        files: Vec<String>,
    },
    /// Print lines that do not match a semver version.
    #[clap(aliases = &["nom", "nomatch"])]
    Invert {
        /// Files to process, if '-' read standard input
        #[clap(name = "FILE", required = true, num_args = 1.., value_hint = clap::ValueHint::FilePath)]
        files: Vec<String>,
    },
    /// Generate completion for the specified shell and exit
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

fn process_files<F>(files: &Vec<String>, mut consumer: F) -> std::io::Result<()>
where
    F: FnMut(String),
{
    if files.first().unwrap().eq("-") {
        let buffer = BufReader::new(std::io::stdin());

        for line in buffer.lines().map_while(Result::ok) {
            consumer(line);
        }
    } else {
        for fname in files {
            match File::open(fname) {
                Ok(f) => {
                    let buffer = BufReader::new(f);

                    for line in buffer.lines().map_while(Result::ok) {
                        consumer(line);
                    }
                }

                Err(e) => return Err(e),
            }
        }
    }

    Ok(())
}

fn print_invert(line: String) {
    if Version::parse(&line).is_err() {
        println!("{}", line);
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Invert { files } => {
            return match process_files(files, print_invert) {
                Ok(_) => Ok(()),
                Err(e) => return Err(e.to_string()),
            };
        }
        Commands::Match {
            sort,
            reverse,
            uniq,
            filter,
            files,
        } => {
            let requirement: Option<VersionReq> = match filter {
                Some(f) => match VersionReq::parse(f) {
                    Ok(r) => Some(r),
                    Err(e) => return Err(format!("Illegal filter format expression: {e}")),
                },
                None => None,
            };

            if *sort || *reverse || *uniq {
                let mut versions: Vec<Version> = Vec::new();

                let collect_semver = |line: String| {
                    if let Ok(v) = Version::parse(&line) {
                        if requirement.as_ref().is_none()
                            || requirement.as_ref().is_some_and(|r| r.matches(&v))
                        {
                            versions.push(v);
                        }
                    }
                };

                match process_files(files, collect_semver) {
                    Ok(_) => {
                        if *sort || *uniq {
                            versions.sort();
                        }

                        if *uniq {
                            versions.dedup()
                        }

                        if *reverse {
                            versions.reverse()
                        }

                        for version in versions.iter() {
                            println!("{version}")
                        }
                    }
                    Err(e) => return Err(e.to_string()),
                };
            } else {
                let print_semver = |line: String| {
                    if let Ok(v) = Version::parse(&line) {
                        if requirement.as_ref().is_none()
                            || requirement.as_ref().is_some_and(|r| r.matches(&v))
                        {
                            println!("{}", v);
                        }
                    }
                };

                return match process_files(files, print_semver) {
                    Ok(_) => Ok(()),
                    Err(e) => return Err(e.to_string()),
                };
            }
        }
        Commands::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
        }
    }

    Ok(())
}
