[cs140e](https://cs140e.sergio.bz)

rust version:
```
rustup default nightly-2019-06-18
```

qemu:
```shell script
# kernel/Cargo.toml
qemu-system-aarch64 -M raspi3 \
 -serial null -serial mon:stdio \
 -kernel build/kernel.bin -s -sd ../files/sd.img
```
