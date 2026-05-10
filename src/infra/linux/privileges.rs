pub fn running_as_root() -> bool {
    unsafe { libc_geteuid() == 0 }
}

unsafe fn libc_geteuid() -> u32 {
    extern "C" {
        fn geteuid() -> u32;
    }

    geteuid()
}
