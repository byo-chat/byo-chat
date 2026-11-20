#[cfg(test)]
mod tests {
    use std::path::Path;

    const STATUSES: &[&str] = &["proposed", "accepted", "deferred", "rejected"];

    fn has_closed_status(text: &str) -> bool {
        text.lines()
            .find_map(|line| line.strip_prefix("Status: "))
            .is_some_and(|status| {
                STATUSES.contains(&status) || status.starts_with("superseded by ADR-")
            })
    }

    #[test]
    fn status_line_must_come_from_the_closed_set() {
        let cases = [
            ("# T\n\nStatus: accepted\n", true),
            ("# T\n\nStatus: superseded by ADR-0002\n", true),
            ("# T\n\nStatus: done\n", false),
            ("# T\n\nno status\n", false),
        ];
        for (text, expected) in cases {
            assert_eq!(has_closed_status(text), expected, "{text}");
        }
    }

    #[test]
    fn every_adr_has_a_closed_status() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/adr");
        let mut checked = 0;
        for entry in std::fs::read_dir(dir).expect("docs/adr exists") {
            let path = entry.expect("readable entry").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if !name.starts_with(|c: char| c.is_ascii_digit()) {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable ADR");
            assert!(
                has_closed_status(&text),
                "{name} lacks a closed-set Status line"
            );
            checked += 1;
        }
        assert!(checked > 0, "no ADR was read");
    }
}
