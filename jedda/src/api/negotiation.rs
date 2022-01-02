use rcgen;

use tracing::trace;
use crate::error;
use crate::error::IntoJeddaError;

pub const PROTOCOL_VERSION: u8 = 1;

#[tracing::instrument]
pub fn generate_cert() -> Result<rcgen::Certificate, rcgen::RcgenError> {
    let subject_alt_names =
        vec!["localhost".to_string()];

    let cert = rcgen::generate_simple_self_signed(subject_alt_names)?;
    trace!("\n{}\n", cert.serialize_pem()?);
    Ok(cert)
}

pub fn connection_info(protocol_version: u8, hostname: &str, port: u32, server_cert: rcgen::Certificate) -> String {
    format!(
        // "localhost:10000"
        "{}|5|tcp|{}:{}|grpc|{}",
        protocol_version,
        hostname,
        port.to_string(),
        base64::encode_config(
            server_cert.serialize_der().unwrap(),
            base64::STANDARD_NO_PAD
        )
    )
}

pub fn write_connection_info(writer: &mut impl std::io::Write, protocol_version: u8, hostname: &str, port: u32, server_cert: rcgen::Certificate) -> error::Result<()> {
    let info_line = connection_info(protocol_version, hostname, port, server_cert);
    writeln!(
        writer,
        // "localhost:10000"
        "{}",
        info_line,
    ).negotiation_error("Failed to write connection information to Writer")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_log::test;
    use crate::api::negotiation::{generate_cert, PROTOCOL_VERSION, write_connection_info};

    #[test(tokio::test)]
    async fn outputs_connection_info_to_stdio() -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = std::io::stdout();
        let cert = generate_cert().unwrap();
        write_connection_info(&mut stdout, PROTOCOL_VERSION, "localhost", 12000, cert).unwrap();
        Ok(())
    }
}
