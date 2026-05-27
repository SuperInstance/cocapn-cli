//! Fleet color palette — bioluminescent terminal aesthetic.

/// Fleet color constants (ANSI escape codes)
pub mod colors {
    pub const CYAN: &str = "\x1b[36m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const AMBER: &str = "\x1b[33m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const DIM: &str = "\x1b[2m";
    pub const BOLD: &str = "\x1b[1m";
    pub const RESET: &str = "\x1b[0m";
}

/// Fleet tag format: `[TAG  ]` with consistent 6-char width
pub fn tag(label: &str) -> String {
    format!("\x1b[36m[{:6}]\x1b[0m", label.to_uppercase())
}

/// Standard tags for fleet operations
pub mod tags {
    use super::tag;
    pub fn plato() -> String {
        tag("plato")
    }
    pub fn valid() -> String {
        tag("valid")
    }
    pub fn ask() -> String {
        tag("ask")
    }
    pub fn rank() -> String {
        tag("rank")
    }
    pub fn import() -> String {
        tag("import")
    }
    pub fn flux() -> String {
        tag("flux")
    }
    pub fn guard() -> String {
        tag("guard")
    }
    pub fn deploy() -> String {
        tag("deploy")
    }
    pub fn certify() -> String {
        tag("cert")
    }
    pub fn error() -> String {
        tag("error")
    }
    pub fn warn() -> String {
        tag("warn")
    }
}

/// Format a progress line: `[TAG] ████████░░ 60% | detail`
pub fn progress(tag_str: &str, current: usize, total: usize, detail: &str) -> String {
    let pct = current.checked_div(total).map_or(100, |q| q * 100);
    let filled = pct / 5;
    let empty = 20 - filled;
    let bar: String = "█".repeat(filled) + &"░".repeat(empty);
    format!("{} {} {}% | {}", tag_str, bar, pct, detail)
}

/// Format a health status line: `│ Label │ value │ status │`
pub fn health_line(label: &str, value: &str, ok: bool) -> String {
    let status = if ok { "✅" } else { "🔴" };
    format!("│ {:18} │ {:10} │ {} │", label, value, status)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- tag() ---

    #[test]
    fn test_tag_format() {
        let t = tag("plato");
        assert!(t.contains("PLATO"));
    }

    #[test]
    fn test_tag_uppercases_label() {
        let t = tag("flux");
        assert!(t.contains("FLUX"));
        assert!(t.contains("\x1b[36m")); // cyan
    }

    #[test]
    fn test_tag_pads_to_6_chars() {
        let t = tag("ok");
        // strip ANSI to check padding
        let stripped = strip_ansi(&t);
        assert!(stripped.starts_with("[OK    ]"));
    }

    // --- tags module ---

    #[test]
    fn test_all_tags_contain_correct_label() {
        let cases = vec![
            (tags::plato(), "PLATO"),
            (tags::valid(), "VALID"),
            (tags::ask(), "ASK  "),
            (tags::rank(), "RANK"),
            (tags::import(), "IMPORT"),
            (tags::flux(), "FLUX"),
            (tags::guard(), "GUARD"),
            (tags::deploy(), "DEPLOY"),
            (tags::certify(), "CERT"),
            (tags::error(), "ERROR"),
            (tags::warn(), "WARN"),
        ];
        for (tag_str, label) in cases {
            let stripped = strip_ansi(&tag_str);
            assert!(
                stripped.contains(label),
                "tag should contain {:?}, got {:?}",
                label, stripped
            );
        }
    }

    #[test]
    fn test_tags_all_start_with_cyan() {
        assert!(tags::plato().starts_with("\x1b[36m"));
        assert!(tags::error().starts_with("\x1b[36m"));
        assert!(tags::warn().starts_with("\x1b[36m"));
    }

    // --- progress() ---

    #[test]
    fn test_progress() {
        let p = progress("[PLATO]", 50, 100, "test");
        assert!(p.contains("%"));
        assert!(p.contains("test"));
        assert!(p.contains("[PLATO]"));
    }

    #[test]
    fn test_progress_zero_total_returns_100() {
        // checked_div returns None when total=0, maps to 100%
        let p = progress("[TAG]", 5, 0, "no total");
        assert!(p.contains("100%"));
    }

    #[test]
    fn test_progress_full_bar() {
        let p = progress("[TAG]", 100, 100, "done");
        assert!(p.contains("100%"));
        // bar should be all filled
        assert!(p.contains(&"█".repeat(20)));
    }

    #[test]
    fn test_progress_contains_detail() {
        let p = progress("[X]", 0, 10, "my-detail");
        assert!(p.contains("my-detail"));
    }

    // --- health_line() ---

    #[test]
    fn test_health_line_ok() {
        let h = health_line("Tests", "26", true);
        assert!(h.contains("✅"));
        assert!(h.contains("Tests"));
        assert!(h.contains("26"));
    }

    #[test]
    fn test_health_line_fail() {
        let h = health_line("Memory", "low", false);
        assert!(h.contains("🔴"));
        assert!(h.contains("Memory"));
        assert!(h.contains("low"));
    }

    #[test]
    fn test_health_line_has_borders() {
        let h = health_line("X", "Y", true);
        assert!(h.starts_with('│'));
        assert!(h.contains("│"));
    }

    // --- colors constants ---

    #[test]
    fn test_color_constants_are_valid_ansi() {
        assert!(colors::CYAN.starts_with("\x1b["));
        assert!(colors::MAGENTA.starts_with("\x1b["));
        assert!(colors::AMBER.starts_with("\x1b["));
        assert!(colors::RED.starts_with("\x1b["));
        assert!(colors::GREEN.starts_with("\x1b["));
        assert!(colors::DIM.starts_with("\x1b["));
        assert!(colors::BOLD.starts_with("\x1b["));
        assert!(colors::RESET.starts_with("\x1b["));
    }

    #[test]
    fn test_color_reset_value() {
        assert_eq!(colors::RESET, "\x1b[0m");
    }

    // --- helper ---

    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                if chars.peek() == Some(&'[') {
                    chars.next();
                    while let Some(&nc) = chars.peek() {
                        chars.next();
                        if nc.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}
