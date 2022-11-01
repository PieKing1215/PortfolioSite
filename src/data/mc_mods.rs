use std::collections::HashMap;


pub fn invmove_download_count() -> usize {
    let raw: HashMap<&str, usize> = include!(concat!(env!("OUT_DIR"), "/mc_mod_data.rs")).into();
    vec![
        *raw.get("cf_invmove").unwrap(),
        *raw.get("cf_invmove_compats").unwrap(),
        *raw.get("cf_invmove_legacy_forge").unwrap(),
        *raw.get("cf_invmove_legacy_fabric").unwrap(),
        *raw.get("mr_invmove").unwrap(),
        *raw.get("mr_invmove_compats").unwrap(),
        *raw.get("mr_invmove_legacy_forge").unwrap(),
        *raw.get("mr_invmove_legacy_fabric").unwrap(),
    ].iter().sum()
}
