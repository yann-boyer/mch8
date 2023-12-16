This is mch8, a small and basic Chip8 emulator/virtual machine written in Rust.







The sole purpose of this Chip8 virtual machine is to get me used to the Rust language.





You need to execute these following commands to build and run the project :


```console
yann@yann-linux-laptop:~$ git clone https://github.com/yann-boyer/mch8
yann@yann-linux-laptop:~$ cd mch8
yann@yann-linux-laptop:~$ cargo install cargo-vcpkg
yann@yann-linux-laptop:~$ cargo vcpkg build
yann@yann-linux-laptop:~$ cargo build --release
yann@yann-linux-laptop:~$ cp beep.wav target/release/
yann@yann-linux-laptop:~$ cd target/release
yann@yann-linux-laptop:~$ ./mch8 my_chip8_rom.rom
```





Copyright (c) 2023 - Yann BOYER