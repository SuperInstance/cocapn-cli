# cocapn-cli

**Fleet CLI theme and output formatting — the Abyssal Terminal aesthetic.**

Consistent, bioluminescent terminal output for every fleet tool. Dark backgrounds, cyan/magenta accents, amber warnings. Standardized `[TAG  ]` prefix format that both humans and agents can parse.

## Installation

```bash
cargo add cocapn-cli
```

## Usage

```rust
use cocapn_cli::{tag, TideBar, comparison_table};

// Standard fleet tags — 6-char aligned, colorized
println!("{} Deploying fleet stack...", tag("fleet"));
// → [FLEET ] Deploying fleet stack...

println!("{} Conservation law violation in room {}", tag("alert"), "research_log");
// → [ALERT ] Conservation law violation in room research_log

// Progress bars for long operations
let mut bar = TideBar::new(100, "Syncing rooms");
for i in 0..100 {
    bar.update(1, &format!("room_{i}"));
}
// → Syncing rooms [████████████████████] 100% room_99

// Comparison tables
let table = comparison_table(
    &["Model", "Speed", "Accuracy"],
    &[
        vec!["FLUX-VM".into(), "179M/s".into(), "99.7%".into()],
        vec!["Baseline".into(), "12M/s".into(), "94.2%".into()],
    ],
);
println!("{table}");
```

## The Aesthetic

The Abyssal Terminal theme draws from deep-ocean bioluminescence:

| Element | Color | Usage |
|---------|-------|-------|
| **Cyan** | `\x1b[36m` | Primary info, tags, labels |
| **Magenta** | `\x1b[35m` | Highlights, emphasis, data |
| **Amber** | `\x1b[33m` | Warnings, caution states |
| **Red** | `\x1b[31m` | Errors, violations, failures |
| **Green** | `\x1b[32m` | Success, pass, confirmed |
| **Dim** | `\x1b[2m` | Secondary info, timestamps |
| **Bold** | `\x1b[1m` | Headers, critical values |

## Modules

### `theme` — Color Palette & Tags
ANSI color constants and the standardized `[TAG  ]` format. Every fleet tool uses the same tag width for aligned, scannable output.

### `tide` — Tide Bar Progress
A progress bar named for the ocean tide: steady, rhythmic, inevitable. For any operation that takes more than a moment.

### `format` — Output Formatters
Markdown tables, Safe-TOPS/W comparison tables, and other structured output formats for fleet display.

## Why a Shared CLI Theme?

Fleet tools output to terminals read by both humans and agents. A consistent format means:
- **Agents can parse output** — regex-friendly `[TAG  ]` prefixes
- **Humans can scan output** — color-coded severity at a glance
- **Logs are uniform** — same format across every fleet tool

## License

MIT OR Apache-2.0
