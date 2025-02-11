const ROOT_DIR: &str = "./../";
const OUT_DIR: &str = "./protobuf-core/src";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configured_builder = tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .use_arc_self(true)
        .out_dir("./../protobuf-core/src/generated/");

    // TODO - read the protos from the proto directory programatically
    println!("compiling protos!");
    configured_builder.compile_protos(
        &[
            "./../proto/auctioneerapis/v1/payload_delivery.proto",
            "./../proto/sequencerapis/v1/get_payload.proto",
        ],
        &["./../proto/", "./../vendor/"],
    )?;

    Ok(())
}
