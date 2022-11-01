use std::io::Write;
use std::{fs::File, path::Path};

use futures::FutureExt;

fn main() {
    // async_main();

    let path = std::env::var("OUT_DIR").expect("Missing OUT_DIR");
    let path = Path::new(&path).join("mc_mod_data.rs");
    let mut out = File::create(&path).expect("Failed to create mc_mod_data.rs");
    writeln!(
        out,
        r#"[
        ("cf_invmove", 172430),
        ("cf_invmove_compats", 106629),
        ("cf_invmove_legacy_forge", 356293),
        ("cf_invmove_legacy_fabric", 97981),
        ("cf_drip_sounds_forge", 279459),
        ("cf_drip_sounds_fabric", 97981),
        ("mr_invmove", 5435),
        ("mr_invmove_compats", 505),
        ("mr_invmove_legacy_forge", 71),
        ("mr_invmove_legacy_fabric", 1274),
        ("mr_drip_sounds_forge", 502),
        ("mr_drip_sounds_fabric", 10722),
        ]
        "#
    )
    .unwrap();
}

#[tokio::main]
async fn async_main() {
    let cf_api =
        furse::Furse::new(&std::env::var("CF_API_KEY").expect("Missing env var CF_API_KEY"));

    let cf_invmove = cf_api.get_mod(581854).map(|r| r.map(|m| m.download_count));
    let cf_invmove_compats = cf_api.get_mod(581875).map(|r| r.map(|m| m.download_count));
    let cf_invmove_legacy_forge = cf_api.get_mod(390992).map(|r| r.map(|m| m.download_count));
    let cf_invmove_legacy_fabric = cf_api.get_mod(419550).map(|r| r.map(|m| m.download_count));
    let cf_drip_sounds_forge = cf_api.get_mod(390986).map(|r| r.map(|m| m.download_count));
    let cf_drip_sounds_fabric = cf_api.get_mod(419550).map(|r| r.map(|m| m.download_count));

    let mr_api = ferinth::Ferinth::default();

    // let mrmods = MRMods::fetch().await.expect("Failed to fetch CF mods");

    let mr_invmove = mr_api
        .get_project("REfW2AEX")
        .map(|r| r.map(|m| m.downloads));
    let mr_invmove_compats = mr_api
        .get_project("6IpcGP7T")
        .map(|r| r.map(|m| m.downloads));
    let mr_invmove_legacy_forge = mr_api
        .get_project("1mObQ0Yz")
        .map(|r| r.map(|m| m.downloads));
    let mr_invmove_legacy_fabric = mr_api
        .get_project("McNdqLv2")
        .map(|r| r.map(|m| m.downloads));
    let mr_drip_sounds_forge = mr_api
        .get_project("XOhqdyTf")
        .map(|r| r.map(|m| m.downloads));
    let mr_drip_sounds_fabric = mr_api
        .get_project("T8MMXTpr")
        .map(|r| r.map(|m| m.downloads));

    let path = std::env::var("OUT_DIR").expect("Missing OUT_DIR");
    let path = Path::new(&path).join("mc_mod_data.rs");
    let mut out = File::create(&path).expect("Failed to create mc_mod_data.rs");
    writeln!(out, "[").unwrap();

    writeln!(
        out,
        "({:?}, {:?}),",
        "cf_invmove",
        cf_invmove.await.expect("Failed to fetch CF mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "cf_invmove_compats",
        cf_invmove_compats.await.expect("Failed to fetch CF mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "cf_invmove_legacy_forge",
        cf_invmove_legacy_forge
            .await
            .expect("Failed to fetch CF mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "cf_invmove_legacy_fabric",
        cf_invmove_legacy_fabric
            .await
            .expect("Failed to fetch CF mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "cf_drip_sounds_forge",
        cf_drip_sounds_forge.await.expect("Failed to fetch CF mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "cf_drip_sounds_fabric",
        cf_drip_sounds_fabric.await.expect("Failed to fetch CF mod")
    )
    .unwrap();

    writeln!(
        out,
        "({:?}, {:?}),",
        "mr_invmove",
        mr_invmove.await.expect("Failed to fetch MR mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "mr_invmove_compats",
        mr_invmove_compats.await.expect("Failed to fetch MR mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "mr_invmove_legacy_forge",
        mr_invmove_legacy_forge
            .await
            .expect("Failed to fetch MR mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "mr_invmove_legacy_fabric",
        mr_invmove_legacy_fabric
            .await
            .expect("Failed to fetch MR mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "mr_drip_sounds_forge",
        mr_drip_sounds_forge.await.expect("Failed to fetch MR mod")
    )
    .unwrap();
    writeln!(
        out,
        "({:?}, {:?}),",
        "mr_drip_sounds_fabric",
        mr_drip_sounds_fabric.await.expect("Failed to fetch MR mod")
    )
    .unwrap();

    writeln!(out, "]").unwrap();
}
