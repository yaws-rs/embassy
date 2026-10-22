//! Blueprint for our higher level protocols

mod ytls;

use crate::bp_known::Orbits;
use blueprint::Orbit;
use blueprint_h11spec::H11Serving;

use embassy_rp::clocks::RoscRng;
use yaws_app_rick_roll::RickRoll;
pub use ytls::TlsServerConfig;
use ytls_rustcrypto::RustCrypto;

/// Set this to your version of Rick in his text form
const RICK_BYTES: &'static [u8] = include_bytes!("../assets/rick-roll.txt");

pub fn ytls_server_blueprints()
-> Blueprints<1, Orbits<TlsServerConfig, RustCrypto, RoscRng, RickRoll>> {
    let rick: RickRoll = RickRoll::with_rick(RICK_BYTES);
    BlueprintsLayers::<1>::layers([Orbits::YtlsServer(ytls::init_ytls_server())])
        .app(Orbits::H11Server(H11Serving::with_app(rick)))
}

#[derive(Clone, Debug)]
pub struct Blueprints<const LAYERS: usize, O: Orbit> {
    layers: [O; LAYERS],
    app: O,
}

impl<const LAYERS: usize, O: Orbit> Blueprints<LAYERS, O> {
    /// Borrow the intermediate LAYERS as mutable
    pub fn layers_as_mut(&mut self) -> &mut [O; LAYERS] {
        &mut self.layers
    }
    /// Borrow the terminating App layer as mutable
    pub fn app_as_mut(&mut self) -> &mut O {
        &mut self.app
    }
    /// Count of all LAYERS +1 App
    #[allow(dead_code)]
    pub fn count_all_layers(&self) -> usize {
        self.layers.len() + 1
    }
    /// Iterate mutable non-App layers
    #[allow(dead_code)]
    pub fn layers_iter_mut<'a>(&'a mut self) -> impl Iterator + use<'a, LAYERS, O> {
        self.layers.iter_mut()
    }
}

/// Blueprints needs Layers constructor
pub struct BlueprintsLayers<const LAYERS: usize>;

impl<const LAYERS: usize> BlueprintsLayers<LAYERS> {
    /// Set the intermediate layers
    pub fn layers<O: Orbit>(orbits: [O; LAYERS]) -> BlueprintsNeedApp<LAYERS, O> {
        BlueprintsNeedApp { layers: orbits }
    }
    /// No intermediate layers involved
    #[allow(dead_code)]
    pub fn no_layers<O: Orbit>() -> BlueprintsNeedApp<0, O> {
        BlueprintsNeedApp { layers: [] }
    }
}

/// Blueprints needs App constructor
pub struct BlueprintsNeedApp<const LAYERS: usize, O: Orbit> {
    layers: [O; LAYERS],
}

impl<const LAYERS: usize, O: Orbit> BlueprintsNeedApp<LAYERS, O> {
    /// Set the terminating App or final layer
    pub fn app(self, app: O) -> Blueprints<LAYERS, O> {
        Blueprints::<LAYERS, _> {
            layers: self.layers,
            app,
        }
    }
}
