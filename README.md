# Rust for nrf52840-mdk

https://github.com/makerdiary/nrf52840-mdk

## flashing with cargo flash

This is the recommended way to go for flashing

* Install the dependencies for[cargo-flash](https://crates.io/crates/cargo-flash)
* Install probe-run `cargo install cargo-flash`

Then simply `cargo flash --release --example blinky`

## debugging with probe-run

This is the recommended way to go for debugging

* Install the dependencies for[probe-run](https://crates.io/crates/probe-run) (same as for cargo-flash if you already have that)
* Install probe-run `cargo install probe-run`
* On linux you need udev rules saved to somewhere like /etc/udev/rules.d/50-cmsis-dap.rules

```
# 0d28:0204 DAPLink
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```

Then or reload your udev rules with something like `sudo udevadm control -R`

Then simply `cargo run --example debug`

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `probe-run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/examples/debug`
  (HOST) INFO  flashing program
  (HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
this is what debugging looks like
stack backtrace:
   0: 0x00001fd2 - HardFaultTrampoline
      <exception entry>
   1: 0x0000149c - __udf
   2: 0x00001b2c - cortex_m::asm::udf
   3: 0x00001bc4 - rust_begin_unwind
   4: 0x0000154a - core::panicking::panic_fmt
   5: 0x000014e2 - core::panicking::panic
   6: 0x000002d8 - debug::__cortex_m_rt_main
   7: 0x0000019a - main
   8: 0x00001fbc - ResetTrampoline
   9: 0x00000156 - Reset
```

## Getting softdevice OpenOCD
* Get openocd
* Download [SoftDevice S140](https://www.nordicsemi.com/Software-and-tools/Software/S140/Download) from Nordic. Supported versions are 7.x.x
* Unzip Erase if you need to `openocd -f interface/cmsis-dap.cfg -f target/nrf52.cfg -c "init; reset halt; nrf51 mass_erase; exit"`
* Upload softdevice `openocd -f interface/cmsis-dap.cfg -f target/nrf52.cfg -c "program s140_nrf52_7.0.1_softdevice.hex verify exit"`

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
