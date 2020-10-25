fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &["temporal/api/workflowservice/v1/service.proto"],
            &["proto/temporal", "proto/protobuf/src"],
        )?;
    Ok(())
}