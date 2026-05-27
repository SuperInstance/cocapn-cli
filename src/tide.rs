//! Tide bar — progress indicator for long operations.
//! Named for the ocean tide: steady, rhythmic, inevitable.

use std::io::{self, Write};

/// A simple progress bar for fleet operations
pub struct TideBar {
    total: usize,
    current: usize,
    label: String,
}

impl TideBar {
    pub fn new(total: usize, label: &str) -> Self {
        Self {
            total,
            current: 0,
            label: label.to_string(),
        }
    }

    pub fn update(&mut self, delta: usize, detail: &str) {
        self.current = (self.current + delta).min(self.total);
        let pct = self
            .current
            .checked_div(self.total)
            .map_or(100, |q| q * 100);
        let filled = pct / 5;
        let empty = 20 - filled;
        let bar: String = "▓".repeat(filled) + &"░".repeat(empty);
        eprint!(
            "\r[{}] {} {}/{} ({}%) | {}",
            self.label, bar, self.current, self.total, pct, detail
        );
        io::stderr().flush().ok();
    }

    pub fn finish(&mut self) {
        eprintln!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tide_bar_create() {
        let bar = TideBar::new(100, "IMPORT");
        assert_eq!(bar.total, 100);
        assert_eq!(bar.current, 0);
        assert_eq!(bar.label, "IMPORT");
    }

    #[test]
    fn test_tide_bar_create_zero_total() {
        let bar = TideBar::new(0, "EMPTY");
        assert_eq!(bar.total, 0);
        assert_eq!(bar.current, 0);
    }

    #[test]
    fn test_tide_bar_update_advances_current() {
        let mut bar = TideBar::new(100, "TEST");
        bar.update(25, "quarter");
        assert_eq!(bar.current, 25);
    }

    #[test]
    fn test_tide_bar_update_clamps_at_total() {
        let mut bar = TideBar::new(50, "TEST");
        bar.update(999, "overflow");
        assert_eq!(bar.current, 50); // clamped to total
    }

    #[test]
    fn test_tide_bar_multiple_updates() {
        let mut bar = TideBar::new(100, "STEP");
        bar.update(10, "a");
        assert_eq!(bar.current, 10);
        bar.update(20, "b");
        assert_eq!(bar.current, 30);
        bar.update(70, "c");
        assert_eq!(bar.current, 100);
    }

    #[test]
    fn test_tide_bar_finish_does_not_panic() {
        let mut bar = TideBar::new(10, "FIN");
        bar.update(5, "mid");
        bar.finish(); // should not panic
    }

    #[test]
    fn test_tide_bar_label_stored() {
        let bar = TideBar::new(1, "my-label");
        assert_eq!(bar.label, "my-label");
    }
}
