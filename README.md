# aht20 sensor on esp32-c3, no-std

```bash
cargo espflash --release --monitor /dev/ttyUSB0  # adapt your tty here
```

Basically https://github.com/esp-rs/esp-hal/blob/main/esp32c3-hal/examples/hello_world.rs + https://github.com/fmckeogh/aht20

std + wifi version: https://github.com/nim65s/aht20-esp32c3-std
