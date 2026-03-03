/// SPL2 stats 関数エントリです。
pub struct Spl2StatsFunctionEntry {
    pub name: &'static str,
    pub category: &'static str,
}

/// SPL2 の stats 関数のカテゴリ付きリストです。
pub static KNOWN_SPL2_STATS_FUNCTION_ENTRIES: &[Spl2StatsFunctionEntry] = &[
    // Aggregate functions
    Spl2StatsFunctionEntry {
        name: "avg",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "count",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "dc",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "distinct_count",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "estdc",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "estdc_error",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "exactperc",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "max",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "mean",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "median",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "min",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "mode",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "perc",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "percentile",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "range",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "stdev",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "stdevp",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "sum",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "sumsq",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "upperperc",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "var",
        category: "Aggregate",
    },
    Spl2StatsFunctionEntry {
        name: "varp",
        category: "Aggregate",
    },
    // Event order functions
    Spl2StatsFunctionEntry {
        name: "earliest",
        category: "EventOrder",
    },
    Spl2StatsFunctionEntry {
        name: "earliest_time",
        category: "EventOrder",
    },
    Spl2StatsFunctionEntry {
        name: "first",
        category: "EventOrder",
    },
    Spl2StatsFunctionEntry {
        name: "last",
        category: "EventOrder",
    },
    Spl2StatsFunctionEntry {
        name: "latest",
        category: "EventOrder",
    },
    Spl2StatsFunctionEntry {
        name: "latest_time",
        category: "EventOrder",
    },
    // Multivalue stats functions
    Spl2StatsFunctionEntry {
        name: "list",
        category: "Multivalue",
    },
    Spl2StatsFunctionEntry {
        name: "values",
        category: "Multivalue",
    },
    // Rate functions
    Spl2StatsFunctionEntry {
        name: "per_day",
        category: "Rate",
    },
    Spl2StatsFunctionEntry {
        name: "per_hour",
        category: "Rate",
    },
    Spl2StatsFunctionEntry {
        name: "per_minute",
        category: "Rate",
    },
    Spl2StatsFunctionEntry {
        name: "per_second",
        category: "Rate",
    },
    Spl2StatsFunctionEntry {
        name: "rate",
        category: "Rate",
    },
    Spl2StatsFunctionEntry {
        name: "rate_avg",
        category: "Rate",
    },
    Spl2StatsFunctionEntry {
        name: "rate_sum",
        category: "Rate",
    },
    // SPL2 追加
    Spl2StatsFunctionEntry {
        name: "dataset",
        category: "SPL2",
    },
    Spl2StatsFunctionEntry {
        name: "pivot",
        category: "SPL2",
    },
    Spl2StatsFunctionEntry {
        name: "span",
        category: "SPL2",
    },
    Spl2StatsFunctionEntry {
        name: "sparkline",
        category: "SPL2",
    },
];

/// SPL2 の stats 関数名が組み込み関数として認識されるか判定します。
pub fn is_known_spl2_stats_function(name: &str) -> bool {
    KNOWN_SPL2_STATS_FUNCTION_ENTRIES
        .iter()
        .any(|f| f.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_known_spl2_stats_function() {
        assert!(is_known_spl2_stats_function("count"));
        assert!(is_known_spl2_stats_function("avg"));
        assert!(is_known_spl2_stats_function("dc"));
    }

    #[test]
    fn test_spl2_specific_stats_functions() {
        assert!(is_known_spl2_stats_function("dataset"));
        assert!(is_known_spl2_stats_function("pivot"));
        assert!(is_known_spl2_stats_function("span"));
        assert!(is_known_spl2_stats_function("sparkline"));
    }

    #[test]
    fn test_unknown_spl2_stats_function() {
        assert!(!is_known_spl2_stats_function("foobar"));
    }

    #[test]
    fn test_no_duplicate_entries() {
        let mut seen = HashSet::new();
        for entry in KNOWN_SPL2_STATS_FUNCTION_ENTRIES {
            assert!(
                seen.insert(entry.name),
                "duplicate SPL2 stats function entry: {}",
                entry.name
            );
        }
    }
}
