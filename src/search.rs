//! Product-neutral, deterministic filtering for demo tables.
///
/// Every whitespace-separated term must match at least one visible field.
/// Unicode lowercase conversion keeps Chinese labels intact and makes
/// English executable names case-insensitive.
pub fn matches_fields(query: &str, fields: &[&str]) -> bool {
    query.split_whitespace().all(|word| {
        let term = word.to_lowercase();
        fields.iter().any(|field| field.to_lowercase().contains(&term))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_across_columns_with_and_semantics() {
        let row = ["Windows Search", "运行中", "自动", "SearchHost.exe", "文件索引"];
        assert!(matches_fields("windows 运行", &row));
        assert!(matches_fields("SEARCHHOST 文件", &row));
        assert!(matches_fields("  ", &row));
        assert!(!matches_fields("windows 已停止", &row));
    }

    #[test]
    fn supports_unicode_and_empty_rows() {
        assert!(matches_fields("应用", &["Windows 应用", "运行中"]));
        assert!(!matches_fields("ANY", &[]));
        assert!(matches_fields("", &[]));
    }
}
