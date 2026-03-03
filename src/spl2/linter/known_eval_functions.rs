/// SPL2 eval 関数エントリです。
pub struct Spl2EvalFunctionEntry {
    pub name: &'static str,
    pub category: &'static str,
}

/// SPL2 の eval 関数のカテゴリ付きリストです。
pub static KNOWN_SPL2_EVAL_FUNCTION_ENTRIES: &[Spl2EvalFunctionEntry] = &[
    // --- Bitwise ---
    Spl2EvalFunctionEntry {
        name: "bit_and",
        category: "Bitwise",
    },
    Spl2EvalFunctionEntry {
        name: "bit_or",
        category: "Bitwise",
    },
    Spl2EvalFunctionEntry {
        name: "bit_not",
        category: "Bitwise",
    },
    Spl2EvalFunctionEntry {
        name: "bit_xor",
        category: "Bitwise",
    },
    Spl2EvalFunctionEntry {
        name: "bit_shift_left",
        category: "Bitwise",
    },
    Spl2EvalFunctionEntry {
        name: "bit_shift_right",
        category: "Bitwise",
    },
    // --- Comparison / Conditional ---
    Spl2EvalFunctionEntry {
        name: "case",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "cidrmatch",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "coalesce",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "false",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "if",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "in",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "like",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "lookup",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "match",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "null",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "nullif",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "searchmatch",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "true",
        category: "Conditional",
    },
    Spl2EvalFunctionEntry {
        name: "validate",
        category: "Conditional",
    },
    // --- Conversion ---
    Spl2EvalFunctionEntry {
        name: "ipmask",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "printf",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "toarray",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "tobool",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "todouble",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "toint",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "tojson",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "tomv",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "tonumber",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "toobject",
        category: "Conversion",
    },
    Spl2EvalFunctionEntry {
        name: "tostring",
        category: "Conversion",
    },
    // --- Cryptographic ---
    Spl2EvalFunctionEntry {
        name: "md5",
        category: "Cryptographic",
    },
    Spl2EvalFunctionEntry {
        name: "sha1",
        category: "Cryptographic",
    },
    Spl2EvalFunctionEntry {
        name: "sha256",
        category: "Cryptographic",
    },
    Spl2EvalFunctionEntry {
        name: "sha512",
        category: "Cryptographic",
    },
    // --- Date and Time ---
    Spl2EvalFunctionEntry {
        name: "now",
        category: "DateTime",
    },
    Spl2EvalFunctionEntry {
        name: "relative_time",
        category: "DateTime",
    },
    Spl2EvalFunctionEntry {
        name: "strftime",
        category: "DateTime",
    },
    Spl2EvalFunctionEntry {
        name: "strptime",
        category: "DateTime",
    },
    Spl2EvalFunctionEntry {
        name: "time",
        category: "DateTime",
    },
    // --- Informational ---
    Spl2EvalFunctionEntry {
        name: "isarray",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "isbool",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "isdouble",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "isint",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "ismv",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "isnotnull",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "isnull",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "isnum",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "isobject",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "isstr",
        category: "Informational",
    },
    Spl2EvalFunctionEntry {
        name: "typeof",
        category: "Informational",
    },
    // --- JSON ---
    Spl2EvalFunctionEntry {
        name: "json",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_object",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_append",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_array",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_array_to_mv",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_delete",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_entries",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_extend",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_extract",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_extract_exact",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_has_key_exact",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_keys",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_set",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_set_exact",
        category: "JSON",
    },
    Spl2EvalFunctionEntry {
        name: "json_valid",
        category: "JSON",
    },
    // --- Mathematical ---
    Spl2EvalFunctionEntry {
        name: "abs",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "ceiling",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "ceil",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "exact",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "exp",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "floor",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "ln",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "log",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "pi",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "pow",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "round",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "sigfig",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "sqrt",
        category: "Mathematical",
    },
    Spl2EvalFunctionEntry {
        name: "sum",
        category: "Mathematical",
    },
    // --- Multivalue ---
    Spl2EvalFunctionEntry {
        name: "commands",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvappend",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvcount",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvdedup",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvfilter",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvfind",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvindex",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvjoin",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvmap",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvrange",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvreverse",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvsort",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mvzip",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "mv_to_json_array",
        category: "Multivalue",
    },
    Spl2EvalFunctionEntry {
        name: "split",
        category: "Multivalue",
    },
    // --- Statistical ---
    Spl2EvalFunctionEntry {
        name: "avg",
        category: "Statistical",
    },
    Spl2EvalFunctionEntry {
        name: "max",
        category: "Statistical",
    },
    Spl2EvalFunctionEntry {
        name: "min",
        category: "Statistical",
    },
    Spl2EvalFunctionEntry {
        name: "random",
        category: "Statistical",
    },
    // --- Text ---
    Spl2EvalFunctionEntry {
        name: "len",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "lower",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "ltrim",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "replace",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "rtrim",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "spath",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "substr",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "trim",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "upper",
        category: "Text",
    },
    Spl2EvalFunctionEntry {
        name: "urldecode",
        category: "Text",
    },
    // --- Trigonometric / Hyperbolic ---
    Spl2EvalFunctionEntry {
        name: "acos",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "acosh",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "asin",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "asinh",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "atan",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "atan2",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "atanh",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "cos",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "cosh",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "hypot",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "sin",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "sinh",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "tan",
        category: "Trigonometric",
    },
    Spl2EvalFunctionEntry {
        name: "tanh",
        category: "Trigonometric",
    },
    // --- Higher-order functions (SPL2 新規) ---
    Spl2EvalFunctionEntry {
        name: "all",
        category: "HigherOrder",
    },
    Spl2EvalFunctionEntry {
        name: "any",
        category: "HigherOrder",
    },
    Spl2EvalFunctionEntry {
        name: "filter",
        category: "HigherOrder",
    },
    Spl2EvalFunctionEntry {
        name: "map",
        category: "HigherOrder",
    },
    Spl2EvalFunctionEntry {
        name: "reduce",
        category: "HigherOrder",
    },
    // --- Object functions (SPL2 新規) ---
    Spl2EvalFunctionEntry {
        name: "object_to_array",
        category: "Object",
    },
    // --- Conversion (SPL2 追加) ---
    Spl2EvalFunctionEntry {
        name: "to_ocsf",
        category: "Conversion",
    },
];

