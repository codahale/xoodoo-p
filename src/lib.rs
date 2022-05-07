//! High-performance, pure-Rust implementation of the Xoodoo-_p_ permutation.
//!
//! Optimized for generic 64-bit processors.

#![no_std]

/// The maximum number of rounds possible for Xoodoo.
pub const MAX_ROUNDS: usize = 12;

/// Run the Xoodoo permutation with `R` rounds on the given state.
///
/// ```rust
/// use xoodoo_p::xoodoo;
///
/// // The full 12-round permutation. Test vector from XKCP, rev 2a8d2311.
/// let mut st = [0u32; 12];
/// xoodoo::<12>(&mut st);
/// assert_eq!(st, [
///     0x89D5D88D, 0xA963FCBF, 0x1B232D19, 0xFFA5A014,
///     0x36B18106, 0xAFC7C1FE, 0xAEE57CBE, 0xA77540BD,
///     0x2E86E870, 0xFEF5B7C9, 0x8B4FADF2, 0x5E4F4062,
/// ]);
///
/// // The reduced 6-round permutation. Test vector from XKCP, rev 2a8d2311.
/// let mut st = [0u32; 12];
/// xoodoo::<6>(&mut st);
/// assert_eq!(st, [
///     0x28C9CEA3, 0xAD204F60, 0x2EC3D0D6, 0xF050C7C5,
///     0x08DC1225, 0x61992304, 0x9E0D402D, 0x42D59B9B,
///     0x1E6114FC, 0x186EB697, 0x35DBBC7F, 0xA1F9104E,
/// ]);
/// ```
pub fn xoodoo<const R: usize>(st: &mut [u32; 12]) {
    debug_assert!(R <= MAX_ROUNDS, "R must be <= {} (was {}", R, MAX_ROUNDS);

    // Load lanes into registers.
    let mut st00 = st[0];
    let mut st01 = st[1];
    let mut st02 = st[2];
    let mut st03 = st[3];
    let mut st04 = st[4];
    let mut st05 = st[5];
    let mut st06 = st[6];
    let mut st07 = st[7];
    let mut st08 = st[8];
    let mut st09 = st[9];
    let mut st10 = st[10];
    let mut st11 = st[11];

    // Perform last R rounds.
    for &round_key in &ROUND_KEYS[MAX_ROUNDS - R..MAX_ROUNDS] {
        let p0 = st00 ^ st04 ^ st08;
        let p1 = st01 ^ st05 ^ st09;
        let p2 = st02 ^ st06 ^ st10;
        let p3 = st03 ^ st07 ^ st11;

        let e0 = p3.rotate_left(5) ^ p3.rotate_left(14);
        let e1 = p0.rotate_left(5) ^ p0.rotate_left(14);
        let e2 = p1.rotate_left(5) ^ p1.rotate_left(14);
        let e3 = p2.rotate_left(5) ^ p2.rotate_left(14);

        let tmp0 = e0 ^ st00 ^ round_key;
        let tmp1 = e1 ^ st01;
        let tmp2 = e2 ^ st02;
        let tmp3 = e3 ^ st03;
        let tmp4 = e3 ^ st07;
        let tmp5 = e0 ^ st04;
        let tmp6 = e1 ^ st05;
        let tmp7 = e2 ^ st06;
        let tmp8 = (e0 ^ st08).rotate_left(11);
        let tmp9 = (e1 ^ st09).rotate_left(11);
        let tmp10 = (e2 ^ st10).rotate_left(11);
        let tmp11 = (e3 ^ st11).rotate_left(11);

        st00 = (!tmp4 & tmp8) ^ tmp0;
        st01 = (!tmp5 & tmp9) ^ tmp1;
        st02 = (!tmp6 & tmp10) ^ tmp2;
        st03 = (!tmp7 & tmp11) ^ tmp3;

        st04 = ((!tmp8 & tmp0) ^ tmp4).rotate_left(1);
        st05 = ((!tmp9 & tmp1) ^ tmp5).rotate_left(1);
        st06 = ((!tmp10 & tmp2) ^ tmp6).rotate_left(1);
        st07 = ((!tmp11 & tmp3) ^ tmp7).rotate_left(1);

        st08 = ((!tmp2 & tmp6) ^ tmp10).rotate_left(8);
        st09 = ((!tmp3 & tmp7) ^ tmp11).rotate_left(8);
        st10 = ((!tmp0 & tmp4) ^ tmp8).rotate_left(8);
        st11 = ((!tmp1 & tmp5) ^ tmp9).rotate_left(8);
    }

    // Load registers into lanes.
    st[0] = st00;
    st[1] = st01;
    st[2] = st02;
    st[3] = st03;
    st[4] = st04;
    st[5] = st05;
    st[6] = st06;
    st[7] = st07;
    st[8] = st08;
    st[9] = st09;
    st[10] = st10;
    st[11] = st11;
}

const ROUND_KEYS: [u32; MAX_ROUNDS] = [
    0x00000058, 0x00000038, 0x000003C0, 0x000000D0, 0x00000120, 0x00000014, 0x00000060, 0x0000002C,
    0x00000380, 0x000000F0, 0x000001A0, 0x00000012,
];
