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
        assert_eq!(
            run(default_english, "user is it "),
            "user is it ",
            "default_english={default_english}"
        );
        assert_eq!(run(default_english, "code or text "), "code or text ");
        assert_eq!(run(default_english, "text as code "), "text as code ");
    }
}

#[test]
fn vietnamese_context_keeps_vietnamese() {
    // "đi"/"tôi" carry Telex marks → Vietnamese context
    for default_english in [true, false] {
        assert_eq!(
            run(default_english, "ddi ddi ir "),
            "đi đi ỉ ",
            "default_english={default_english}"
        );
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
        assert_eq!(
            type_word(&mut e, "is "),
            expected,
            "default_english={default_english}"
        );
    }
}

#[test]
fn two_ambiguous_words_in_a_row_become_english() {
    // "thí í" means nothing; both words are rewritten to English (Vietnamese default, no context)
    assert_eq!(run(false, "this is "), "this is ");
    assert_eq!(run(false, "this is it "), "this is it ");
    assert_eq!(run(false, "yes or no "), "yes or no ");
    assert_eq!(run(false, "as is or "), "as is or ");
    assert_eq!(run(true, "this is "), "this is ");
}

#[test]
fn lone_or_separated_ambiguous_words_are_unchanged() {
    // One ambiguous word alone keeps the default; a clear Vietnamese word in between breaks the run
    assert_eq!(run(false, "this "), "thí ");
    assert_eq!(run(false, "is "), "í ");
    assert_eq!(run(false, "ddi is "), "đi í ");
    assert_eq!(run(false, "ddi ir ir "), "đi ỉ ỉ ");
}

#[test]
fn backspace_forgets_the_run() {
    // After editing, the screen no longer matches what we remember: never rewrite blindly
    let mut e = Engine::new();
    e.set_english_auto_restore(true);
    type_word(&mut e, "this ");
    e.on_key(gonhanh_core::data::keys::DELETE, false, false);
    e.on_key(gonhanh_core::data::keys::DELETE, false, false);
    // whatever comes next must not try to rewrite the old "thí "
    let out = type_word(&mut e, "is ");
    assert_eq!(out, "í ");
}

#[test]
fn user_slang_unaffected() {
    for default_english in [true, false] {
        assert_eq!(run(default_english, "ddoiwjiiiiiii "), "đợiiiiiiii ");
        assert_eq!(run(default_english, "daasu "), "dấu ");
    }
}
