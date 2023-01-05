use std::{env, path::PathBuf};

#[allow(dead_code)]
fn main() {
    // for (key, value) in env::vars() {
    //     println!("{key}: {value}");
    // }

    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    println!("get out_dir {}", out_dir.display());

    tonic_build::configure()
//        .file_descriptor_set_path(out_dir.join("streaming_descriptor.bin"))
        .compile(&["proto/grpc/streaming.proto"], &["proto"])
        .unwrap();

    // let path = std::env::current_dir().unwrap();
    // println!("get dir {}", path.display());
    // let file = path.join(r"examples/src/proto_build/streaming.proto");
    // println!("get file {}", file.display());
    // let fs = std::fs::read(file.clone());
    // tonic_build::compile_protos(file).unwrap();
    println!("build ok");
}