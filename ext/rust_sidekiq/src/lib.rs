#[macro_use]
extern crate ruru;
extern crate redis;
extern crate rustc_serialize;

use ruru::{AnyObject, Class, VM};
use ruru::types::{Argc, Value};
use ruru::traits::Object;

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
pub extern fn Init_librust_sidekiq() {
    Class::new("RustSidekiq").define(|itself| {
        itself.def("start", start);
    });
}
