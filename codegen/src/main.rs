fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configured_builder = tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .use_arc_self(true)
        .compile_well_known_types(true)
        .bytes([".astria", ".trustedrelay"])
        .extern_path(".google.protobuf", "::pbjson_types")
        .out_dir("./../protobuf-core/src/generated/");

    let prost_config = prost_build_config();

    // TODO - read the protos from the proto directory programatically
    println!("compiling protos!");
    configured_builder.compile_protos_with_config(
        prost_config,
        &[
            "./../proto/trustedrelay/auctioneerapis/v1/allocation_delivery.proto",
            "./../proto/trustedrelay/sequencerapis/v1/get_allocation.proto",
        ],
        &["./../proto/", "./../vendor/"],
    )?;

    Ok(())
}

fn prost_build_config() -> prost_build::Config {
    let mut config = prost_build::Config::new();
    config.enable_type_names();
    config
}