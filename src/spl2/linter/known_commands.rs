/// SPL2 コマンドエントリです。
pub struct Spl2CommandEntry {
    pub name: &'static str,
    pub category: &'static str,
}

/// SPL2 の組み込みコマンドのカテゴリ付きリストです。
pub static KNOWN_SPL2_COMMAND_ENTRIES: &[Spl2CommandEntry] = &[
    // --- Data Source ---
    Spl2CommandEntry {
        name: "from",
        category: "DataSource",
    },
    Spl2CommandEntry {
        name: "into",
        category: "DataSource",
    },
    Spl2CommandEntry {
        name: "loadjob",
        category: "DataSource",
    },
    Spl2CommandEntry {
        name: "makeresults",
        category: "DataSource",
    },
    Spl2CommandEntry {
        name: "mstats",
        category: "DataSource",
    },
    Spl2CommandEntry {
        name: "tstats",
        category: "DataSource",
    },
    Spl2CommandEntry {
        name: "union",
        category: "DataSource",
    },
    // --- Search / Filter ---
    Spl2CommandEntry {
        name: "search",
        category: "Search",
    },
    Spl2CommandEntry {
        name: "where",
        category: "Search",
    },
    Spl2CommandEntry {
        name: "dedup",
        category: "Search",
    },
    Spl2CommandEntry {
        name: "head",
        category: "Search",
    },
    Spl2CommandEntry {
        name: "reverse",
        category: "Search",
    },
    // --- Reporting / Aggregation ---
    Spl2CommandEntry {
        name: "stats",
        category: "Reporting",
    },
    Spl2CommandEntry {
        name: "eventstats",
        category: "Reporting",
    },
    Spl2CommandEntry {
        name: "streamstats",
        category: "Reporting",
    },
    Spl2CommandEntry {
        name: "timechart",
        category: "Reporting",
    },
    Spl2CommandEntry {
        name: "timewrap",
        category: "Reporting",
    },
    // --- Eval / Calculation ---
    Spl2CommandEntry {
        name: "eval",
        category: "Eval",
    },
    Spl2CommandEntry {
        name: "bin",
        category: "Eval",
    },
    Spl2CommandEntry {
        name: "convert",
        category: "Eval",
    },
    Spl2CommandEntry {
        name: "addinfo",
        category: "Eval",
    },
    // --- Field manipulation ---
    Spl2CommandEntry {
        name: "fields",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "rename",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "table",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "rex",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "spath",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "makemv",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "mvcombine",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "mvexpand",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "nomv",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "fillnull",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "replace",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "untable",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "fieldsummary",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "flatten",
        category: "Field",
    },
    Spl2CommandEntry {
        name: "expand",
        category: "Field",
    },
    // --- Sort ---
    Spl2CommandEntry {
        name: "sort",
        category: "Sort",
    },
    // --- Lookup ---
    Spl2CommandEntry {
        name: "lookup",
        category: "Lookup",
    },
    // --- Join / Append ---
    Spl2CommandEntry {
        name: "join",
        category: "Join",
    },
    Spl2CommandEntry {
        name: "append",
        category: "Join",
    },
    Spl2CommandEntry {
        name: "appendcols",
        category: "Join",
    },
    Spl2CommandEntry {
        name: "appendpipe",
        category: "Join",
    },
    // --- Geo ---
    Spl2CommandEntry {
        name: "iplocation",
        category: "Geo",
    },
    // --- Security ---
    Spl2CommandEntry {
        name: "decrypt",
        category: "Security",
    },
    // --- Event types / Tags ---
    Spl2CommandEntry {
        name: "typer",
        category: "EventType",
    },
    Spl2CommandEntry {
        name: "tags",
        category: "Tags",
    },
    // --- Flow control ---
    Spl2CommandEntry {
        name: "branch",
        category: "Flow",
    },
    Spl2CommandEntry {
        name: "route",
        category: "Flow",
    },
    Spl2CommandEntry {
        name: "thru",
        category: "Flow",
    },
    // --- Interop ---
    Spl2CommandEntry {
        name: "spl1",
        category: "Interop",
    },
    // --- OCSF ---
    Spl2CommandEntry {
        name: "ocsf",
        category: "OCSF",
    },
];

/// コマンド名が SPL2 の組み込みコマンドとして認識されるか判定します。
pub fn is_known_spl2_command(name: &str) -> bool {
    KNOWN_SPL2_COMMAND_ENTRIES
        .iter()
        .any(|c| c.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_known_spl2_command() {
        assert!(is_known_spl2_command("from"));
        assert!(is_known_spl2_command("stats"));
        assert!(is_known_spl2_command("eval"));
        assert!(is_known_spl2_command("where"));
    }

    #[test]
    fn test_known_spl2_command_case_insensitive() {
        assert!(is_known_spl2_command("FROM"));
        assert!(is_known_spl2_command("Stats"));
    }

    #[test]
    fn test_unknown_spl2_command() {
        assert!(!is_known_spl2_command("foobar"));
        assert!(!is_known_spl2_command("notACommand"));
    }

    #[test]
    fn test_spl2_specific_commands() {
        assert!(is_known_spl2_command("spl1"));
        assert!(is_known_spl2_command("branch"));
        assert!(is_known_spl2_command("route"));
        assert!(is_known_spl2_command("thru"));
        assert!(is_known_spl2_command("ocsf"));
        assert!(is_known_spl2_command("flatten"));
        assert!(is_known_spl2_command("expand"));
        assert!(is_known_spl2_command("decrypt"));
    }

    #[test]
    fn test_no_duplicate_entries() {
        let mut seen = HashSet::new();
        for entry in KNOWN_SPL2_COMMAND_ENTRIES {
            assert!(
                seen.insert(entry.name),
                "duplicate SPL2 command entry: {}",
                entry.name
            );
        }
    }
}
