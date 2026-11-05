//! vietnativekey: user-reported cases (elongated slang, English, start of line).
//! Expected outputs confirmed by the user.

mod common;
use common::{telex, telex_auto_restore};

const SLANG: &[(&str, &str)] = &[
    ("xisuuuuuu ", "xíuuuuuu "),
    ("ddoiwjiiiiiii ", "đợiiiiiiii "),
    ("ddiiiiiii ", "điiiiiii "),
    // a/e/o: first repeat makes the circumflex (nấ), next one undoes it (Telex aaa→aa)
    ("nasaaaaaa ", "náaaaaa "),
];

#[test]
fn slang_plain_telex() {
    telex(SLANG);
}

#[test]
fn slang_with_english_auto_restore() {
    telex_auto_restore(SLANG);
}

#[test]
fn english_stays_english() {
    telex_auto_restore(&[
        ("code ", "code "),
        ("review ", "review "),
        ("deadline ", "deadline "),
        ("request ", "request "),
    ]);
}

#[test]
fn basic_telex() {
    telex_auto_restore(&[("dd", "đ"), ("truowfng ", "trường "), ("aw", "ă")]);
}
