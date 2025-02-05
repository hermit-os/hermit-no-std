# hermit-no-std

`hermit-no-std` allows to boot HermitOS without _Rust Standard Library_.
The interface to the kernel is not (yet) standardized and can change between kernel versions.

## Requirements

* [`rustup`](https://www.rust-lang.org/tools/install)

## Building the kernel

```sh
cargo build
```

## Booting the kernel

Boot kernel with `cargo run` or use following command

```sh
qemu-system-x86_64 -display none -serial stdio -kernel hermit-loader-x86_64 -cpu Skylake-Client -device isa-debug-exit,iobase=0xf4,iosize=0x04 -smp 1 -m 512M -netdev user,id=u1,hostfwd=tcp::9975-:9975,hostfwd=udp::9975-:9975,net=192.168.76.0/24,dhcpstart=192.168.76.9 -device virtio-net-pci,netdev=u1,disable-legacy=on -initrd target/x86_64-unknown-none/debug/boot_image
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

The kernel is being developed on [hermit-os/kernel](https://github.com/hermit-os/kernel).
Create your own fork, send us a pull request, and chat with us on [Zulip](https://hermit.zulipchat.com/).
