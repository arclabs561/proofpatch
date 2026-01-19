use proofloops_core::ingest_research_json;

#[test]
fn ingest_dedupes_urls_and_keeps_titles() {
    let v = serde_json::json!({
        "results": [
            {"title":"A","url":"https://example.com/a","content":"x"},
            {"title":"A-dup","url":"https://example.com/a","content":"y"},
            {"title":"B","link":"https://example.com/b","snippet":"z"},
            {"title":"not-a-url","url":"file:///tmp/nope"},
            {"title":"schema-noise","url":"https://json-schema.org/draft-07/schema#"}
        ]
    });

    let notes = ingest_research_json(&v);
    assert_eq!(notes.deduped_urls, 2);
    assert!(notes.raw_urls >= 3);

    let mut urls: Vec<String> = notes.sources.iter().map(|s| s.url.clone()).collect();
    urls.sort();
    assert_eq!(urls, vec!["https://example.com/a", "https://example.com/b"]);

    let a = notes
        .sources
        .iter()
        .find(|s| s.url == "https://example.com/a")
        .expect("missing a");
    assert_eq!(a.title.as_deref(), Some("A"));
}

