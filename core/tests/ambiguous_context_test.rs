//! Words valid in both English and Vietnamese (is/í, or/ỏ, mix/mĩ): the line's context decides.

use gonhanh_core::engine::Engine;
use gonhanh_core::utils::type_word;

fn run(default_english: bool, input: &str) -> String {
    let mut e = Engine::new();
    e.set_english_auto_restore(true);
    e.set_ambiguous_default_english(default_english);
    type_word(&mut e, input)
}

#[test]
fn english_context_keeps_english() {
    // "code"/"user" are clearly English, so the ambiguous word after them stays English
    for default_english in [true, false] {
        assert_eq!(run(default_english, "user is it "), "user is it ", "default_english={default_english}");
        assert_eq!(run(default_english, "code or text "), "code or text ");
        assert_eq!(run(default_english, "text as code "), "text as code ");
    }
}

#[test]
fn vietnamese_context_keeps_vietnamese() {
    // "đi"/"tôi" carry Telex marks → Vietnamese context
    for default_english in [true, false] {
        assert_eq!(run(default_english, "ddi ddi ir "), "đi đi ỉ ", "default_english={default_english}");
        assert_eq!(run(default_english, "tooi ddi or "), "tôi đi ỏ ");
    }
}

#[test]
fn no_context_uses_default() {
    assert_eq!(run(true, "is "), "is ");
    assert_eq!(run(true, "or "), "or ");
    assert_eq!(run(true, "mix "), "mix ");
    assert_eq!(run(false, "is "), "í ");
    assert_eq!(run(false, "or "), "ỏ ");
}

#[test]
fn enter_resets_context() {
    // English line, Enter, then an ambiguous word at the start of a new line -> default again
    for (default_english, expected) in [(false, "í "), (true, "is ")] {
        let mut e = Engine::new();
        e.set_english_auto_restore(true);
        e.set_ambiguous_default_english(default_english);
        type_word(&mut e, "user code ");
        e.on_key(gonhanh_core::data::keys::RETURN, false, false);
        assert_eq!(type_word(&mut e, "is "), expected, "default_english={default_english}");
    }
}

#[test]
fn user_slang_unaffected() {
    for default_english in [true, false] {
        assert_eq!(run(default_english, "ddoiwjiiiiiii "), "đợiiiiiiii ");
        assert_eq!(run(default_english, "daasu "), "dấu ");
    }
}
