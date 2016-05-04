use redis::{Client, Commands, Connection, RedisResult};
use rustc_serialize::Encodable;
use rustc_serialize::json;

use ruru::{AnyObject, Class, RString};
use ruru::traits::Object;

#[derive(RustcEncodable, RustcDecodable, Debug)]
struct Job {
    class: String,
    args: Vec<String>,
}

pub struct SidekiqServer {
    connection: Connection
}

impl SidekiqServer {
    pub fn new() -> Self {
        SidekiqServer {
            connection: Self::get_connection()
        }
    }

    fn get_connection() -> Connection {
        let client = Client::open("redis://127.0.0.1/").unwrap();

        client.get_connection().unwrap()
    }

    pub fn start(&self) {
        println!("Server is running...");

        let mut performed_jobs = 0u64;

        loop {
            match self.reserve() {
                Some(_) => performed_jobs += 1,
                None => {}
            }

            println!("{} jobs performed", performed_jobs);
        }
    }

    fn reserve(&self) -> Option<()> {
        println!("### Checking the queue");

        let result: Option<Vec<String>> = self.connection.brpop("queue:default", 2).unwrap();

        match result {
            Some(brpop_result) => {
                self.perform(&brpop_result[1]);

                Some(())
            },
            None => None
        }
    }

    fn perform(&self, json_job: &str) {
        let job: Job = json::decode(&*json_job).unwrap();

        let class = Class::from_existing(&job.class);
        let args: Vec<AnyObject> =
            job.args
                .iter()
                .map(|a| RString::new(a).to_any_object())
                .collect();

        // Magic happens here :)
        // Initialize Ruby class and call `#perform` with given args
        class.new_instance(vec![]).send("perform", args);
    }
}
