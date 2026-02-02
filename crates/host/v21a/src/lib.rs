mod bindgen {
    wasmtime::component::bindgen!({
        path: "../wit",
        async: true,
    });
}

use std::time::Duration;

use anyhow::Context as _;
use wasmtime::component::Linker;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};

use crate::bindgen::sammyne::host::api;

#[derive(Default)]
pub struct Host {
    wasi_state: MyWasiState,
}

pub struct MyWasiState {
    ctx: WasiCtx,
    resources: ResourceTable,
}

impl Host {
    pub fn link(linker: &mut Linker<Self>) -> wasmtime::Result<()> {
        api::add_to_linker::<Self, Self>(linker, |v: &mut Self| v).context("link Host")?;

        wasmtime_wasi::add_to_linker_async::<Self>(linker).context("link WASI")
    }
}

#[async_trait::async_trait]
impl api::Host for Host {
    async fn echo(&mut self, v: u64) -> u64 {
        v + 1
    }

    async fn sleep(&mut self, ms: u64) {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }
}

impl WasiView for Host {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_state.ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.wasi_state.resources
    }
}

impl Default for MyWasiState {
    fn default() -> Self {
        Self {
            ctx: wasmtime_wasi::WasiCtxBuilder::new().build(),
            resources: ResourceTable::new(),
        }
    }
}
