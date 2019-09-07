//! Root test_project for testing racer

pub use test_crate2::useless_func;

pub struct TestStruct {
    pub name: &'static str,
    pub number: usize,
}

impl TestStruct {
    pub fn new() -> Self {
        TestStruct {
            name: "test-struct",
            number: 0,
        }
    }
}

pub(crate) fn name() {}
