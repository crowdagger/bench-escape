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
const NO_HTML_LONG: &'static str = "Another paragraph without characters that need to be escaped. This paragraph is a bit longer, as sometimes there can be large paragraphs that don't any special characters, e.g., in novels or whatever.";
const HTML_SHORT: &'static str = "Here->An <<example>> of rust codefn foo(u: &u32) -> &u32 {u}";
const HTML_LONG: &'static str = "A somewhat longer paragraph containing a character that needs to be escaped, because e.g. the author mentions the movie Fast&Furious in it. This paragraph is also quite long to match the non-html one.";

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
fn bench_cow_find_morecap_no_html(b: &mut Bencher) {
    use escape_cow::find_morecap;
    b.iter(|| {
        find_morecap(NO_HTML_SHORT);
        find_morecap(NO_HTML_LONG);
    })
}

#[bench]
fn bench_cow_find_morecap_html(b: &mut Bencher) {
    use escape_cow::find_morecap;
    b.iter(|| {
        find_morecap(HTML_SHORT);
        find_morecap(HTML_LONG);
    })
}

#[bench]
fn bench_cow_find_morecap_all(b: &mut Bencher) {
    use escape_cow::find_morecap;
    b.iter(|| {
        find_morecap(NO_HTML_SHORT);
        find_morecap(NO_HTML_LONG);
        find_morecap(HTML_SHORT);
        find_morecap(HTML_LONG);
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
fn bench_regex_html(b: &mut Bencher) {
    use escape_regex::find;
    b.iter(|| {
        find(HTML_SHORT);
        find(HTML_LONG);
    })
}

#[bench]
fn bench_regex_all(b: &mut Bencher) {
    use escape_regex::find;
    b.iter(|| {
        find(NO_HTML_SHORT);
        find(NO_HTML_LONG);
        find(HTML_SHORT);
        find(HTML_LONG);
    })
}


#[bench]
fn bench_regex_capacity_no_html(b: &mut Bencher) {
    use escape_regex::find_capacity;
    b.iter(|| {
        find_capacity(NO_HTML_SHORT);
        find_capacity(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_capacity_html(b: &mut Bencher) {
    use escape_regex::find_capacity;
    b.iter(|| {
        find_capacity(HTML_SHORT);
        find_capacity(HTML_LONG);
    })
}

#[bench]
fn bench_regex_capacity_all(b: &mut Bencher) {
    use escape_regex::find_capacity;
    b.iter(|| {
        find_capacity(NO_HTML_SHORT);
        find_capacity(NO_HTML_LONG);
        find_capacity(HTML_SHORT);
        find_capacity(HTML_LONG);
    })
}

#[bench]
fn bench_regex_iter_no_html(b: &mut Bencher) {
    use escape_regex::find_iter;
    b.iter(|| {
        find_iter(NO_HTML_SHORT);
        find_iter(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_iter_html(b: &mut Bencher) {
    use escape_regex::find_iter;
    b.iter(|| {
        find_iter(HTML_SHORT);
        find_iter(HTML_LONG);
    })
}

#[bench]
fn bench_regex_iter_all(b: &mut Bencher) {
    use escape_regex::find_iter;
    b.iter(|| {
        find_iter(NO_HTML_SHORT);
        find_iter(NO_HTML_LONG);
        find_iter(HTML_SHORT);
        find_iter(HTML_LONG);
    })
}


#[bench]
fn bench_regex_iter_morecap_no_html(b: &mut Bencher) {
    use escape_regex::find_iter_morecap;
    b.iter(|| {
        find_iter_morecap(NO_HTML_SHORT);
        find_iter_morecap(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_iter_morecap_html(b: &mut Bencher) {
    use escape_regex::find_iter_morecap;
    b.iter(|| {
        find_iter_morecap(HTML_SHORT);
        find_iter_morecap(HTML_LONG);
    })
}

#[bench]
fn bench_regex_iter_morecap_all(b: &mut Bencher) {
    use escape_regex::find_iter_morecap;
    b.iter(|| {
        find_iter_morecap(NO_HTML_SHORT);
        find_iter_morecap(NO_HTML_LONG);
        find_iter_morecap(HTML_SHORT);
        find_iter_morecap(HTML_LONG);
    })
}


#[bench]
fn bench_regex_no_static_no_html(b: &mut Bencher) {
    use escape_regex::find_no_static;
    b.iter(|| {
        find_no_static(NO_HTML_SHORT);
        find_no_static(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_no_static_html(b: &mut Bencher) {
    use escape_regex::find_no_static;
    b.iter(|| {
        find_no_static(HTML_SHORT);
        find_no_static(HTML_LONG);
    })
}

#[bench]
fn bench_regex_no_static_all(b: &mut Bencher) {
    use escape_regex::find_no_static;
    b.iter(|| {
        find_no_static(NO_HTML_SHORT);
        find_no_static(NO_HTML_LONG);
        find_no_static(HTML_SHORT);
        find_no_static(HTML_LONG);
    })
}


#[bench]
fn bench_regex_no_reserve_no_html(b: &mut Bencher) {
    use escape_regex::find_no_reserve;
    b.iter(|| {
        find_no_reserve(NO_HTML_SHORT);
        find_no_reserve(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_no_reserve_html(b: &mut Bencher) {
    use escape_regex::find_no_reserve;
    b.iter(|| {
        find_no_reserve(HTML_SHORT);
        find_no_reserve(HTML_LONG);
    })
}

#[bench]
fn bench_regex_no_reserve_all(b: &mut Bencher) {
    use escape_regex::find_no_reserve;
    b.iter(|| {
        find_no_reserve(NO_HTML_SHORT);
        find_no_reserve(NO_HTML_LONG);
        find_no_reserve(HTML_SHORT);
        find_no_reserve(HTML_LONG);
    })
}


#[bench]
fn bench_regex_replace_no_html(b: &mut Bencher) {
    use escape_regex::replace;
    b.iter(|| {
        replace(NO_HTML_SHORT);
        replace(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_replace_html(b: &mut Bencher) {
    use escape_regex::replace;
    b.iter(|| {
        replace(HTML_SHORT);
        replace(HTML_LONG);
    })
}

#[bench]
fn bench_regex_replace_all(b: &mut Bencher) {
    use escape_regex::replace;
    b.iter(|| {
        replace(NO_HTML_SHORT);
        replace(NO_HTML_LONG);
        replace(HTML_SHORT);
        replace(HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_no_html(b: &mut Bencher) {
    use escape_regex::find_u8;
    b.iter(|| {
        find_u8(NO_HTML_SHORT);
        find_u8(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_html(b: &mut Bencher) {
    use escape_regex::find_u8;
    b.iter(|| {
        find_u8(HTML_SHORT);
        find_u8(HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_all(b: &mut Bencher) {
    use escape_regex::find_u8;
    b.iter(|| {
        find_u8(NO_HTML_SHORT);
        find_u8(NO_HTML_LONG);
        find_u8(HTML_SHORT);
        find_u8(HTML_LONG);
    })
}


#[bench]
fn bench_regex_u8_morecap_no_html(b: &mut Bencher) {
    use escape_regex::find_u8_morecap;
    b.iter(|| {
        find_u8_morecap(NO_HTML_SHORT);
        find_u8_morecap(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_morecap_html(b: &mut Bencher) {
    use escape_regex::find_u8_morecap;
    b.iter(|| {
        find_u8_morecap(HTML_SHORT);
        find_u8_morecap(HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_morecap_all(b: &mut Bencher) {
    use escape_regex::find_u8_morecap;
    b.iter(|| {
        find_u8_morecap(NO_HTML_SHORT);
        find_u8_morecap(NO_HTML_LONG);
        find_u8_morecap(HTML_SHORT);
        find_u8_morecap(HTML_LONG);
    })
}


#[bench]
fn bench_regex_u8_iter_no_html(b: &mut Bencher) {
    use escape_regex::find_u8_iter;
    b.iter(|| {
        find_u8_iter(NO_HTML_SHORT);
        find_u8_iter(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_iter_html(b: &mut Bencher) {
    use escape_regex::find_u8_iter;
    b.iter(|| {
        find_u8_iter(HTML_SHORT);
        find_u8_iter(HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_iter_all(b: &mut Bencher) {
    use escape_regex::find_u8_iter;
    b.iter(|| {
        find_u8_iter(NO_HTML_SHORT);
        find_u8_iter(NO_HTML_LONG);
        find_u8_iter(HTML_SHORT);
        find_u8_iter(HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_unsafe_no_html(b: &mut Bencher) {
    use escape_regex::find_u8_unsafe;
    b.iter(|| {
        find_u8_unsafe(NO_HTML_SHORT);
        find_u8_unsafe(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_unsafe_html(b: &mut Bencher) {
    use escape_regex::find_u8_unsafe;
    b.iter(|| {
        find_u8_unsafe(HTML_SHORT);
        find_u8_unsafe(HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_unsafe_all(b: &mut Bencher) {
    use escape_regex::find_u8_unsafe;
    b.iter(|| {
        find_u8_unsafe(NO_HTML_SHORT);
        find_u8_unsafe(NO_HTML_LONG);
        find_u8_unsafe(HTML_SHORT);
        find_u8_unsafe(HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_capacity_no_html(b: &mut Bencher) {
    use escape_regex::find_u8_capacity;
    b.iter(|| {
        find_u8_capacity(NO_HTML_SHORT);
        find_u8_capacity(NO_HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_capacity_html(b: &mut Bencher) {
    use escape_regex::find_u8_capacity;
    b.iter(|| {
        find_u8_capacity(HTML_SHORT);
        find_u8_capacity(HTML_LONG);
    })
}

#[bench]
fn bench_regex_u8_capacity_all(b: &mut Bencher) {
    use escape_regex::find_u8_capacity;
    b.iter(|| {
        find_u8_capacity(NO_HTML_SHORT);
        find_u8_capacity(NO_HTML_LONG);
        find_u8_capacity(HTML_SHORT);
        find_u8_capacity(HTML_LONG);
    })
}
