use clap::{CommandFactory, Parser, Subcommand};
use semver::{Version, VersionReq};

use version::{process_files, InputHandle};

/// Print, filter, sort lines that match a semantic version (https://semver.org).
#[derive(Parser)]
#[clap(author, version, about, bin_name = "semver")]
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
        filter: Option<VersionReq>,

        /// Files to process, if '-' read standard input
        #[clap(name = "FILE", required = true, num_args = 1.., value_hint = clap::ValueHint::FilePath)]
        files: Vec<InputHandle>,
    },
    /// Print lines that do not match a semver version.
    #[clap(aliases = &["i", "nomatch"])]
    Invert {
        /// Files to process, if '-' read standard input
        #[clap(name = "FILE", required = true, num_args = 1.., value_hint = clap::ValueHint::FilePath)]
        files: Vec<InputHandle>,
    },
    /// Generate completion for the specified shell and exit
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
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
            if *sort || *reverse || *uniq {
                let mut versions: Vec<Version> = Vec::new();

                let collect_semver = |line: String| {
                    if let Ok(v) = Version::parse(&line) {
                        if filter.as_ref().is_none()
                            || filter.as_ref().is_some_and(|r| r.matches(&v))
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
                        if filter.as_ref().is_none()
                            || filter.as_ref().is_some_and(|r| r.matches(&v))
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
