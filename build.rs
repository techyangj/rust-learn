use prost_build::Config;


fn main() {
    println!("cargo:rerun-if-changed=person.proto");
    println!("cargo:rerun-if-changed=build.rs");

    Config::new()
        .out_dir("src/pb")
       // .bytes(&["."])
        .btree_map(&["scores"])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["person.proto"], &["."])
        .unwrap();
}
