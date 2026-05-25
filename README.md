# cocapn-cli

**Fleet CLI theme and output formatting — the Abyssal Terminal aesthetic.**

Consistent, bioluminescent terminal output for every fleet tool. Dark backgrounds, cyan/magenta accents, amber warnings. Standardized `[TAG  ]` prefix format that both humans and agents can parse.

A Rust library (`no_std`-friendly, zero dependencies) providing ANSI color constants, progress bars, and markdown table formatters. Every tool in the Cocapn fleet uses `cocapn-cli` for uniform, parseable terminal output.

## Installation

```bash
cargo add cocapn-cli
```

## Usage

### Standard Fleet Tags

```rust
use cocapn_cli::{tag, tags};

// Generic tag — 6-char aligned, cyan-colored
println!("{} Deploying fleet stack...", tag("fleet"));
// → [FLEET ] Deploying fleet stack...

// Predefined fleet tags
println!("{} Room synced", tags::plato());
println!("{} Constraint verified", tags::valid());
println!("{} Model ranking complete", tags::rank());
println!("{} Conservation law violation", tags::warn());
println!("{} Tile submitted to PLATO", tags::flux());
println!("{} Fleet deployed", tags::deploy());
```

Available predefined tags: `plato()`, `valid()`, `ask()`, `rank()`, `import()`, `flux()`, `guard()`, `deploy()`, `certify()`, `error()`, `warn()`.

### Progress Reporting

```rust
use cocapn_cli::{tag, progress};

// One-shot progress line
let line = progress("[PLATO]", 60, 100, "syncing tiles");
println!("{line}");
// → [PLATO] ████████████░░░░░░░░ 60% | syncing tiles
```

### Health Status Lines

```rust
use cocapn_cli::health_line;

println!("{}", health_line("Tests", "26", true));
// → │ Tests              │         26 │ ✅ │

println!("{}", health_line("Conservation", "FAIL", false));
// → │ Conservation       │       FAIL │ 🔴 │
```

### Tide Bar (Streaming Progress)

```rust
use cocapn_cli::TideBar;

let mut bar = TideBar::new(100, "Syncing rooms");
for i in 0..100 {
    bar.update(1, &format!("room_{i}"));
}
bar.finish();
// → [Syncing rooms] ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ 100/100 (100%) | room_99
```

Named for the ocean tide: steady, rhythmic, inevitable. For any operation that takes more than a moment.

### Comparison Tables

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

### Safe-TOPS/W Hardware Comparison

```rust
use cocapn_cli::safe_tops_w_table;

println!("{}", safe_tops_w_table());
// Pre-built table comparing FLUX-LUCID, Jetson Orin AGX, Hailo-8 Safety,
// Groq LPU, Google TPU v5e, Mobileye EyeQ6H on Safe-TOPS/W metric.
```

## The Abyssal Terminal Aesthetic

The theme draws from deep-ocean bioluminescence — designed for dark terminal backgrounds where cyan and magenta glow against the void.

| Element | ANSI Code | Usage |
|---------|-----------|-------|
| **Cyan** | `\x1b[36m` | Primary info, tags, labels |
| **Magenta** | `\x1b[35m` | Highlights, emphasis, data |
| **Amber** | `\x1b[33m` | Warnings, caution states |
| **Red** | `\x1b[31m` | Errors, violations, failures |
| **Green** | `\x1b[32m` | Success, pass, confirmed |
| **Dim** | `\x1b[2m` | Secondary info, timestamps |
| **Bold** | `\x1b[1m` | Headers, critical values |

## Module Reference

### `theme` — Color Palette & Tags

- `colors::{CYAN, MAGENTA, AMBER, RED, GREEN, DIM, BOLD, RESET}` — Raw ANSI escape constants
- `tag(label: &str) -> String` — Format a `[TAG  ]` string (6-char padded, cyan)
- `tags::plato()`, `tags::valid()`, etc. — Predefined fleet tag functions
- `progress(tag, current, total, detail) -> String` — Formatted progress bar string
- `health_line(label, value, ok) -> String` — Status row with ✅/🔴 indicator

### `tide` — Tide Bar Progress

- `TideBar::new(total, label)` — Create a new progress bar
- `bar.update(delta, detail)` — Advance progress and redraw to stderr
- `bar.finish()` — Print final newline

### `format` — Output Formatters

- `comparison_table(headers, rows) -> String` — Markdown table from headers and row data
- `safe_tops_w_table() -> String` — Pre-built Safe-TOPS/W hardware comparison table

## Why a Shared CLI Theme?

Fleet tools output to terminals read by both humans and agents. A consistent format means:

- **Agents can parse output** — regex-friendly `[TAG  ]` prefixes with fixed-width alignment
- **Humans can scan output** — color-coded severity at a glance, aligned columns
- **Logs are uniform** — same format across every fleet tool, grep-friendly
- **Branding is coherent** — the Abyssal Terminal look is instantly recognizable

## Design Principles

1. **Zero dependencies** — no external crates, just stdlib ANSI codes
2. **Fixed-width tags** — 6 chars ensures aligned output regardless of tag name
3. **Color + text** — every colored output includes the text fallback for non-TTY contexts
4. **Testable** — string-returning functions, no side effects except `TideBar` (stderr)

## Related SuperInstance Repos

| Repo | Description |
|------|-------------|
| [flux-verify-api](https://github.com/SuperInstance/flux-verify-api) | Verification API using this theme for output |
| [fleet-stack](https://github.com/SuperInstance/fleet-stack) | Docker deployment with themed service logs |
| [quality-gate-stream](https://github.com/SuperInstance/quality-gate-stream) | Quality scoring with fleet-formatted output |
| [plato-core](https://github.com/SuperInstance/plato-core) | PLATO server using fleet tags in logs |
| [cocapn-traps](https://github.com/SuperInstance/cocapn-traps) | Trap management using themed CLI output |

## License

MIT OR Apache-2.0
