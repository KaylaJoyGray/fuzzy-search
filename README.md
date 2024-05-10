# fuzzy_search

A simple fuzzy search implementation.

Included functions:

`fuzzy_match(pattern: &str, s: &str) -> Option<f32>`  
`fuzzy_search(query: &str, domain: &[&str]) -> Vec<FuzzySearchResult>`

Examples:

`assert_eq!(fuzzy_match("ABC", "CB"), None);`  
`assert_eq!(fuzzy_match("ABC", "ABC"), Some(100.0));`  
`assert!(fuzzy_match("A", "ABC").unwrap() < fuzzy_match("AB", "ABC").unwrap());`

`let domain = ["ABC", "QZR", "APW"];`  
`let results = fuzzy_search("A", &domain);`  
`assert_eq!(results.len(), 2);`  
`assert_eq!(results[0].pattern, "ABC");`  
`assert_eq!(results[1].pattern, "APW");`