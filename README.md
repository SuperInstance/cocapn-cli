# cocapn-cli — Fleet Terminal Formatting

**Consistent, parseable terminal output for every tool in the Cocapn fleet. Zero dependencies, `no_std`-friendly Rust.**

## What This Gives You

- **ANSI color palette** — bioluminescent terminal aesthetic (cyan/magenta/amber) applied consistently across all fleet tools
- **`[TAG  ]` prefix format** — fixed-width tags (`[FLUX  ]`, `[PLATO ]`, `[VALID ]`) for agent-parseable log lines
- **Markdown table formatter** — aligned comparison tables from structured data
- **Tide progress bar** — `▓░` progress indicators for long-running fleet operations
- **Zero dependencies** — pure Rust, works in `no_std` contexts

## Quick Start

```toml
# Cargo.toml
[dependencies]
cocapn-cli = "0.1"
```

```rust
use cocapn_cli::{tag, colors, TideBar, comparison_table};

// Tagged output — every fleet tool uses the same format
println!("{} System check passed", tag("valid"));

// Colored output
println!("{}{}Critical alert{}", colors::AMBER, colors::BOLD, colors::RESET);

// Progress bar
let mut bar = TideBar::new(100, "BENCHMARK");
for i in 0..100 {
    bar.update(1, &format!("run {}", i));
}
bar.finish();

// Comparison tables
let table = comparison_table(
    &["Model", "Latency", "Cost"],
    &[
        vec!["claude-3.5-sonnet".into(), "1.2s".into(), "$0.003".into()],
        vec!["gpt-4o".into(), "0.8s".into(), "$0.005".into()],
    ],
);
println!("{}", table);
```

## API Reference

### `colors` — ANSI Constants
`CYAN`, `MAGENTA`, `AMBER`, `RED`, `GREEN`, `DIM`, `BOLD`, `RESET`

### `tag(label: &str) -> String`
Formats a fixed-width `[TAG   ]` prefix with cyan coloring.

### `tags` — Standard Fleet Tags
`plato()`, `valid()`, `ask()`, `rank()`, `import()`, `flux()`

### `TideBar`
```rust
TideBar::new(total: usize, label: &str) -> Self
bar.update(delta: usize, detail: &str)
bar.finish()
```

### `comparison_table(headers, rows) -> String`
Generates an aligned markdown table.

## How It Fits
- [OpenConstruct Documentation](https://github.com/SuperInstance/openconstruct-docs) — ecosystem-wide docs and guides

The formatting backbone for the [SuperInstance fleet](https://github.com/SuperInstance). Every fleet tool — benchmarks, health checks, explainability reports — uses `cocapn-cli` for uniform output that both humans and agents can parse.

- **[cocapn](https://github.com/SuperInstance/cocapn)** — Core agent infrastructure
- **[cocapn-health-rs](https://github.com/SuperInstance/cocapn-health-rs)** — Fleet health monitoring (uses this for output)
- **[cocapn-benchmark](https://github.com/SuperInstance/cocapn-benchmark)** — Performance testing (uses this for tables)

## Testing

```bash
cargo test
```

## Installation

```toml
[dependencies]
cocapn-cli = "0.1"
```

MIT OR Apache-2.0.
