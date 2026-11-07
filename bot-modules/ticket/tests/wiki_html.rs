use ticket::wiki::html_to_markdown;

#[test]
fn headings_and_emphasis_become_markdown() {
    let md = html_to_markdown(r#"<h1 id="a">Title</h1><p><strong>x</strong></p>"#)
        .expect("converts");

    assert!(md.contains("# Title"), "{md}");
    assert!(md.contains("**x**"), "{md}");
}

#[test]
fn code_blocks_are_fenced_with_their_language() {
    let md =
        html_to_markdown("<pre><code class=\"language-yaml\">a: 1\n</code></pre>")
            .expect("converts");

    assert!(md.contains("```yaml"), "{md}");
}

#[test]
fn a_tab_is_labelled() {
    let md = html_to_markdown(
        r#"<block-tabs><block-tab label="Docker"><p>run it</p></block-tab></block-tabs>"#,
    )
    .expect("converts");

    assert!(md.contains("**Docker**"), "{md}");
    assert!(md.contains("run it"), "{md}");
}

#[test]
fn scripts_are_dropped() {
    let md = html_to_markdown("<script>bad()</script><p>ok</p>").expect("converts");

    assert!(!md.contains("bad"), "{md}");
    assert!(md.contains("ok"), "{md}");
}
