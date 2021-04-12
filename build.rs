fn main() {
    let mut prost_build_config = prost_build::Config::default();
    prost_build_config.out_dir("src/proto");
    prost_build_config
        .compile_protos(&["proto/helloworld.proto"], &["proto/"])
        .unwrap()
}
