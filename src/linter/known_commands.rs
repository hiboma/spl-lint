/// SPL コマンドエントリです。
pub struct CommandEntry {
    pub name: &'static str,
    pub category: &'static str,
}

/// SPL の組み込みコマンドのカテゴリ付きリストです。
/// Splunk SPL Search Reference v10.2 に基づきます。
/// https://help.splunk.com/en/splunk-enterprise/spl-search-reference/10.2/search-commands
pub static KNOWN_COMMAND_ENTRIES: &[CommandEntry] = &[
    // --- Search ---
    CommandEntry {
        name: "search",
        category: "Search",
    },
    CommandEntry {
        name: "where",
        category: "Search",
    },
    CommandEntry {
        name: "dedup",
        category: "Search",
    },
    CommandEntry {
        name: "head",
        category: "Search",
    },
    CommandEntry {
        name: "tail",
        category: "Search",
    },
    CommandEntry {
        name: "sample",
        category: "Search",
    },
    CommandEntry {
        name: "reverse",
        category: "Search",
    },
    CommandEntry {
        name: "return",
        category: "Search",
    },
    CommandEntry {
        name: "regex",
        category: "Search",
    },
    CommandEntry {
        name: "delete",
        category: "Search",
    },
    CommandEntry {
        name: "require",
        category: "Search",
    },
    // --- Reporting / Aggregation ---
    CommandEntry {
        name: "stats",
        category: "Reporting",
    },
    CommandEntry {
        name: "chart",
        category: "Reporting",
    },
    CommandEntry {
        name: "timechart",
        category: "Reporting",
    },
    CommandEntry {
        name: "top",
        category: "Reporting",
    },
    CommandEntry {
        name: "rare",
        category: "Reporting",
    },
    CommandEntry {
        name: "eventstats",
        category: "Reporting",
    },
    CommandEntry {
        name: "streamstats",
        category: "Reporting",
    },
    CommandEntry {
        name: "sistats",
        category: "Reporting",
    },
    CommandEntry {
        name: "sichart",
        category: "Reporting",
    },
    CommandEntry {
        name: "sitimechart",
        category: "Reporting",
    },
    CommandEntry {
        name: "sitop",
        category: "Reporting",
    },
    CommandEntry {
        name: "sirare",
        category: "Reporting",
    },
    CommandEntry {
        name: "geostats",
        category: "Reporting",
    },
    CommandEntry {
        name: "tstats",
        category: "Reporting",
    },
    CommandEntry {
        name: "mstats",
        category: "Reporting",
    },
    CommandEntry {
        name: "mcollect",
        category: "Reporting",
    },
    CommandEntry {
        name: "meventcollect",
        category: "Reporting",
    },
    CommandEntry {
        name: "tscollect",
        category: "Reporting",
    },
    // --- Eval / Calculation ---
    CommandEntry {
        name: "eval",
        category: "Eval",
    },
    CommandEntry {
        name: "addtotals",
        category: "Eval",
    },
    CommandEntry {
        name: "addcoltotals",
        category: "Eval",
    },
    CommandEntry {
        name: "addinfo",
        category: "Eval",
    },
    CommandEntry {
        name: "autoregress",
        category: "Eval",
    },
    CommandEntry {
        name: "accum",
        category: "Eval",
    },
    CommandEntry {
        name: "delta",
        category: "Eval",
    },
    CommandEntry {
        name: "gauge",
        category: "Eval",
    },
    CommandEntry {
        name: "rangemap",
        category: "Eval",
    },
    CommandEntry {
        name: "predict",
        category: "Eval",
    },
    CommandEntry {
        name: "trendline",
        category: "Eval",
    },
    CommandEntry {
        name: "x11",
        category: "Eval",
    },
    // --- Clustering / Anomaly ---
    CommandEntry {
        name: "anomalies",
        category: "Anomaly",
    },
    CommandEntry {
        name: "anomalousvalue",
        category: "Anomaly",
    },
    CommandEntry {
        name: "anomalydetection",
        category: "Anomaly",
    },
    CommandEntry {
        name: "kmeans",
        category: "Anomaly",
    },
    CommandEntry {
        name: "cluster",
        category: "Anomaly",
    },
    CommandEntry {
        name: "outlier",
        category: "Anomaly",
    },
    // --- Sort / Order ---
    CommandEntry {
        name: "sort",
        category: "Sort",
    },
    // --- Field manipulation ---
    CommandEntry {
        name: "fields",
        category: "Field",
    },
    CommandEntry {
        name: "rename",
        category: "Field",
    },
    CommandEntry {
        name: "table",
        category: "Field",
    },
    CommandEntry {
        name: "rex",
        category: "Field",
    },
    CommandEntry {
        name: "extract",
        category: "Field",
    },
    CommandEntry {
        name: "erex",
        category: "Field",
    },
    CommandEntry {
        name: "spath",
        category: "Field",
    },
    CommandEntry {
        name: "xmlkv",
        category: "Field",
    },
    CommandEntry {
        name: "kvform",
        category: "Field",
    },
    CommandEntry {
        name: "multikv",
        category: "Field",
    },
    CommandEntry {
        name: "makemv",
        category: "Field",
    },
    CommandEntry {
        name: "mvcombine",
        category: "Field",
    },
    CommandEntry {
        name: "mvexpand",
        category: "Field",
    },
    CommandEntry {
        name: "nomv",
        category: "Field",
    },
    CommandEntry {
        name: "reltime",
        category: "Field",
    },
    CommandEntry {
        name: "convert",
        category: "Field",
    },
    CommandEntry {
        name: "fillnull",
        category: "Field",
    },
    CommandEntry {
        name: "filldown",
        category: "Field",
    },
    CommandEntry {
        name: "replace",
        category: "Field",
    },
    CommandEntry {
        name: "strcat",
        category: "Field",
    },
    CommandEntry {
        name: "fieldformat",
        category: "Field",
    },
    CommandEntry {
        name: "bin",
        category: "Field",
    },
    CommandEntry {
        name: "bucket",
        category: "Field",
    },
    CommandEntry {
        name: "bucketdir",
        category: "Field",
    },
    CommandEntry {
        name: "untable",
        category: "Field",
    },
    CommandEntry {
        name: "xyseries",
        category: "Field",
    },
    CommandEntry {
        name: "ctable",
        category: "Field",
    },
    CommandEntry {
        name: "setfields",
        category: "Field",
    },
    CommandEntry {
        name: "abstract",
        category: "Field",
    },
    CommandEntry {
        name: "highlight",
        category: "Field",
    },
    CommandEntry {
        name: "iconify",
        category: "Field",
    },
    CommandEntry {
        name: "fieldsummary",
        category: "Field",
    },
    CommandEntry {
        name: "findtypes",
        category: "Field",
    },
    CommandEntry {
        name: "analyzefields",
        category: "Field",
    },
    CommandEntry {
        name: "makecontinuous",
        category: "Field",
    },
    // --- Lookup ---
    CommandEntry {
        name: "lookup",
        category: "Lookup",
    },
    CommandEntry {
        name: "inputlookup",
        category: "Lookup",
    },
    CommandEntry {
        name: "outputlookup",
        category: "Lookup",
    },
    CommandEntry {
        name: "inputcsv",
        category: "Lookup",
    },
    CommandEntry {
        name: "outputcsv",
        category: "Lookup",
    },
    // --- Join / Append ---
    CommandEntry {
        name: "join",
        category: "Join",
    },
    CommandEntry {
        name: "selfjoin",
        category: "Join",
    },
    CommandEntry {
        name: "append",
        category: "Join",
    },
    CommandEntry {
        name: "appendcols",
        category: "Join",
    },
    CommandEntry {
        name: "appendpipe",
        category: "Join",
    },
    CommandEntry {
        name: "multisearch",
        category: "Join",
    },
    CommandEntry {
        name: "union",
        category: "Join",
    },
    CommandEntry {
        name: "set",
        category: "Join",
    },
    // --- Transaction ---
    CommandEntry {
        name: "transaction",
        category: "Transaction",
    },
    CommandEntry {
        name: "concurrency",
        category: "Transaction",
    },
    // --- Formatting / Output ---
    CommandEntry {
        name: "format",
        category: "Format",
    },
    CommandEntry {
        name: "outputtext",
        category: "Format",
    },
    CommandEntry {
        name: "sendemail",
        category: "Format",
    },
    CommandEntry {
        name: "sendalert",
        category: "Format",
    },
    CommandEntry {
        name: "collect",
        category: "Format",
    },
    // --- Subsearch / Iteration ---
    CommandEntry {
        name: "map",
        category: "Subsearch",
    },
    CommandEntry {
        name: "foreach",
        category: "Subsearch",
    },
    // --- Metadata ---
    CommandEntry {
        name: "metadata",
        category: "Metadata",
    },
    CommandEntry {
        name: "metasearch",
        category: "Metadata",
    },
    CommandEntry {
        name: "dbinspect",
        category: "Metadata",
    },
    CommandEntry {
        name: "typeahead",
        category: "Metadata",
    },
    CommandEntry {
        name: "history",
        category: "Metadata",
    },
    CommandEntry {
        name: "rest",
        category: "Metadata",
    },
    CommandEntry {
        name: "eventcount",
        category: "Metadata",
    },
    CommandEntry {
        name: "walklex",
        category: "Metadata",
    },
    // --- Event types / Tags ---
    CommandEntry {
        name: "typer",
        category: "EventType",
    },
    CommandEntry {
        name: "typelearner",
        category: "EventType",
    },
    CommandEntry {
        name: "tags",
        category: "Tags",
    },
    // --- Macro / Saved searches / Scripts ---
    CommandEntry {
        name: "savedsearch",
        category: "Macro",
    },
    CommandEntry {
        name: "macro",
        category: "Macro",
    },
    CommandEntry {
        name: "run",
        category: "Macro",
    },
    CommandEntry {
        name: "script",
        category: "Macro",
    },
    // --- Data model ---
    CommandEntry {
        name: "datamodel",
        category: "DataModel",
    },
    CommandEntry {
        name: "datamodelsimple",
        category: "DataModel",
    },
    CommandEntry {
        name: "pivot",
        category: "DataModel",
    },
    // --- Data generation ---
    CommandEntry {
        name: "gentimes",
        category: "DataGen",
    },
    CommandEntry {
        name: "makeresults",
        category: "DataGen",
    },
    // --- Geo ---
    CommandEntry {
        name: "geom",
        category: "Geo",
    },
    CommandEntry {
        name: "geomfilter",
        category: "Geo",
    },
    CommandEntry {
        name: "iplocation",
        category: "Geo",
    },
    // --- XML / JSON ---
    CommandEntry {
        name: "xmlunescape",
        category: "XML",
    },
    CommandEntry {
        name: "xpath",
        category: "XML",
    },
    CommandEntry {
        name: "fromjson",
        category: "JSON",
    },
    CommandEntry {
        name: "tojson",
        category: "JSON",
    },
    // --- Database ---
    CommandEntry {
        name: "dbxquery",
        category: "Database",
    },
    // --- Data source ---
    CommandEntry {
        name: "from",
        category: "DataSource",
    },
    CommandEntry {
        name: "msearch",
        category: "DataSource",
    },
    CommandEntry {
        name: "mpreview",
        category: "DataSource",
    },
    CommandEntry {
        name: "loadjob",
        category: "DataSource",
    },
    CommandEntry {
        name: "inputintelligence",
        category: "DataSource",
    },
    CommandEntry {
        name: "ingestpreview",
        category: "DataSource",
    },
    // --- ServiceNow integration ---
    CommandEntry {
        name: "snowincident",
        category: "ServiceNow",
    },
    CommandEntry {
        name: "snowincidentstream",
        category: "ServiceNow",
    },
    CommandEntry {
        name: "snowevent",
        category: "ServiceNow",
    },
    CommandEntry {
        name: "snoweventstream",
        category: "ServiceNow",
    },
    // --- AWS integration ---
    CommandEntry {
        name: "awssnsalert",
        category: "AWS",
    },
    // --- Internal ---
    CommandEntry {
        name: "collapse",
        category: "Internal",
    },
    CommandEntry {
        name: "dump",
        category: "Internal",
    },
    CommandEntry {
        name: "findkeywords",
        category: "Internal",
    },
    CommandEntry {
        name: "makejson",
        category: "Internal",
    },
    CommandEntry {
        name: "mcatalog",
        category: "Internal",
    },
    CommandEntry {
        name: "noop",
        category: "Internal",
    },
    CommandEntry {
        name: "prjob",
        category: "Internal",
    },
    CommandEntry {
        name: "redistribute",
        category: "Internal",
    },
    CommandEntry {
        name: "runshellscript",
        category: "Internal",
    },
    // --- Misc ---
    CommandEntry {
        name: "transpose",
        category: "Misc",
    },
    CommandEntry {
        name: "contingency",
        category: "Misc",
    },
    CommandEntry {
        name: "correlate",
        category: "Misc",
    },
    CommandEntry {
        name: "associate",
        category: "Misc",
    },
    CommandEntry {
        name: "arules",
        category: "Misc",
    },
    CommandEntry {
        name: "diff",
        category: "Misc",
    },
    CommandEntry {
        name: "localop",
        category: "Misc",
    },
    CommandEntry {
        name: "uniq",
        category: "Misc",
    },
    CommandEntry {
        name: "overlap",
        category: "Misc",
    },
    CommandEntry {
        name: "cofilter",
        category: "Misc",
    },
    CommandEntry {
        name: "scrub",
        category: "Misc",
    },
    CommandEntry {
        name: "localize",
        category: "Misc",
    },
    CommandEntry {
        name: "folderize",
        category: "Misc",
    },
    CommandEntry {
        name: "entitymerge",
        category: "Misc",
    },
    CommandEntry {
        name: "rtorder",
        category: "Misc",
    },
    CommandEntry {
        name: "searchtxn",
        category: "Misc",
    },
    CommandEntry {
        name: "timewrap",
        category: "Misc",
    },
    CommandEntry {
        name: "audit",
        category: "Misc",
    },
    CommandEntry {
        name: "af",
        category: "Misc",
    },
    CommandEntry {
        name: "crawl",
        category: "Misc",
    },
    CommandEntry {
        name: "summary",
        category: "Misc",
    },
    CommandEntry {
        name: "movingavg",
        category: "Misc",
    },
    CommandEntry {
        name: "cefout",
        category: "Misc",
    },
];

