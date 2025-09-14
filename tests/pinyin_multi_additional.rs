// Additional integration tests for PinyinMulti and related iterators.
 
// -----------------------------------------------------------------------------
// Additional tests generated: using Rust built-in test framework (cargo test).
// These tests validate PinyinMulti, its iterator, and ToPinyinMulti for char & str.
// -----------------------------------------------------------------------------

#[cfg(test)]
mod more_tests_for_pinyin_multi {
    // Keep imports scoped to avoid conflicts with existing mod tests in this file (if any).
    use crate::ToPinyinMulti;
    #[cfg(feature = "with_tone")]
    use crate::Pinyin;

    // Helper to unwrap a multi for a given char, panicking with context if missing.
    fn multi_of(c: char) -> crate::PinyinMulti {
        c.to_pinyin_multi().unwrap_or_else(|| panic\!("No pinyin for char: {c}"))
    }

    // ---------- Non-tone tests (compile regardless of with_tone) ----------

    #[test]
    fn char_without_pinyin_returns_none() {
        // Use an emoji to ensure there is no mapping.
        let none = '🙂'.to_pinyin_multi();
        assert\!(none.is_none(), "Expected None for emoji without pinyin");
    }

    #[test]
    fn char_with_multiple_pronunciations_count_is_correct() {
        // Based on doc examples in this crate, '还' has three pronunciations.
        let m = multi_of('还');
        assert_eq\!(m.count(), 3, "Expected three pronunciations for '还'");
    }

    #[test]
    fn get_opt_boundaries_for_multi_pronunciation_char() {
        let m = multi_of('还');
        // Should have entries at 0,1,2 and None at 3
        assert\!(m.get_opt(0).is_some());
        assert\!(m.get_opt(1).is_some());
        assert\!(m.get_opt(2).is_some());
        assert\!(m.get_opt(3).is_none());
    }

    #[test]
    fn iterator_yields_exact_number_then_none_repeatedly() {
        let mut it = multi_of('还').into_iter();
        assert\!(it.next().is_some());
        assert\!(it.next().is_some());
        assert\!(it.next().is_some());
        assert\!(it.next().is_none());
        // Subsequent calls should remain None
        assert\!(it.next().is_none());
        assert\!(it.next().is_none());
    }

    #[test]
    fn into_iter_can_be_reused_because_pinyinmulti_is_copy() {
        let m = multi_of('子'); // known 2 pronunciations from existing tests
        let v1 = m.into_iter().collect::<Vec<_>>();
        let v2 = m.into_iter().collect::<Vec<_>>();
        assert_eq\!(v1.len(), 2);
        assert_eq\!(v2.len(), 2);
        // We do not assert element equality here to avoid requiring tone formatting.
    }

    #[test]
    fn get_opt_with_extremely_large_index_returns_none() {
        let m = multi_of('子');
        assert\!(m.get_opt(usize::MAX).is_none());
    }

    #[test]
    fn str_iterator_emits_none_for_non_han_and_some_for_han() {
        let mut it = "🙂还A".to_pinyin_multi();
        let first = it.next(); // emoji
        let second = it.next(); // '还'
        let third = it.next(); // 'A'

        assert\!(first.is_some());
        assert\!(first.unwrap().is_none(), "Emoji should map to None");

        assert\!(second.is_some());
        assert\!(second.unwrap().is_some(), "'还' should map to Some(PinyinMulti)");

        assert\!(third.is_some());
        // ASCII 'A' should have no mapping in this dataset
        assert\!(third.unwrap().is_none(), "'A' should map to None");
        assert\!(it.next().is_none());
    }

    #[test]
    #[should_panic]
    fn get_panics_when_index_equals_count() {
        // '还' has 3 pronunciations; index 3 should panic (0..=2 are valid)
        multi_of('还').get(3);
    }

    // ---------- Tone-gated tests (only compile when feature = "with_tone") ----------

    #[test]
    #[cfg(feature = "with_tone")]
    fn char_hai_has_expected_tone_strings_and_order() {
        // From crate docs and existing tests: "hái", "huán", "fú"
        let tones = multi_of('还')
            .into_iter()
            .map(Pinyin::with_tone)
            .collect::<Vec<_>>();
        assert_eq\!(tones, vec\!["hái", "huán", "fú"]);
    }

    #[test]
    #[cfg(feature = "with_tone")]
    fn copy_semantics_allow_replaying_iteration_to_same_results() {
        let m = multi_of('子');
        let a = m.into_iter().map(Pinyin::with_tone).collect::<Vec<_>>();
        let b = m.into_iter().map(Pinyin::with_tone).collect::<Vec<_>>();
        assert_eq\!(a, b);
        assert_eq\!(a, vec\!["zi", "zǐ"]);
    }

    #[test]
    #[cfg(feature = "with_tone")]
    fn str_iterator_with_filtering_none_and_flattening() {
        // Mix None and Some, then flatten and assert flattened tone strings
        let flattened = "🙂还A"
            .to_pinyin_multi()
            .filter_map(|opt| opt) // drop Nones (chars without pinyin)
            .flat_map(|m| m.into_iter().map(Pinyin::with_tone))
            .collect::<Vec<_>>();
        assert_eq\!(flattened, vec\!["hái", "huán", "fú"]);
    }
}