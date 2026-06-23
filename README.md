# pkgrisk

A 5-second package health and risk analysis tool. Before running `npm install`, `pip install`, or `cargo add`, verify if a package is sustainable, abandoned, or introduces a risky dependency tree.

```
$ pkgrisk check left-pad

  left-pad@1.3.0 (npm)

  Health Score: 38/100  [RISKY]

  ├─ Maintenance     [WARN]  Last publish: 6 years ago
  ├─ Bus Factor      [FAIL]  1 maintainer, 0 commits in 4y
  ├─ Dependents      [PASS]  2.1M weekly downloads
  ├─ License         [PASS]  MIT (permissive, no conflicts)
  ├─ Vuln History    [PASS]  No known CVEs
  └─ Dep Tree Risk   [PASS]  0 dependencies (leaf package)

  Verdict: Works, but unmaintained. Fine for a frozen/stable
  use case, risky if you expect future fixes or support.
```

Why this exists: Existing tools like `npm audit` only check for known CVEs, while services like npms.io are outdated. `pkgrisk` solves this by providing a unified, single-binary health check for three major ecosystems (npm, PyPI, crates.io).

---

## Features

- **Unified Ecosystem Support:** Query npm, PyPI, and crates.io packages with the same command.
- **Maintenance Analysis:** Evaluates the last commit/publish date, issue resolution rate, and release frequency trends.
- **Bus Factor Analysis:** Determines the number of active maintainers. Projects reliant on a single maintainer are flagged as high risk.
- **Dependency Tree Scanning:** Checks for known vulnerabilities (via OSV.dev) and abandoned packages within transitive dependencies.
- **License Compatibility:** Ensures the project's license does not conflict with its dependencies.
- **Single Binary:** Compiled with Rust, requiring no runtime dependencies. Installation is instantaneous.
- **CI/CD Integration:** Utilize `--fail-under <score>` to enforce automated quality gates in your pipelines.
- **Offline Cache:** Features local SQLite caching for repeated queries to prevent rate-limiting.

## Installation

### Via Cargo (Recommended)

```bash
cargo install pkgrisk
```

### Build from Source

```bash
git clone https://github.com/hasanerman/pkgrisk.git
cd pkgrisk
cargo build --release
# Binary is located at ./target/release/pkgrisk
```

Verify the installation:

```bash
pkgrisk --version
```

## Usage

### Single Package Analysis

```bash
# Ecosystem is automatically inferred
pkgrisk check express

# Explicitly specify the ecosystem
pkgrisk check requests --ecosystem pypi
pkgrisk check serde --ecosystem crates

# Check a specific version
pkgrisk check lodash@4.17.20
```

### Manifest Scanning (package.json / Cargo.toml)

Run the scan command in your project root to evaluate all dependencies:

```bash
cd my-project/
pkgrisk scan
```

Output provides a summary per package and a total project risk score:

```
Scanning package.json (47 dependencies)...

  PACKAGE              SCORE   STATUS
  express              92      PASS
  left-pad             38      WARN
  some-abandoned-lib   12      FAIL
  ...

  Project Risk Summary: 2 failed, 45 scanned total.
```

### CI/CD Pipeline Integration

```bash
# Returns exit code 1 if any package scores below 50
pkgrisk scan --fail-under 50
```

GitHub Actions Example:

```yaml
- name: Check dependency health
  run: |
    cargo install pkgrisk
    pkgrisk scan --fail-under 40 --format json > pkgrisk-report.json
```

## Scoring Methodology

The health score ranges from 0 to 100, calculated as a weighted average across 6 primary categories:

| Category | Weight | Description |
|---|---|---|
| **Maintenance** | 25% | Last commit/release date, and commit frequency trends. |
| **Bus Factor** | 20% | Number of active maintainers distributing commits. |
| **Community Health** | 15% | Open issue/PR ratio and average response times. |
| **Dependents & Adoption** | 15% | Weekly downloads and dependent package metrics. |
| **License Compatibility** | 15% | License type and potential conflicts with standard project licenses. |
| **Dependency Tree Risk** | 10% | Known CVEs in transitive dependencies and abandoned sub-packages. |

**Thresholds:**
- `80-100`: Healthy - Safe for production use.
- `50-79`: Caution - Usable, but requires monitoring (e.g., low bus factor).
- `0-49`: Risky - Avoid unless absolutely necessary.

Note: The score is a signal, not an absolute metric. A low score on a feature-complete, stable package may simply indicate it no longer requires active maintenance. 

## Output Formats

```bash
pkgrisk check express                  # Human-readable colored terminal output (default)
pkgrisk check express --format json     # JSON format for automation
pkgrisk check express --format markdown # Markdown format for PRs/Issues
pkgrisk check express --quiet           # Minimal output (score and verdict only)
```

## Configuration

Configuration can be placed in `~/.config/pkgrisk/config.toml` or `.pkgriskrc.toml` at the project root:

```toml
[general]
cache_ttl_hours = 24
default_format = "terminal"

[thresholds]
fail_under = 40
warn_under = 65

[license]
project_license = "MIT"
blocklist = ["AGPL-3.0", "GPL-2.0"]

[ecosystems]
disabled = []
```

## Architecture

The project is structured as follows:

- `src/main.rs`: CLI entry point utilizing `clap`.
- `src/cli/`: Command implementations (`check`, `scan`, `compare`).
- `src/ecosystems/`: API clients for npm, PyPI, Crates.io, and OSV.dev.
- `src/scoring/`: Weighted algorithm logic per category.
- `src/cache/`: Local SQLite caching layer via `rusqlite`.
- `src/output/`: Formatting implementations (Terminal, JSON, Markdown).

**Data Sources:**
- npm: `registry.npmjs.org`
- PyPI: `pypi.org/pypi/<pkg>/json`
- crates.io: `crates.io/api/v1/`
- Vulnerabilities: `api.osv.dev`

All external API calls route through the local SQLite cache to respect rate limits.

## Development

```bash
git clone https://github.com/hasanerman/pkgrisk.git
cd pkgrisk

# Build the project
cargo build

# Run tests
cargo test

# Linting
cargo clippy -- -D warnings
```

**Environment Variables:**

To prevent rate-limiting when scanning large projects, supply a GitHub token:

```bash
export GITHUB_TOKEN=ghp_xxxxxxxxxxxx
```

## License

MIT License.