/// SPL2 の eval 関数名が組み込み関数として認識されるか判定します。
pub fn is_known_spl2_eval_function(name: &str) -> bool {
    KNOWN_SPL2_EVAL_FUNCTION_ENTRIES
        .iter()
        .any(|f| f.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_known_spl2_eval_function() {
        assert!(is_known_spl2_eval_function("if"));
        assert!(is_known_spl2_eval_function("coalesce"));
        assert!(is_known_spl2_eval_function("mvappend"));
    }

    #[test]
    fn test_spl2_specific_functions() {
        assert!(is_known_spl2_eval_function("all"));
        assert!(is_known_spl2_eval_function("any"));
        assert!(is_known_spl2_eval_function("filter"));
        assert!(is_known_spl2_eval_function("map"));
        assert!(is_known_spl2_eval_function("reduce"));
        assert!(is_known_spl2_eval_function("object_to_array"));
        assert!(is_known_spl2_eval_function("to_ocsf"));
        assert!(is_known_spl2_eval_function("tojson"));
    }

    #[test]
    fn test_unknown_spl2_eval_function() {
        assert!(!is_known_spl2_eval_function("foobar"));
    }

    #[test]
    fn test_no_duplicate_entries() {
        let mut seen = HashSet::new();
        for entry in KNOWN_SPL2_EVAL_FUNCTION_ENTRIES {
            assert!(
                seen.insert(entry.name),
                "duplicate SPL2 eval function entry: {}",
                entry.name
            );
        }
    }
}
