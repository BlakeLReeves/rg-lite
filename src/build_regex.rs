use regex::Regex;

pub fn build_regex(pattern: &str, ignore_case: &bool) -> Regex {
    let regex_pattern = if *ignore_case {
        format!("(?i){}", regex::escape(pattern))
    } else {
        regex::escape(pattern)
    };

    Regex::new(&regex_pattern).unwrap()
}
