fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/terraform")
        .compile(
            &[
                "../vendor/terraform/docs/plugin-protocol/tfplugin6.1.proto",
                "../vendor/go-plugin/internal/plugin/grpc_stdio.proto",
                "../vendor/go-plugin/internal/plugin/grpc_broker.proto",
            ],
            &[
                "../vendor/terraform/docs/plugin-protocol",
                "../vendor/go-plugin/internal/plugin",
            ],
        )?;
    Ok(())
}
