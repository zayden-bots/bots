use ticket::wiki::{normalize_path, path_hash};

#[test]
fn hashes_match_the_wiki_client() {
    let vectors = [
        ("sonarr", "f7f8cb91375f9"),
        ("home", "1867eaf483ceab"),
        ("", "1867eaf483ceab"),
        ("a", "1c2ba782c97901"),
        ("guides/setup", "16cf565ca1ca91"),
        ("/Guides/Setup/", "16cf565ca1ca91"),
        ("ünï", "1d8241d4f2fd3"),
        ("x/😀", "660ccc8e016cb"),
    ];

    for (input, expected) in vectors {
        assert_eq!(path_hash(&normalize_path(input)), expected, "{input:?}");
    }
}
