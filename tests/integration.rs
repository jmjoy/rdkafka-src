use rdkafka_src::Build;
use std::env::temp_dir;

#[test]
fn integrate() {
    const VERSIONS: &[&str] = &[
        "2.0", "2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7", "2.8",
    ];

    for version in VERSIONS {
        let out_dir = temp_dir().join("rdkafka-src");
        let art = Build::new(&out_dir, *version).build().unwrap();
        assert_eq!(art.install_dir, out_dir.join("librdkafka").join(version));
        assert_eq!(
            art.include_dir,
            out_dir.join("librdkafka").join(version).join("include")
        );
        assert_eq!(
            art.lib_dir,
            out_dir.join("librdkafka").join(version).join("lib")
        );
    }
}
