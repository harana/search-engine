pub fn random() -> u64 {
    unsafe {
        static mut STATE: u64 = 0x123456789abcdef0;
        STATE = STATE.wrapping_mul(2862933555777941757).wrapping_add(3037000493);
        STATE
    }
}