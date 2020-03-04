extern crate protobuf_codegen_pure;

fn main() {
//    protobuf_codegen_pure::Args::new()
//        .out_dir("src/protos")
//        .inputs(&["protos/a.proto", "protos/b.proto"])
//        .include("protos")
//        .run()
//        .expect("protoc");

    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/proto",
        input: &["proto/struct.proto"],
        includes: &["proto"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("protoc");
}