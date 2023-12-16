This is mch8, a small and basic Chip8 emulator/virtual machine written in Rust.







The sole purpose of this Chip8 virtual machine is to get me used to the Rust language.






On Linux you will need to install SDL2 and SDL2_mixer : 
```console
yann@yann-linux-laptop:~$ sudo zypper in libSDL2-2_0-0 libSDL2_mixer-2_0-0 SDL2-devel SDL2_mixer-devel // openSUSE Tumbleweed
yann@yann-linux-laptop:~$ sudo pacman -Syy sdl2 sdl2_mixer // Arch Linux
yann@yann-linux-laptop:~$ sudo apt-get install libsdl2-dev libsdl2-mixer-dev // Debian/Ubuntu
 ```



You need to execute these following commands to build and run the project :


```console
yann@yann-linux-laptop:~$ git clone https://github.com/yann-boyer/mch8
yann@yann-linux-laptop:~$ cd mch8
yann@yann-linux-laptop:~$ cargo build --release
yann@yann-linux-laptop:~$ cp beep.wav target/release/
yann@yann-linux-laptop:~$ cd target/release
yann@yann-linux-laptop:~$ ./mch8 my_chip8_rom.rom
```





Copyright (c) 2023 - Yann BOYER
