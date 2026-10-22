# YAWS Embassy

This is an example how to run [YAWS](https://yaws-rs.github.io/book/) in [Embassy](https://embassy.dev/) embedded framework.

Supported embedded targets:
- Raspberry Pico W 2040
- Raspberry Pico W 2350

It creates an open WiFi Access Point with SSID Rick-Roll and provides default response to any HTTPS request at https://test.rustcryp.to:1234/

### Usage

Generate TLS Certificates & Key material with OpenSSL
```ignore
cd test_certs
make prime256v1
```

Convert the above material into correct form
```ignore
cd pem_rfc7468
cargo run --target [your target]
```

NOTE: Due to embedded setup you will need to set up the target manually to your compiler host target.

See rust [target list](https://doc.rust-lang.org/beta/rustc/platform-support.html).

Configure the runner:
```ignore
vi .cargo/config.toml
```

If you have a debug probe use probe-rs and if not, use the picotool and press BOOTSEL before connecting the Pico into USB.

Run
```ignore
cargo run
```

You can also use [cargo embed](https://probe.rs/docs/tools/cargo-embed/)
```ignore
cargo embed with_rtt
```

When you run the example through [probe.rs](https://probe.rs) i.e. using picodebug / debugger you will get logging through [RTT](https://pico.implrust.com/debugging/rtt.html).

### Credits

This example is built on top of [embassy/examples/wifi_ap_tcp_server](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_ap_tcp_server.rs).

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions.

