use rcgen;

use crate::error::Result;

const PROTOCOL_VERSION: u8 = 1;

// Usually PROTOCOL_VERSION
async fn connection_info(writer: &mut impl std::io::Write, protocol_version: u8, hostname: &str, port: u32, server_cert: rcgen::Certificate) -> Result<()> {
    use std::io::Write;

    writeln!(
        writer,
        // "localhost:10000"
        "{}|5|tcp|{}:{}|grpc|{}",
        protocol_version,
        hostname,
        port.to_string(),
        base64::encode_config(
            server_cert.serialize_der().unwrap(),
            base64::STANDARD_NO_PAD
        )
    );
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::api::negotiation::{connection_info, PROTOCOL_VERSION};

    #[tokio::test]
    async fn outputs_connection_info_to_stdio() {
        let mut stdout = std::io::stdout();
        connection_info(&mut stdout, PROTOCOL_VERSION, todo!()).await.unwrap();
    }
}
