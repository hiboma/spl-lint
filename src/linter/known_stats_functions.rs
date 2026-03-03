/// stats 関数エントリです。
pub struct StatsFunctionEntry {
    pub name: &'static str,
    pub category: &'static str,
}

/// SPL の stats 関数のカテゴリ付きリストです。
/// Splunk SPL Search Reference v10.2 に基づきます。
/// https://help.splunk.com/en/splunk-enterprise/spl-search-reference/10.2/statistical-and-charting-functions
pub static KNOWN_STATS_FUNCTION_ENTRIES: &[StatsFunctionEntry] = &[
    // Aggregate functions
    StatsFunctionEntry {
        name: "avg",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "count",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "dc",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "distinct_count",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "estdc",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "estdc_error",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "exactperc",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "max",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "mean",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "median",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "min",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "mode",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "perc",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "percentile",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "range",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "stdev",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "stdevp",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "sum",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "sumsq",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "upperperc",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "var",
        category: "Aggregate",
    },
    StatsFunctionEntry {
        name: "varp",
        category: "Aggregate",
    },
    // Event order functions
    StatsFunctionEntry {
        name: "earliest",
        category: "EventOrder",
    },
    StatsFunctionEntry {
        name: "earliest_time",
        category: "EventOrder",
    },
    StatsFunctionEntry {
        name: "first",
        category: "EventOrder",
    },
    StatsFunctionEntry {
        name: "last",
        category: "EventOrder",
    },
    StatsFunctionEntry {
        name: "latest",
        category: "EventOrder",
    },
    StatsFunctionEntry {
        name: "latest_time",
        category: "EventOrder",
    },
    // Multivalue stats functions
    StatsFunctionEntry {
        name: "list",
        category: "Multivalue",
    },
    StatsFunctionEntry {
        name: "values",
        category: "Multivalue",
    },
    // Rate functions
    StatsFunctionEntry {
        name: "per_day",
        category: "Rate",
    },
    StatsFunctionEntry {
        name: "per_hour",
        category: "Rate",
    },
    StatsFunctionEntry {
        name: "per_minute",
        category: "Rate",
    },
    StatsFunctionEntry {
        name: "per_second",
        category: "Rate",
    },
    StatsFunctionEntry {
        name: "rate",
        category: "Rate",
    },
    StatsFunctionEntry {
        name: "rate_avg",
        category: "Rate",
    },
    StatsFunctionEntry {
        name: "rate_sum",
        category: "Rate",
    },
];

/// stats 関数名が組み込み関数として認識されるか判定します。
pub fn is_known_stats_function(name: &str) -> bool {
    KNOWN_STATS_FUNCTION_ENTRIES
        .iter()
        .any(|f| f.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_known_stats_function() {
        assert!(is_known_stats_function("count"));
        assert!(is_known_stats_function("avg"));
        assert!(is_known_stats_function("dc"));
    }

    #[test]
    fn test_known_stats_function_case_insensitive() {
        assert!(is_known_stats_function("COUNT"));
        assert!(is_known_stats_function("Avg"));
    }

    #[test]
    fn test_unknown_stats_function() {
        assert!(!is_known_stats_function("foobar"));
    }

    #[test]
    fn test_new_functions_from_v102() {
        assert!(is_known_stats_function("rate"));
        assert!(is_known_stats_function("rate_avg"));
        assert!(is_known_stats_function("rate_sum"));
    }

    #[test]
    fn test_no_duplicate_entries() {
        let mut seen = HashSet::new();
        for entry in KNOWN_STATS_FUNCTION_ENTRIES {
            assert!(
                seen.insert(entry.name),
                "duplicate stats function entry: {}",
                entry.name
            );
        }
    }
}
