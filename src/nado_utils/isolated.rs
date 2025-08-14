pub fn is_isolated_subaccount(subaccount: [u8; 32]) -> bool {
    let (a, b, c) = (
        subaccount[31] as u32,
        subaccount[30] as u32,
        subaccount[29] as u32,
    );
    let last_three_bytes = a | (b << 8) | (c << 16);
    last_three_bytes == 6910831
}

pub fn get_isolated_product_id(subaccount: [u8; 32]) -> u32 {
    if !is_isolated_subaccount(subaccount) {
        return 0;
    }
    let (a, b) = (subaccount[27] as u32, subaccount[26] as u32);
    a | (b << 8)
}
