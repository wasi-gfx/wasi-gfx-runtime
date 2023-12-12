use crate::{
    component::webgpu::pointer_events::{HostPointerUp, PointerEvent, PointerUp, Pollable},
    HostState,
};
use async_std::task::sleep;
use std::time::Duration;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

#[async_trait::async_trait]
impl crate::component::webgpu::pointer_events::Host for HostState {
    async fn up(&mut self) -> wasmtime::Result<wasmtime::component::Resource<PointerUp>> {
        println!("in pointer_events::up");
        let g = self.table_mut().push(HostPointerEvent {}).unwrap();
        Ok(Resource::new_own(g.rep()))
    }
}

#[async_trait::async_trait]
impl HostPointerUp for HostState {
    async fn subscribe(&mut self, self_: Resource<PointerUp>) -> wasmtime::Result<Resource<Pollable>> {
        println!("in subscribe");
        let g: Resource<HostPointerEvent> = Resource::new_own(self_.rep());
        let gg = preview2::subscribe(self.table_mut(), g).unwrap();
        Ok(gg)
    }
    async fn get(&mut self, _self_: Resource<PointerUp>) -> wasmtime::Result<Option<PointerEvent>> {
        println!("in get");
        Ok(Some(PointerEvent { x: 120, y: 120 }))
    }
    fn drop(&mut self, _self_: Resource<PointerUp>) -> wasmtime::Result<()> {
        println!("in drop");
        Ok(())
    }
}

struct HostPointerEvent {}

#[async_trait::async_trait]
impl preview2::Subscribe for HostPointerEvent {
    async fn ready(&mut self) {
        println!("in subscribe::ready");
        sleep(Duration::from_millis(2000)).await
    }
}
