# 🔍🧹 spl-lint

A linter for Splunk Search Processing Language (SPL / SPL2).

<p align="center">
──────────────────────────────────────────────────────────────────────────────────────────────<br>
<strong>Beta</strong>: This tool is in beta. Syntax coverage and lint rules are still limited. Expect breaking changes.<br>
──────────────────────────────────────────────────────────────────────────────────────────────
</p>

`spl-lint` takes SPL query strings and detects syntax errors, unknown commands, best-practice violations, and style issues.

## Installation

### From source

```bash
cargo install --git https://github.com/hiboma/spl-lint
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/hiboma/spl-lint/releases).

### Homebrew (macOS)

```bash
brew tap hiboma/tap
brew install spl-lint
```

## Usage

```bash
# Lint files
spl-lint query.spl

# Lint from stdin
echo 'index=main | stats count by src_ip' | spl-lint

# JSON output
spl-lint --format json query.spl

# Disable specific rules
spl-lint --disable W002 query.spl

# List available rules
spl-lint --list-rules

# List supported SPL commands
spl-lint --list-commands

# List supported eval functions
spl-lint --list-functions

# Verbose mode (show success messages)
spl-lint -v query.spl

# Remove backtick comments and print formatted query
spl-lint --trim query.spl

# Remove backtick comments and write back to file
spl-lint --trim --write query.spl

# SPL2 mode
spl-lint --spl2 query.spl2
spl-lint --spl2 --list-rules
spl-lint --spl2 --list-commands
spl-lint --spl2 --list-functions
```

### Exit codes

- `0`: No errors or warnings
- `1`: One or more errors or warnings found

## SPL1 Lint Rules

| ID | Severity | Category | Description |
|---|---|---|---|
| `E001` | error | syntax | Syntax error (parse failure) |
| `W001` | warning | performance | Search with `*` only (matches everything) |
| `W002` | warning | correctness | Unknown command name |
| `W003` | warning | correctness | Missing required arguments for command |
| `W004` | info | style | No whitespace around pipe `\|` |
| `W005` | warning | correctness | Ambiguous AND/OR precedence (mixed without parentheses) |
| `W006` | info | performance | Filter after aggregation command |
| `W007` | warning | correctness | Unknown eval function name |

## SPL2 Lint Rules

| ID | Severity | Category | Description |
|---|---|---|---|
| `S001` | error | syntax | SPL2 syntax error (parse failure) |
| `S002` | warning | correctness | Unknown SPL2 command name |
| `S003` | warning | correctness | Unknown SPL2 eval/stats function name |
| `S004` | warning | correctness | String concatenation using `.` (use `+` in SPL2) |
| `S005` | info | style | Special character field name without single quotes |
| `S006` | warning | correctness | Reserved word used as field name |

## Coverage

- **155+** SPL commands
- **101+** eval functions
- **34+** stats functions
- **50** SPL2 commands
- **130+** SPL2 eval functions
- **41** SPL2 stats functions

## Building from source

```bash
git clone https://github.com/hiboma/spl-lint.git
cd spl-lint
cargo build --release
```

### Requirements

- Rust 1.85+ (edition 2024)

### Running tests

```bash
cargo test
```

### Linting

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## License

[MIT](LICENSE)
