use std::cmp::Ordering;

///
///
/// Calculates a score between 0 and 100, and returns None if the score is zero or no matches are
/// found. This algorithm only returns a value if the pattern is a true substring of s.
///
/// # Arguments
///
/// * `pattern`: comparison string
/// * `s`: string slice
///
/// returns: Option<f32>
///
/// # Examples
///
/// ```
/// use fuzzy_search::fuzzy_match;
/// assert_eq!(fuzzy_match("ABC", "CB"), None);
/// assert_eq!(fuzzy_match("ABC", "ABC"), Some(100.0));
/// ```
pub fn fuzzy_match(pattern: &str, s: &str) -> Option<f32> {
    let s = s.to_uppercase();
    let pattern = pattern.to_uppercase();
    if let Some((ind, _sub)) = s.match_indices(&pattern).next() {
        println!("{}", ind);
        let score =
            ((pattern.len() as f32 / s.len() as f32) * 100.) - (ind as f32 / s.len() as f32);
        if score > 0. {
            return Some(score);
        }
    }
    None
}

///
///
/// FuzzySearchResult
///
/// The result of a fuzzy search, which includes the calculated score and the search pattern
///
pub struct FuzzySearchResult {
    score: f32,
    pattern: String,
}

///
///
/// Returns a vector of FuzzySearchResult sorted in descending order.
/// matches that score 0.0 are removed.
///
/// # Arguments
///
/// * `query`: the search query
/// * `domain`: the values to be searched
///
/// returns: Vec<FuzzySearchResult, Global>
///
pub fn fuzzy_search(query: &str, domain: &[&str]) -> Vec<FuzzySearchResult> {
    let mut vec: Vec<FuzzySearchResult> = domain
        .iter()
        .filter_map(|&searched| {
            let score = fuzzy_match(query, searched);
            score.map(|score| FuzzySearchResult {
                score,
                pattern: (*searched).to_string(),
            })
        })
        .collect();
    vec.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(Ordering::Equal)
    });
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match() {
        assert_eq!(fuzzy_match("ABC", "CB"), None);
        assert_eq!(fuzzy_match("ABC", "ABC"), Some(100.0));
        assert!(fuzzy_match("A", "ABC").is_some());
        assert!(fuzzy_match("A", "ABC").unwrap() < fuzzy_match("AB", "ABC").unwrap());
        assert!(fuzzy_match("AB", "ABC").unwrap() < fuzzy_match("ABC", "ABC").unwrap());
    }

    #[test]
    fn test_fuzzy_search() {
        let domain = ["ABC", "QZR", "APW"];
        let results = fuzzy_search("A", &domain);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].pattern, "ABC");
        assert_eq!(results[1].pattern, "APW");
    }
}
