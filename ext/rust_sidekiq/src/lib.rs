#[macro_use]
extern crate ruru;
extern crate redis;
extern crate rustc_serialize;

use ruru::{Class, Object};

use server::SidekiqServer;

mod server;

class!(RustSidekiq);

methods!(
    RustSidekiq,
    itself,

    fn start() -> RustSidekiq {
        SidekiqServer::new().start();

        itself
    }
);

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Init_librust_sidekiq() {
    Class::new("RustSidekiq", None).define(|itself| {
        itself.def("start", start);
    });
}
