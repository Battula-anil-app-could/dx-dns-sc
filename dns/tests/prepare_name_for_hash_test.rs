// use dns::name_validation::SuffixType;
// use dharitri_sc::types::ManagedBuffer;
// use dharitri_sc_scenario::DebugApi;

// fn prepare_name_for_hash_and_classify(name_str: &str) -> (ManagedBuffer<DebugApi>, SuffixType) {
//     let mb = ManagedBuffer::<DebugApi>::from(name_str.as_bytes());
//     dns::name_validation::prepare_name_for_hash_and_classify(&mb)
// }

// fn check(name: &str, expected: &str, suffix_type: SuffixType) {
//     assert_eq!(
//         prepare_name_for_hash_and_classify(name),
//         (ManagedBuffer::from(expected.as_bytes()), suffix_type)
//     );
// }

// #[test]
// fn prepare_name_for_hash_test() {
//     let _ = DebugApi::dummy();

    // // .x is replaced with .dharitri
    // check("aaa.x", "aaa.dharitri", SuffixType::X);
    // check("aaaaaaaaaa.x", "aaaaaaaaaa.dharitri", SuffixType::X);
    // check("zzzzzzzzzz.x", "zzzzzzzzzz.dharitri", SuffixType::X);
    // check("0000000000.x", "0000000000.dharitri", SuffixType::X);
    // check("9999999999.x", "9999999999.dharitri", SuffixType::X);
    // check("coolname0001.x", "coolname0001.dharitri", SuffixType::X);

    // // .dharitri names are returned unchanged
    // check("aaa.dharitri", "aaa.dharitri", SuffixType::Dharitri);
    // check("aaaaaaaaaa.dharitri", "aaaaaaaaaa.dharitri", SuffixType::Dharitri);
    // check("zzzzzzzzzz.dharitri", "zzzzzzzzzz.dharitri", SuffixType::Dharitri);
    // check("0000000000.dharitri", "0000000000.dharitri", SuffixType::Dharitri);
    // check("9999999999.dharitri", "9999999999.dharitri", SuffixType::Dharitri);
    // check(
    //     "coolname0001.dharitri",
    //     "coolname0001.dharitri",
    //     SuffixType::Dharitri,
    // );

    // undefined behavior for invalid names
    // tests are only for ensuring that the function doesn't panic
    // check("aa.x", "aa.dharitri", SuffixType::X);
    // check("test.abc.x", "test.abc.dharitri", SuffixType::X);
    // check("test.abc.foo", "test.abc.foo", SuffixType::Dharitri);
    // check("test", "test", SuffixType::Dharitri);
    // check("test.abc", "test.abc", SuffixType::Dharitri);
// }
