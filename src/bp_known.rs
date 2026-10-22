//! Known blueprints enum-dispatch

pub use blueprint::{Left, Orbit, Right};

use blueprint_ytls::{CryptoConfig, CryptoRng, TlsServerCtxConfig};

/// All the known Orbits enum-dispatch
pub enum Orbits<Config, Crypto, Rng, App> {
    /// Known yTls Server Orbit
    YtlsServer(blueprint_ytls::TlsServerOrbit<Config, Crypto, Rng>),
    /// Known H11Server Orbit
    H11Server(blueprint_h11spec::H11Serving<App>),
}

impl<Config, Crypto, Rng, App> core::fmt::Debug for Orbits<Config, Crypto, Rng, App> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Orbits::Something")
    }
}

/// No harmonized position currently used
#[derive(Debug)]
pub struct NoPosition;

// TODO: harmonise the error & Position
/// No harmonized error currently used
#[derive(Debug)]
pub struct NoError;

use blueprint_h11spec::{
    RequestBodyReceiver, RequestHeaderReceiver, RequestReceiver, Reset, ResponseSender,
};

impl<Config, Crypto, Rng, App> Orbit for Orbits<Config, Crypto, Rng, App>
where
    Config: TlsServerCtxConfig,
    Crypto: CryptoConfig + Clone,
    Rng: CryptoRng,
    App: ResponseSender
        + RequestReceiver
        + RequestHeaderReceiver
        + RequestBodyReceiver
        + Reset
        + Default,
{
    type Position = NoPosition;
    type Error = NoError;
    #[inline]
    fn advance_with<B, L: Left, R: Right>(
        &mut self,
        b: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<Self::Position, Self::Error> {
        _ = match self {
            Self::YtlsServer(t) => {
                let _ = t.advance_with(b, l, r);
            }
            Self::H11Server(t) => {
                let _ = t.advance_with(b, l, r);
            }
        };
        // TODO: harmonize the error & Position
        Ok(NoPosition)
    }
}
