# Rust Sidekiq server

## Description

An example of simple sidekiq-compatible server which performs Sidekiq jobs.

This server does not require to change any Ruby code, neither
for pushing jobs to queues nor for using workers.

## Performace

 - Both servers were run in Single-threaded mode

 - Simple workers were used to compare only the speed of fetching jobs, but not the performance of Ruby
   workers themselves.

```
10000 jobs

sidekiq - 3.42 sec
rust_sidekiq - 0.94 sec (~3.63 times faster)
```

## Technologies

- [Rust](https://www.rust-lang.org/)
  to write native extension
- [Sidekiq](https://github.com/mperham/sidekiq)
  to push jobs (simulate the real Ruby application)
- [Ruru](https://github.com/d-unseductable/ruru)
  to allow Rust to communicate with Ruby (declare classes, run workers etc)

## Current limitations

- `String` arguments for jobs

- Single-threaded

## Usage

1. Compile Rust library

  ```bash
  $ cd ext/rust_sidekiq
  $ cargo build --release

  $ cd ../..
  ```

2. Install `sidekiq` gem

  ```bash
  $ bundle install
  ```

3. Push some Sidekiq jobs to the queue

  ```bash
  $ bundle exec ruby bin/push.rb
  ```

4. Start `rust_sidekiq` server

  ```bash
  $ bundle exec ruby bin/server.rb
  ```

5. You can also push another 10k jobs and start real Sidekiq server

  ```bash
  $ bundle exec ruby bin/push.rb
  $ bundle exec sidekiq -r ./lib/worker/printer.rb -c 1
  ```

Special thanks go to [Julien Blanchard](http://julienblanchard.com/) whose article inspired for
creating this example with both Rust and Ruby!
