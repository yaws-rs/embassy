# YAWS Embassy

This is an example how to run [YAWS](https://yaws-rs.github.io/book/) in [Embassy](https://embassy.dev/) embedded framework.

Supported embedded targets:
- Raspberry Pico W or "2040" (default)
- Raspberry Pico 2W or "2530" (Use files from rp235x)

It creates an open WiFi Access Point with SSID Rick-Roll and provides default response to any HTTPS request at https://test.rustcryp.to:1234/

### Usage

Add `test.rustcryp.to` in hosts file / relevant in the host that you are using a client from to resolve to IPv4 address `10.0.0.1`.

Generate TLS Certificates & Key material with OpenSSL
```ignore
cd test_certs
make prime256v1
```

Add the generated CA Certificate `ca.prime256v1.crt` into your client hosts root cert store as trusted.

Convert the above material into correct form
```ignore
cd pem_rfc7468
cargo run --target [your target]
```

NOTE: Due to embedded setup you will need to set up the target manually to your compiler host target.

See rust [target list](https://doc.rust-lang.org/beta/rustc/platform-support.html).

Build
```
cargo build
```

### Run

Configure the runner:
```ignore
vi .cargo/config.toml
```

If you have a debug probe use [probe-rs](https://probe.rs) and if not then use the [picotool](https://github.com/raspberrypi/picotool) without a debugger directly USB attached.

If you use picotool (without debugger) press BOOTSEL before connecting the Pico into USB in the host programmer.

Run to program (and attach with RTT if using debugger)
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

