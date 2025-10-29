arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/test_rtic test_rtic.hex
arm-none-eabi-objcopy -O binary --strip-all  target/thumbv7em-none-eabihf/release/test_rtic test_rtic.bin

rem arm-none-eabi-readelf -S target/thumbv7em-none-eabihf/release/test_rtic


arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/debug/test_rtic test_rtic_d.hex
arm-none-eabi-objcopy -O binary --strip-all  target/thumbv7em-none-eabihf/debug/test_rtic test_rtic_d.bin



