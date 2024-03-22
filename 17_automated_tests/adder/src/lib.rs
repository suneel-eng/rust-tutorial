#[cfg(test)] // cfg means configuration
mod tests {
    // ========= Tests ============

    // For all below test functions you can pass second argument of type string
    // to summarize the error.
    #[test]
    fn boolean() {
        assert!(true); // this will pass if the argument is true else it will fail
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4); // this test will pass
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail"); // this test will fail due to panic
    // }

    #[test]
    fn not_equal() {
        assert_ne!(2 + 2, 5); // this test will pass
    }

    #[test]
    #[should_panic]
    fn another_1() {
        panic!("This won't fail"); // this test will pass
    }

    #[test]
    #[ignore]
    fn ignored() {
        panic!("This test will be ignored.")
    }
}
