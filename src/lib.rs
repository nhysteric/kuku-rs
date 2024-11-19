pub(crate) type LocaltionType = u32;
pub(crate) type TableSizeType = LocaltionType;
pub(crate) type ItemType = [u8; 16];
pub mod hash;
pub mod kuku;
pub mod locfunc;

pub fn make_item_from_u64(d1: &u64, d2: &u64) -> ItemType {
    let mut result = [0u8; 16];
    result[..8].copy_from_slice(&d1.to_ne_bytes());
    result[8..].copy_from_slice(&d2.to_ne_bytes());
    result
}
