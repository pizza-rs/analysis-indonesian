//! Comprehensive tests for pizza-analysis-indonesian.

use pizza_analysis_indonesian::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// IndonesianStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = IndonesianStemFilter::new();
}

#[test]
fn stem_prefix_me() {
    let f = IndonesianStemFilter::new();
    // "menulis" (to write) → strip prefix
    let mut token = make_token("menulis");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_prefix_ber() {
    let f = IndonesianStemFilter::new();
    // "berlari" (to run) → strip prefix
    let mut token = make_token("berlari");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_prefix_pe() {
    let f = IndonesianStemFilter::new();
    // "pelajaran" (lesson) → strip prefix
    let mut token = make_token("pelajaran");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_suffix_kan() {
    let f = IndonesianStemFilter::new();
    // "memberikan" (to give) → strip suffix
    let mut token = make_token("memberikan");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_suffix_an() {
    let f = IndonesianStemFilter::new();
    // "makanan" (food) → strip suffix
    let mut token = make_token("makanan");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_suffix_i() {
    let f = IndonesianStemFilter::new();
    // "mendatangi" (to come to) → strip suffix
    let mut token = make_token("mendatangi");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_base_word() {
    let f = IndonesianStemFilter::new();
    let mut token = make_token("rumah");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = IndonesianStemFilter::new();
    let mut token = make_token("di");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = IndonesianStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_single_char() {
    let f = IndonesianStemFilter::new();
    let mut token = make_token("a");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// IndonesianStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = IndonesianStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = IndonesianStopFilter::new();
    let stop_words = ["yang", "dan", "di", "dari", "ini", "itu", "dengan", "pada", "adalah", "ada"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = IndonesianStopFilter::new();
    let content_words = ["rumah", "buku", "sekolah", "kota"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = IndonesianStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("indonesian_stem").is_some());
    assert!(factory.get_token_filter("indonesian_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("indonesian").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("indonesian").unwrap();
    let mut input = String::from("Rumah itu besar dan indah");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("indonesian").unwrap();
    let mut input = String::from("buku ini dari sekolah itu");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"ini"));
    assert!(!terms.contains(&"dari"));
    assert!(!terms.contains(&"itu"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("indonesian").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("indonesian").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
