#![feature(test)]

extern crate test;

use std::io;
use std::io::Write;
use test::Bencher;

mod escape_naive;

const NO_HTML_SHORT: &'static str = "A paragraph without HTML characters that need to be escaped.";
const NO_HTML_LONG: &'static str = "Another paragraph without characters that need to be escaped.
This paragraph is a bit longer, as sometimes there can be large paragraphs that don't contain
any special characters, e.g. in novels or whatever.";
const HTML_SHORT: &'static str = "fn foo(u: &u32) -> &u32 { u }";
const HTML_LONG: &'static str = "A somewhat longer paragraph containing a character that needs to be escaped, because e.g. the author mentions the movie 'Fast&Furious' in it.";

#[bench]
fn bench_naive_no_html(b: &mut Bencher) {
    use escape_naive::escape;
    b.iter(|| {
        escape(NO_HTML_SHORT);
        escape(NO_HTML_LONG);
    })
}

#[bench]
fn bench_naive_html(b: &mut Bencher) {
    use escape_naive::escape;
    b.iter(|| {
        escape(HTML_SHORT);
        escape(HTML_LONG);
    })
}

#[bench]
fn bench_naive_all(b: &mut Bencher) {
    use escape_naive::escape;
    b.iter(|| {
        escape(NO_HTML_SHORT);
        escape(NO_HTML_LONG);
        escape(HTML_SHORT);
        escape(HTML_LONG);
    })
}

    
    
