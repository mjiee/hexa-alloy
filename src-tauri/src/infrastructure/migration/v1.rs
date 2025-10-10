pub const SQL: &str = r#"
CREATE TABLE IF NOT EXISTS system_configs (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
);
"#;
