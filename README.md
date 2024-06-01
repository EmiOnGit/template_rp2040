# Minimal pico(rp 2040) template

Most 'templates' for the raspberry pico are rather involved and import many dependencies you might not want.
This template aims to include only a couple of crates that I considered absolutely needed (if you don't want to write a bootloader yourself that is).

The template is constructed to be used for a *single* pico, which is directly flashed over a usb connection.

To install this template run:
```bash
git clone https://github.com/emiongit/template_rp2040
cd template_rp2040
rustup target install thumbv6m-none-eabi
cargo install --locked elf2uf2-rs
```

If you want to run your project use:
```bash
cargo r -r
# or
cargo b -r && elf2uf2-rs -d target/thumbv6m-none-eabi/release/bin
```
Make sure your pico is connected to your pc and ready to be flashed (connected while pressing the BOOTSEL button).

This template assumes that *you have a LED connected at gpio20*
You can change that in the `src/bin/bin.rs` file.
Also if you are using a *pico (H)* and not a *pico W(H)* then you can also use the *onboard LED*. 
See the comments in the `bin.rs` to change to the onboard LED! 
