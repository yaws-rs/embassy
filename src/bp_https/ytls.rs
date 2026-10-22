//! yTLS Orbit initialization

use blueprint_ytls::{Alpn, TlsServerCtxConfig, TlsServerOrbit};
use embassy_rp::clocks::RoscRng;
use ytls_rustcrypto::RustCrypto;

pub fn init_ytls_server() -> TlsServerOrbit<TlsServerConfig, RustCrypto, RoscRng> {
    let crypto = ytls_rustcrypto::RustCrypto;
    let rng = RoscRng;
    let tls_config_server = TlsServerConfig::default();
    TlsServerOrbit::with_required(tls_config_server, crypto, rng)
}

const CA_BYTES: &'static [u8] = include_bytes!("../../test_certs/ca_data.bin");
const CERT_BYTES: &'static [u8] = include_bytes!("../../test_certs/cert_data.bin");
const KEY_BYTES: &'static [u8] = include_bytes!("../../test_certs/key_data.bin");

pub struct TlsServerConfig;

impl TlsServerCtxConfig for TlsServerConfig {
    #[inline]
    fn dns_host_name(&self, host: &str) -> bool {
        host == "test.rustcryp.to"
    }
    #[inline]
    fn alpn<'r>(&self, alpn: Alpn<'r>) -> bool {
        if alpn == Alpn::Http11 {
            return true;
        }
        false
    }
    #[inline]
    fn server_private_key(&self) -> &[u8] {
        &KEY_BYTES
    }
    #[inline]
    fn server_cert_chain(&self) -> &[u8] {
        &[0, 1]
    }
    #[inline]
    fn server_cert(&self, id: u8) -> &[u8] {
        match id {
            0 => &CERT_BYTES,
            1 => &CA_BYTES,
            _ => unreachable!(),
        }
    }
}

impl Default for TlsServerConfig {
    fn default() -> TlsServerConfig {
        Self
    }
}
