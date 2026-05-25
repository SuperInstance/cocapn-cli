# cocapn-cli

Rust library for consistent terminal output formatting across fleet tools.

Zero dependencies. `no_std`-friendly. Provides ANSI color constants, fixed-width `[TAG  ]` prefixes, progress bars, and markdown table formatters. Every tool in the Cocapn fleet uses this for uniform, parseable terminal output.

## Install

```bash
cargo add cocapn-cli
```

## Tags

Fleet output uses a fixed-width `[TAG  ]` prefix — 6 chars, left-aligned, cyan ANSI. Both humans and agents can parse it at a glance.

```rust
use cocapn_cli::{tag, tags};

// Any string → [TAG  ] format (cyan, 6-char padded)
println!("{} Deploying fleet stack...", tag("fleet"));
// [FLEET ] Deploying fleet stack...

println!("{} Room synced", tags::plato());
println!("{} Constraint verified", tags::valid());
println!("{} Model ranking complete", tags::rank());
println!("{} Conservation law violation", tags::warn());
println!("{} Tile submitted to PLATO", tags::flux());
println!("{} Fleet deployed", tags::deploy());
```

Available tags: `plato()`, `valid()`, `ask()`, `rank()`, `import()`, `flux()`, `guard()`, `deploy()`, `certify()`, `error()`, `warn()`.

## Progress Bar

```rust
use cocapn_cli::{tag, progress};

// One-shot: [TAG] ████████░░ 60% | detail
let line = progress("[PLATO]", 60, 100, "syncing tiles");
println!("{line}");
// [PLATO] ████████░░░░░░░░░░░░ 60% | syncing tiles
```

## Streaming Progress (TideBar)

For operations that run over time — file imports, network syncs, batch processing:

```rust
use cocapn_cli::TideBar;

let mut bar = TideBar::new(100, "IMPORT");
for i in 0..100 {
    bar.update(1, &format!("item_{i}"));
    // \r[IMPORT] ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ 100/100 (100%) | item_99
}
bar.finish();
```

`TideBar` writes to stderr with `\r` carriage returns — no screen spam in piped output.

## Health Status Lines

```rust
use cocapn_cli::health_line;

println!("{}", health_line("Tests", "26", true));
// │ Tests              │         26 │ ✅ │

println!("{}", health_line("Conservation", "FAIL", false));
// │ Conservation       │       FAIL │ 🔴 │
```

18-char label, 10-char value, emoji status. Fixed-width columns align across all fleet tools.

## Markdown Tables

```rust
use cocapn_cli::comparison_table;

let table = comparison_table(
    &["Model", "Speed", "Accuracy"],
    &[
        vec!["FLUX-VM".into(), "179M/s".into(), "99.7%".into()],
        vec!["Baseline".into(), "12M/s".into(), "94.2%".into()],
    ],
);
println!("{table}");
// | Model | Speed | Accuracy |
// |---|---|---|
// | FLUX-VM | 179M/s | 99.7% |
// | Baseline | 12M/s | 94.2% |
```

Also includes `safe_tops_w_table()` for a pre-built hardware comparison table.

## Color Constants

Raw ANSI escape strings — use them directly when you need custom formatting:

```rust
use cocapn_cli::colors::*;

println!("{CYAN}info{RESET}: something happened");
println!("{AMBER}warn{RESET}: check this");
println!("{RED}error{RESET}: something broke");
println!("{GREEN}ok{RESET}: all clear");
println!("{DIM}2024-05-23 15:06:00{RESET} {BOLD}header{RESET}");
```

| Constant | ANSI | Use for |
|----------|------|---------|
| `CYAN` | `\x1b[36m` | Tags, labels, primary info |
| `MAGENTA` | `\x1b[35m` | Highlights, data values |
| `AMBER` | `\x1b[33m` | Warnings |
| `RED` | `\x1b[31m` | Errors, failures |
| `GREEN` | `\x1b[32m` | Success, confirmed |
| `DIM` | `\x1b[2m` | Timestamps, secondary info |
| `BOLD` | `\x1b[1m` | Headers, critical values |
| `RESET` | `\x1b[0m` | Reset all formatting |

## Module Reference

| Module | What | Key Functions |
|--------|------|---------------|
| `theme` | Colors, tags, progress, health_line | `tag()`, `tags::*`, `progress()`, `health_line()` |
| `tide` | Streaming progress bar | `TideBar::new()`, `.update()`, `.finish()` |
| `format` | Table formatters | `comparison_table()`, `safe_tops_w_table()` |

## Why This Exists

Fleet tools output to terminals read by both humans and agents. A shared format means:

- Agents can parse `[TAG  ]` prefixes with regex (fixed 6-char width)
- Humans can scan color-coded severity at a glance
- Logs look the same across every fleet tool
- No external formatting dependencies in any fleet crate

## Tests

```bash
cargo test
# 5 tests: tag format, progress output, health line, comparison table, Safe-TOPS/W table
```

## Related Fleet Repos

| Repo | What |
|------|------|
| [cocapn-traps](https://github.com/SuperInstance/cocapn-traps) | Crab trap management |
| [cocapn-health](https://github.com/SuperInstance/cocapn-health) | Fleet health monitoring |
| [cocapn-glue-core](https://github.com/SuperInstance/cocapn-glue-core) | Binary wire protocol |
| [cocapn-plato](https://github.com/SuperInstance/cocapn-plato) | PLATO engine + SDK |

## License

MIT OR Apache-2.0
