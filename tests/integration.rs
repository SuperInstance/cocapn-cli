use cocapn_cli::*;

#[test]
fn test_comparison_table_basic() {
    let table = comparison_table(
        &["Name", "Value"],
        &[vec!["a".into(), "1".into()], vec!["b".into(), "2".into()]],
    );
    assert!(table.contains("Name"));
    assert!(table.contains("Value"));
    assert!(table.contains("a"));
    assert!(table.contains("---"));
}

#[test]
fn test_comparison_table_empty() {
    let table = comparison_table(&["Col"], &[]);
    assert!(table.contains("Col"));
}