/// コマンド名が組み込みコマンドとして認識されるか判定します。
pub fn is_known_command(name: &str) -> bool {
    KNOWN_COMMAND_ENTRIES
        .iter()
        .any(|c| c.name.eq_ignore_ascii_case(name))
}

/// 集約コマンドの一覧です。
static AGGREGATE_COMMANDS: &[&str] = &[
    "stats",
    "chart",
    "timechart",
    "top",
    "rare",
    "eventstats",
    "streamstats",
    "sistats",
    "geostats",
    "tstats",
    "mstats",
];

/// コマンド名が集約コマンドか判定します。
pub fn is_aggregate_command(name: &str) -> bool {
    AGGREGATE_COMMANDS
        .iter()
        .any(|&c| c.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_known_command() {
        assert!(is_known_command("stats"));
        assert!(is_known_command("eval"));
        assert!(is_known_command("table"));
    }

    #[test]
    fn test_known_command_case_insensitive() {
        assert!(is_known_command("Stats"));
        assert!(is_known_command("EVAL"));
    }

    #[test]
    fn test_unknown_command() {
        assert!(!is_known_command("foobar"));
        assert!(!is_known_command("notACommand"));
    }

    #[test]
    fn test_aggregate_command() {
        assert!(is_aggregate_command("stats"));
        assert!(is_aggregate_command("timechart"));
        assert!(!is_aggregate_command("eval"));
    }

    #[test]
    fn test_new_commands_from_v102() {
        assert!(is_known_command("awssnsalert"));
        assert!(is_known_command("cofilter"));
        assert!(is_known_command("fromjson"));
        assert!(is_known_command("tojson"));
        assert!(is_known_command("snowincident"));
        assert!(is_known_command("xpath"));
        assert!(is_known_command("iplocation"));
        assert!(is_known_command("geom"));
        assert!(is_known_command("entitymerge"));
        assert!(is_known_command("walklex"));
    }

    #[test]
    fn test_internal_commands() {
        assert!(is_known_command("collapse"));
        assert!(is_known_command("dump"));
        assert!(is_known_command("findkeywords"));
        assert!(is_known_command("makejson"));
        assert!(is_known_command("mcatalog"));
        assert!(is_known_command("noop"));
        assert!(is_known_command("prjob"));
        assert!(is_known_command("redistribute"));
        assert!(is_known_command("runshellscript"));
    }

    #[test]
    fn test_no_duplicate_entries() {
        let mut seen = HashSet::new();
        for entry in KNOWN_COMMAND_ENTRIES {
            assert!(
                seen.insert(entry.name),
                "duplicate command entry: {}",
                entry.name
            );
        }
    }
}
