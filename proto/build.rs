fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    let build = tonic_build::configure();
    cfg_if::cfg_if! {
        if #[cfg(feature = "client")] {
            build
                .build_transport(false)
                .build_server(false);
        }
    }
    build.compile(&["./src/v1/users.proto"], &["./src/v1"])?;

    Ok(())
}
