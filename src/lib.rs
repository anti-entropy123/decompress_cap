use bitflags::bitflags;

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
#[allow(improper_ctypes)]
pub mod binding {
    include!("../bindings.rs");

    #[allow(clippy::derivable_impls)]
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

bitflags! {
    #[derive(Debug)]
    pub struct HWPerms: u32 {
        const CHERI_PERM_GLOBAL = binding::CC128_PERM_GLOBAL;
        const CHERI_PERM_EXECUTE = binding::CC128_PERM_EXECUTE;
        const CHERI_PERM_LOAD = binding::CC128_PERM_LOAD;
        const CHERI_PERM_STORE = binding::CC128_PERM_STORE;
        const CHERI_PERM_LOAD_CAP = binding::CC128_PERM_LOAD_CAP;
        const CHERI_PERM_STORE_CAP = binding::CC128_PERM_STORE_CAP;
        const CHERI_PERM_STORE_LOCAL_CAP = binding::CC128_PERM_STORE_LOCAL;
        const CHERI_PERM_SEAL = binding::CC128_PERM_SEAL;
        const CHERI_PERM_CCALL = binding::CC128_PERM_CINVOKE;
        const CHERI_PERM_UNSEAL = binding::CC128_PERM_UNSEAL;
        const CHERI_PERM_SYSTEM_REGS = binding::CC128_PERM_ACCESS_SYS_REGS;
        const CHERI_PERM_SET_CID = binding::CC128_PERM_SETCID;
    }

    #[derive(Debug)]
    pub struct UPerms: u32 {
        const CHERI_PERM_SW0 = 1;
        const CHERI_PERM_SW1 = 2;
        const CHERI_PERM_SW2 = 3;
        const CHERI_PERM_SW3 = 4;
    }
}
