// Mirrors Wiki.js 3's client-side cyrb53 path hash.
#[must_use]
pub fn normalize_path(path: &str) -> String {
    let normalized =
        path.trim_start_matches('/').trim_end_matches('/').to_lowercase();

    if normalized.is_empty() { String::from("home") } else { normalized }
}

#[must_use]
pub fn path_hash(normalized: &str) -> String {
    let mut n: u32 = 0xdead_beef;
    let mut r: u32 = 0x41c6_ce57;

    for unit in normalized.encode_utf16() {
        let unit = u32::from(unit);
        n = (n ^ unit).wrapping_mul(2_654_435_761);
        r = (r ^ unit).wrapping_mul(1_597_334_677);
    }

    n = (n ^ (n >> 16)).wrapping_mul(2_246_822_507);
    n ^= (r ^ (r >> 13)).wrapping_mul(3_266_489_909);
    r = (r ^ (r >> 16)).wrapping_mul(2_246_822_507);
    r ^= (n ^ (n >> 13)).wrapping_mul(3_266_489_909);

    format!("{:x}", (u64::from(r & 0x1f_ffff) << 32) | u64::from(n))
}
