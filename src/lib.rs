#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]

pub mod binding {
    include!("../bindings.rs");

    impl Default for cc128_cap {
        fn default() -> Self {
            Self {
                _cr_cursor: Default::default(),
                cr_pesbt: Default::default(),
                _cr_top: Default::default(),
                cr_base: Default::default(),
                cr_tag: Default::default(),
                cr_bounds_valid: Default::default(),
                cr_exp: Default::default(),
                cr_extra: Default::default(),
            }
        }
    }

    impl cc128_cap {
        pub fn as_raw_ptr(&self) -> *const Self {
            self as *const Self
        }

        pub fn as_mut_raw_ptr(&mut self) -> *mut Self {
            self as *mut Self
        }

        pub fn as_uintptr(&mut self) -> u64 {
            self.as_mut_raw_ptr() as u64
        }
    }
}

pub use binding::cc128_cap;

pub fn cc128_decompress_mem(pesbt: u64, cursor: u64, tag: bool) -> cc128_cap {
    let mut result = cc128_cap::default();
    unsafe { binding::cc128_decompress(pesbt, cursor, tag, result.as_mut_raw_ptr()) }
    result
}

pub fn cc128_get_perms(cap: &mut cc128_cap) -> i32 {
    unsafe { binding::cc128_perms(cap.as_mut_raw_ptr()) }
}
pub fn cc128_get_uperms(cap: &mut cc128_cap) -> i32 {
    unsafe { binding::cc128_uperms(cap.as_mut_raw_ptr()) }
}
pub fn cc128_is_cap_sealed(cap: &mut cc128_cap) -> bool {
    unsafe { binding::cc128_is_sealed(cap.as_mut_raw_ptr()) }
}
