/// eval 関数エントリです。
pub struct EvalFunctionEntry {
    pub name: &'static str,
    pub category: &'static str,
}

/// SPL の eval 関数のカテゴリ付きリストです。
/// Splunk SPL Search Reference v10.2 に基づきます。
/// https://help.splunk.com/en/splunk-enterprise/spl-search-reference/10.2/evaluation-functions
pub static KNOWN_EVAL_FUNCTION_ENTRIES: &[EvalFunctionEntry] = &[
    // --- Bitwise ---
    EvalFunctionEntry {
        name: "bit_and",
        category: "Bitwise",
    },
    EvalFunctionEntry {
        name: "bit_or",
        category: "Bitwise",
    },
    EvalFunctionEntry {
        name: "bit_not",
        category: "Bitwise",
    },
    EvalFunctionEntry {
        name: "bit_xor",
        category: "Bitwise",
    },
    EvalFunctionEntry {
        name: "bit_shift_left",
        category: "Bitwise",
    },
    EvalFunctionEntry {
        name: "bit_shift_right",
        category: "Bitwise",
    },
    // --- Comparison / Conditional ---
    EvalFunctionEntry {
        name: "case",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "cidrmatch",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "coalesce",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "false",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "if",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "in",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "like",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "lookup",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "match",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "null",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "nullif",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "searchmatch",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "true",
        category: "Conditional",
    },
    EvalFunctionEntry {
        name: "validate",
        category: "Conditional",
    },
    // --- Conversion ---
    EvalFunctionEntry {
        name: "ipmask",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "printf",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "toarray",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "tobool",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "todouble",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "toint",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "tomv",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "tonumber",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "toobject",
        category: "Conversion",
    },
    EvalFunctionEntry {
        name: "tostring",
        category: "Conversion",
    },
    // --- Cryptographic ---
    EvalFunctionEntry {
        name: "md5",
        category: "Cryptographic",
    },
    EvalFunctionEntry {
        name: "sha1",
        category: "Cryptographic",
    },
    EvalFunctionEntry {
        name: "sha256",
        category: "Cryptographic",
    },
    EvalFunctionEntry {
        name: "sha512",
        category: "Cryptographic",
    },
    // --- Date and Time ---
    EvalFunctionEntry {
        name: "now",
        category: "DateTime",
    },
    EvalFunctionEntry {
        name: "relative_time",
        category: "DateTime",
    },
    EvalFunctionEntry {
        name: "strftime",
        category: "DateTime",
    },
    EvalFunctionEntry {
        name: "strptime",
        category: "DateTime",
    },
    EvalFunctionEntry {
        name: "time",
        category: "DateTime",
    },
    // --- Informational ---
    EvalFunctionEntry {
        name: "isarray",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "isbool",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "isdouble",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "isint",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "ismv",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "isnotnull",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "isnull",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "isnum",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "isobject",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "isstr",
        category: "Informational",
    },
    EvalFunctionEntry {
        name: "typeof",
        category: "Informational",
    },
    // --- JSON ---
    EvalFunctionEntry {
        name: "json",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_object",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_append",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_array",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_array_to_mv",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_delete",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_entries",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_extend",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_extract",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_extract_exact",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_has_key_exact",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_keys",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_set",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_set_exact",
        category: "JSON",
    },
    EvalFunctionEntry {
        name: "json_valid",
        category: "JSON",
    },
    // --- Mathematical ---
    EvalFunctionEntry {
        name: "abs",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "ceiling",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "ceil",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "exact",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "exp",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "floor",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "ln",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "log",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "pi",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "pow",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "round",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "sigfig",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "sqrt",
        category: "Mathematical",
    },
    EvalFunctionEntry {
        name: "sum",
        category: "Mathematical",
    },
    // --- Multivalue ---
    EvalFunctionEntry {
        name: "commands",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvappend",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvcount",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvdedup",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvfilter",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvfind",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvindex",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvjoin",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvmap",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvrange",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvreverse",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvsort",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mvzip",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "mv_to_json_array",
        category: "Multivalue",
    },
    EvalFunctionEntry {
        name: "split",
        category: "Multivalue",
    },
    // --- Statistical ---
    EvalFunctionEntry {
        name: "avg",
        category: "Statistical",
    },
    EvalFunctionEntry {
        name: "max",
        category: "Statistical",
    },
    EvalFunctionEntry {
        name: "min",
        category: "Statistical",
    },
    EvalFunctionEntry {
        name: "random",
        category: "Statistical",
    },
    // --- Text ---
    EvalFunctionEntry {
        name: "len",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "lower",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "ltrim",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "replace",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "rtrim",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "spath",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "substr",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "trim",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "upper",
        category: "Text",
    },
    EvalFunctionEntry {
        name: "urldecode",
        category: "Text",
    },
    // --- Trigonometric / Hyperbolic ---
    EvalFunctionEntry {
        name: "acos",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "acosh",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "asin",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "asinh",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "atan",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "atan2",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "atanh",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "cos",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "cosh",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "hypot",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "sin",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "sinh",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "tan",
        category: "Trigonometric",
    },
    EvalFunctionEntry {
        name: "tanh",
        category: "Trigonometric",
    },
];

/// eval 関数名が組み込み関数として認識されるか判定します。
pub fn is_known_eval_function(name: &str) -> bool {
    KNOWN_EVAL_FUNCTION_ENTRIES
        .iter()
        .any(|f| f.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_known_eval_function() {
        assert!(is_known_eval_function("if"));
        assert!(is_known_eval_function("coalesce"));
        assert!(is_known_eval_function("mvappend"));
    }

    #[test]
    fn test_known_eval_function_case_insensitive() {
        assert!(is_known_eval_function("IF"));
        assert!(is_known_eval_function("Coalesce"));
    }

    #[test]
    fn test_unknown_eval_function() {
        assert!(!is_known_eval_function("foobar"));
    }

    #[test]
    fn test_new_functions_from_v102() {
        assert!(is_known_eval_function("bit_and"));
        assert!(is_known_eval_function("bit_shift_left"));
        assert!(is_known_eval_function("ipmask"));
        assert!(is_known_eval_function("toarray"));
        assert!(is_known_eval_function("tobool"));
        assert!(is_known_eval_function("isarray"));
        assert!(is_known_eval_function("ismv"));
        assert!(is_known_eval_function("json"));
        assert!(is_known_eval_function("json_array_to_mv"));
        assert!(is_known_eval_function("json_delete"));
        assert!(is_known_eval_function("mvreverse"));
        assert!(is_known_eval_function("spath"));
    }

    #[test]
    fn test_no_duplicate_entries() {
        let mut seen = HashSet::new();
        for entry in KNOWN_EVAL_FUNCTION_ENTRIES {
            assert!(
                seen.insert(entry.name),
                "duplicate eval function entry: {}",
                entry.name
            );
        }
    }
}
