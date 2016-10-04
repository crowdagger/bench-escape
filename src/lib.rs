#![feature(test)]

extern crate test;
extern crate regex;
#[macro_use] extern crate lazy_static;

use std::io;
use std::io::Write;
use test::Bencher;

mod escape_naive;
mod escape_cow;
mod escape_regex;

const NO_HTML_SHORT: &'static str = "A paragraph without HTML characters that need to be escaped.";
const NO_HTML_LONG: &'static str = "Another paragraph without characters that need to be escaped.
This paragraph is a bit longer, as sometimes there can be large paragraphs that don't contain
any special characters, e.g. in novels or whatever.";
const HTML_SHORT: &'static str = "fn foo(u: &u32) -> &u32 { u }";
const HTML_LONG: &'static str = "A somewhat longer paragraph containing a character that needs to be escaped, because e.g. the author mentions the movie 'Fast&Furious' in it.";

#[bench]
fn bench_naive_no_html(b: &mut Bencher) {
    use escape_naive::naive;
    b.iter(|| {
        naive(NO_HTML_SHORT);
        naive(NO_HTML_LONG);
    })
}

#[bench]
fn bench_naive_html(b: &mut Bencher) {
    use escape_naive::naive;
    b.iter(|| {
        naive(HTML_SHORT);
        naive(HTML_LONG);
    })
}

#[bench]
fn bench_naive_all(b: &mut Bencher) {
    use escape_naive::naive;
    b.iter(|| {
        naive(NO_HTML_SHORT);
        naive(NO_HTML_LONG);
        naive(HTML_SHORT);
        naive(HTML_LONG);
    })
}

#[bench]
fn bench_naive_capacity_no_html(b: &mut Bencher) {
    use escape_naive::capacity;
    b.iter(|| {
        capacity(NO_HTML_SHORT);
        capacity(NO_HTML_LONG);
    })
}

#[bench]
fn bench_naive_capacity_html(b: &mut Bencher) {
    use escape_naive::capacity;
    b.iter(|| {
        capacity(HTML_SHORT);
        capacity(HTML_LONG);
    })
}

#[bench]
fn bench_naive_capacity_all(b: &mut Bencher) {
    use escape_naive::capacity;
    b.iter(|| {
        capacity(NO_HTML_SHORT);
        capacity(NO_HTML_LONG);
        capacity(HTML_SHORT);
        capacity(HTML_LONG);
    })
}


#[bench]
fn bench_naive_cow_no_html(b: &mut Bencher) {
    use escape_cow::naive;
    b.iter(|| {
        naive(NO_HTML_SHORT);
        naive(NO_HTML_LONG);
    })
}

#[bench]
fn bench_naive_cow_html(b: &mut Bencher) {
    use escape_cow::naive;
    b.iter(|| {
        naive(HTML_SHORT);
        naive(HTML_LONG);
    })
}

#[bench]
fn bench_naive_cow_all(b: &mut Bencher) {
    use escape_cow::naive;
    b.iter(|| {
        naive(NO_HTML_SHORT);
        naive(NO_HTML_LONG);
        naive(HTML_SHORT);
        naive(HTML_LONG);
    })
}


#[bench]
fn bench_cow_find_no_html(b: &mut Bencher) {
    use escape_cow::find;
    b.iter(|| {
        find(NO_HTML_SHORT);
        find(NO_HTML_LONG);
    })
}

#[bench]
fn bench_cow_find_html(b: &mut Bencher) {
    use escape_cow::find;
    b.iter(|| {
        find(HTML_SHORT);
        find(HTML_LONG);
    })
}

#[bench]
fn bench_cow_find_all(b: &mut Bencher) {
    use escape_cow::find;
    b.iter(|| {
        find(NO_HTML_SHORT);
        find(NO_HTML_LONG);
        find(HTML_SHORT);
        find(HTML_LONG);
    })
}


#[bench]
fn bench_cow_find_recursive_no_html(b: &mut Bencher) {
    use escape_cow::find_recursive;
    b.iter(|| {
        find_recursive(NO_HTML_SHORT);
        find_recursive(NO_HTML_LONG);
    })
}

#[bench]
fn bench_cow_find_recursive_html(b: &mut Bencher) {
    use escape_cow::find_recursive;
    b.iter(|| {
        find_recursive(HTML_SHORT);
        find_recursive(HTML_LONG);
    })
}

#[bench]
fn bench_cow_find_recursive_all(b: &mut Bencher) {
    use escape_cow::find_recursive;
    b.iter(|| {
        find_recursive(NO_HTML_SHORT);
        find_recursive(NO_HTML_LONG);
        find_recursive(HTML_SHORT);
        find_recursive(HTML_LONG);
    })
}


#[bench]
fn bench_cow_find_chars_no_html(b: &mut Bencher) {
    use escape_cow::find_chars;
    b.iter(|| {
        find_chars(NO_HTML_SHORT);
        find_chars(NO_HTML_LONG);
    })
}

#[bench]
fn bench_cow_find_chars_html(b: &mut Bencher) {
    use escape_cow::find_chars;
    b.iter(|| {
        find_chars(HTML_SHORT);
        find_chars(HTML_LONG);
    })
}

#[bench]
fn bench_cow_find_chars_all(b: &mut Bencher) {
    use escape_cow::find_chars;
    b.iter(|| {
        find_chars(NO_HTML_SHORT);
        find_chars(NO_HTML_LONG);
        find_chars(HTML_SHORT);
        find_chars(HTML_LONG);
    })
}


#[bench]
fn bench_regex_no_html(b: &mut Bencher) {
    use escape_regex::find;
    b.iter(|| {
        find(NO_HTML_SHORT);
        find(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_find_html(b: &mut Bencher) {
    use escape_regex::find;
    b.iter(|| {
        find(HTML_SHORT);
        find(HTML_LONG);
    })
}

#[bench]
fn bench_regex_find_all(b: &mut Bencher) {
    use escape_regex::find;
    b.iter(|| {
        find(NO_HTML_SHORT);
        find(NO_HTML_LONG);
        find(HTML_SHORT);
        find(HTML_LONG);
    })
}
