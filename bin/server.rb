require_relative '../ext/rust_sidekiq/target/release/librust_sidekiq'
require_relative '../lib/worker/printer'

puts 'Starting Rust Sidekiq server...'

RustSidekiq.new.start
