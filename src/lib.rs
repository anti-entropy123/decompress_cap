use std::os::raw::{c_uint, c_ulong};

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]

pub mod binding {
    include!("../bindings.rs");

    impl Default for cc128_cap_t {
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

    impl cc128_cap_t {
        pub fn as_ptr(&self) -> u64 {
            self as *const Self as u64
        }
    }
}

pub use binding::{cc128_cap, false_};

#[link(kind = "static", name = "cheri")]
extern "C" {
    // static inline void cc128_decompress_mem(uint64_t pesbt, uint64_t cursor, bool tag, cc128_cap_t *cdp)
    fn cc128_decompress(
        pesbt: binding::__uint64_t,
        cursor: binding::__uint64_t,
        tag: binding::__uint32_t,
        result: binding::__uint64_t,
    );

    fn cc128_perms(cap: binding::__uint64_t) -> binding::__uint32_t;
    fn cc128_uperms(cap: binding::__uint64_t) -> binding::__uint32_t;
    fn cc128_is_sealed(cap: binding::__uint64_t) -> binding::__uint32_t;
}

pub fn cc128_decompress_mem(pesbt: c_ulong, cursor: c_ulong, tag: c_uint, result: c_ulong) {
    unsafe { cc128_decompress(pesbt, cursor, tag, result) }
}

pub fn cc128_get_perms(cap: &binding::cc128_cap) -> u32 {
    unsafe { cc128_perms(cap.as_ptr()) }
}
pub fn cc128_get_uperms(cap: &binding::cc128_cap) -> u32 {
    unsafe { cc128_uperms(cap.as_ptr()) }
}
pub fn cc128_is_cap_sealed(cap: &binding::cc128_cap) -> u32 {
    unsafe { cc128_is_sealed(cap.as_ptr()) }
}
