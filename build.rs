use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_dir = root.join("gen").join("proto");
    let mut cnidarium_config = prost_build::Config::new();
    cnidarium_config.compile_well_known_types();
    cnidarium_config.extern_path(".google.protobuf", "::pbjson_types");
    cnidarium_config.extern_path(".ibc", "::ibc_proto::ibc");
    cnidarium_config.extern_path(".ics23", "::ics23");
    cnidarium_config.extern_path(".cosmos.ics23", "::ics23");
    tonic_build::configure()
        .out_dir(&target_dir)
        .emit_rerun_if_changed(false)
        // .server_mod_attribute(".", rpc_doc_attr)
        // .client_mod_attribute(".", rpc_doc_attr)
        .compile_with_config(
            cnidarium_config,
            &["./proto/penumbra/penumbra/core/component/dex/v1/dex.proto"],
            &["./proto/penumbra/", "./proto/rust-vendored/"],
        )?;
    // tonic_build::compile_protos("./proto/penumbra/penumbra/core/component/dex/v1/dex.proto")?;
    Ok(())
}
