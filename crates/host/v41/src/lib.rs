mod bindgen {
    wasmtime::component::bindgen!(in "../wit");
}

use std::time::Duration;

use wasmtime::component::{HasSelf, Linker};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};

use crate::bindgen::sammyne::host::api;

#[derive(Default)]
pub struct Host {
    wasi_state: MyWasiState,
}

#[derive(Default)]
pub struct MyWasiState {
    ctx: WasiCtx,
    resources: ResourceTable,
}

impl Host {
    pub fn link(linker: &mut Linker<Self>) -> wasmtime::Result<()> {
        api::add_to_linker::<_, HasSelf<_>>(linker, |v: &mut Self| v)
    }
}

impl api::Host for Host {
    fn sleep(&mut self, ms: u64) {
        std::thread::sleep(Duration::from_millis(ms));
    }
}

impl WasiView for Host {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_state.ctx,
            table: &mut self.wasi_state.resources,
        }
    }
}
