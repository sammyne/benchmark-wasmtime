mod bindgen {
    wasmtime::component::bindgen!({
        path: "../wit",
        imports: {
            default: async,
        },
    });
}

use std::time::Duration;

use anyhow::Context;
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
        api::add_to_linker::<_, HasSelf<_>>(linker, |v: &mut Self| v).context("link Host")?;

        wasmtime_wasi::p2::add_to_linker_async(linker).context("link WASI")
    }
}

impl api::Host for Host {
    async fn echo(&mut self, v: u64) -> u64 {
        v + 1
    }

    async fn sleep(&mut self, ms: u64) {
        tokio::time::sleep(Duration::from_millis(ms)).await;
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
