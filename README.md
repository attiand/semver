# semver
Print, filter, sort lines that match a semantic version (https://semver.org)

Internally uses https://crates.io/crates/semver for matching and sorting. Filter expressions syntax is described [here](https://docs.rs/semver/1.0.9/semver/struct.VersionReq.html#syntax)

```
$ semver help
Print, filter, sort lines that match a semantic version (https://semver.org).

Print lines that match a semantic version to standard output. If FILE is '-', read lines from standard input.

Usage: semver <COMMAND>

Commands:
  match        Print lines that match a semver version
  invert       Print lines that do not match a semver version
  completions  Generate completion for the specified shell and exit
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

```
$ semver help match
Print lines that match a semver version

Usage: semver match [OPTIONS] <FILE>...

Arguments:
  <FILE>...  Files to process, if '-' read standard input

Options:
  -s, --sort           Sort lines
  -r, --reverse        Sort lines in reversed order
  -u, --uniq           Removes repeated versions (implies --sort)
  -f, --filter <EXPR>  Filter versions according to expression
  -h, --help           Print help 
```

## Install
```bash
cargo install --git https://github.com/attiand/semver.git
```
## Examples

Print git tags that matches a semantic version.

```bash
git tag | semver match -
```

Print the highest sematic version in current git repo.

```bash
git tag | semver m --sort - | tail -n1
```

Print lines that matches a semantic version and has major version number 1 from specified file.

```bash
semver m --filter '>= 1, <2' tags.txt
```

Print all versions between `1.2.0` and `1.3.7` (inclusive) from specified files.

```bash
semver m --filter '>= 1.2.0, <=1.3.7' tags1.txt tags2.txt
```
