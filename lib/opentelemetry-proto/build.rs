use std::io::Error;

fn main() -> Result<(), Error> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(
            &[
                "src/proto/opentelemetry/proto/common/v1/common.proto",
                "src/proto/opentelemetry/proto/resource/v1/resource.proto",
                "src/proto/opentelemetry/proto/logs/v1/logs.proto",
                "src/proto/opentelemetry/proto/collector/logs/v1/logs_service.proto",
            ],
            &["src/proto"],
        )?;

    Ok(())
}
