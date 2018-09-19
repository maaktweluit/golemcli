# golemcli in Rust

This is the golemcli in rust

To start the application use
```
cargo run
```

To build the application use
```
cargo build
```

## TODO:
- [ ] project basics
  - [x] README
  - [x] github
- [ ] golemcli features
  - [x] arguments
  - [x] websocket call
  - [ ] command splitting
    - [ ] help
    - [ ] map to rpc call names
  - [ ] interactive mode
  - [ ] use `verify_ssl` argument
  - [ ] mask password https://crates.io/crates/rpassword
- [ ] tests
  - [ ] unit test
  - [ ] integration test
  - [ ] coverage report
- [ ] ci
  - [x] circle
  - [ ] appveyor
  - [ ] travis
  - [ ] codecov
  - [ ] build user docs
  - [ ] deploy user docs
  - [ ] deploy binaries on tag
