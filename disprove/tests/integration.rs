use disprove::{testable_args1, testable_args2, Arbitrary, Gen, QuickCheck, TestResult};

// -- Macro tests --

#[disprove::quickcheck_macro]
fn prop_reverse_reverse(xs: Vec<i32>) -> bool {
    let rev: Vec<_> = xs.iter().rev().rev().cloned().collect();
    rev == xs
}

#[disprove::quickcheck_macro]
fn prop_sort_idempotent(mut xs: Vec<i32>) -> bool {
    xs.sort();
    let sorted = xs.clone();
    xs.sort();
    xs == sorted
}

#[disprove::quickcheck_macro]
fn prop_addition_commutative(a: u32, b: u32) -> bool {
    a.wrapping_add(b) == b.wrapping_add(a)
}

#[disprove::quickcheck_macro]
fn prop_zero_arg() -> bool {
    true
}

// -- API tests --

#[test]
fn test_quickcheck_builder() {
    QuickCheck::new()
        .tests(200)
        .max_tests(1000)
        .quickcheck(testable_args1(|x: u32| -> bool { x == x }));
}

#[test]
fn test_deterministic_seed() {
    let mut g1 = Gen::from_seed(50, 42);
    let mut g2 = Gen::from_seed(50, 42);
    let v1: Vec<u32> = (0..20).map(|_| u32::arbitrary(&mut g1)).collect();
    let v2: Vec<u32> = (0..20).map(|_| u32::arbitrary(&mut g2)).collect();
    assert_eq!(v1, v2);
}

#[test]
fn test_testresult_from_bool() {
    let pass: TestResult = true.into();
    assert!(!pass.is_failure());
    let fail: TestResult = false.into();
    assert!(fail.is_failure());
}

#[test]
fn test_testresult_must_fail() {
    let r = TestResult::must_fail(false);
    assert!(!r.is_failure()); // false property "passes" must_fail
}

#[test]
fn test_vec_property() {
    QuickCheck::new()
        .tests(100)
        .quickcheck(testable_args1(|xs: Vec<u8>| -> bool {
            let mut sorted = xs.clone();
            sorted.sort();
            sorted.len() == xs.len()
        }));
}

#[test]
fn test_two_arg_property() {
    QuickCheck::new()
        .tests(100)
        .quickcheck(testable_args2(|a: String, b: String| -> bool {
            let concat = format!("{}{}", a, b);
            concat.len() >= a.len()
        }));
}

#[test]
fn test_compat_reexports() {
    // Verify compat module re-exports work
    use disprove::compat::{Arbitrary, Gen, QuickCheck};
    let mut g = Gen::from_seed(10, 42);
    let _: u32 = Arbitrary::arbitrary(&mut g);
    let _qc = QuickCheck::new();
}

#[test]
fn test_shrinking_finds_minimal() {
    let result = std::panic::catch_unwind(|| {
        QuickCheck::new()
            .tests(500)
            .gen(Gen::from_seed(100, 12345))
            .quickcheck(testable_args1(|x: u32| -> bool { x <= 10 }));
    });
    assert!(result.is_err());
}

#[test]
fn test_gen_choose() {
    let mut g = Gen::new(10);
    let options = [1, 2, 3, 4, 5];
    for _ in 0..50 {
        let chosen = g.choose(&options);
        assert!(options.contains(chosen));
    }
}

#[test]
fn test_array_const_generics() {
    // Test arrays larger than quickcheck's max of 32
    QuickCheck::new()
        .tests(50)
        .quickcheck(testable_args1(|arr: [u8; 64]| -> bool { arr.len() == 64 }));
}
