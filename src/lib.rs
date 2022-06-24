use regex::Regex;
use std::collections::HashSet;

const FILE_REGEX: &str = r"([\w/\.-]+\.\w+(:\d+)?)";

pub fn find_files(lines: Vec<String>) -> HashSet<String> {
    // Use a HashSet to store unique results
    let mut results = HashSet::new();

    for line in lines {
        let re = Regex::new(FILE_REGEX).unwrap();
        for cap in re.captures_iter(&line) {
            results.insert(cap[0].to_string());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_single_file_with_no_path_in_a_line() {
        let input = lines("Consider example.json blah blah");

        assert_eq!(
            HashSet::from(["example.json".to_string()]),
            find_files(input)
        )
    }

    #[test]
    fn finds_single_file_with_no_path_and_multiple_extensions_in_a_line() {
        let input = lines("Consider example.test.js blah blah");

        assert_eq!(
            HashSet::from(["example.test.js".to_string()]),
            find_files(input)
        )
    }

    #[test]
    fn finds_single_file_with_a_path_with_a_preceeding_slash_in_a_line() {
        let input = lines("Consider /foo/example.json");

        assert_eq!(
            HashSet::from(["/foo/example.json".to_string()]),
            find_files(input)
        )
    }

    #[test]
    fn finds_single_file_with_a_path_with_no_preceeding_slash_in_a_line() {
        let input = lines("Consider foo/example.json blah blah");

        assert_eq!(
            HashSet::from(["foo/example.json".to_string()]),
            find_files(input)
        )
    }

    #[test]
    fn finds_multiple_files_in_a_line() {
        let input = lines("/foo/bar/baz.log cats and dogs, moo.txt.json, 1/2/3/4_-5.txt");

        assert_eq!(
            HashSet::from([
                "/foo/bar/baz.log".to_string(),
                "moo.txt.json".to_string(),
                "1/2/3/4_-5.txt".to_string()
            ]),
            find_files(input)
        )
    }

    #[test]
    fn finds_paths_with_line_numbers() {
        let input = lines("/foo/bar/baz.log:22 cats and dogs:, moo.txt.json:33, 1/2/3/4.txt:498");

        assert_eq!(
            HashSet::from([
                "/foo/bar/baz.log:22".to_string(),
                "moo.txt.json:33".to_string(),
                "1/2/3/4.txt:498".to_string()
            ]),
            find_files(input)
        )
    }

    #[test]
    fn finds_files_and_paths_across_lines_and_ignores_duplicates() {
        let input = lines(
            "\
            /foo/bar/baz.log:22 cats and dogs:, moo.txt.json:33, 1/2/3/4.txt:498
            /foo/bar/baz.log:22 more-goes-here.cat.dog
                    ",
        );

        assert_eq!(
            HashSet::from([
                "/foo/bar/baz.log:22".to_string(),
                "moo.txt.json:33".to_string(),
                "1/2/3/4.txt:498".to_string(),
                "more-goes-here.cat.dog".to_string()
            ]),
            find_files(input)
        )
    }

    fn lines(input: &str) -> Vec<String> {
        input.split_whitespace().map(|s| s.to_string()).collect()
    }
}
