#![no_std]
#![no_main]

use bsp::raspberry_pi_5::uart::TransmitMode;

mod panic_wait;
mod bsp;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        include_str!("asm/aarch64/boot.S")
    );
}

#[no_mangle]
pub fn main() -> ! {
    bsp::init();

    let builder = bsp::raspberry_pi_5::uart::InstanceBuilder::new(0)
    .with_baud_rate(115200)
    .with_transmit_mode(TransmitMode::Bidirectional)
    .with_word_length(bsp::raspberry_pi_5::uart::WordLength::Bits8);

    let instance = builder.build().expect("Failed to create the UART instance!");


    loop{
        instance.poll_write("We're looping!".as_bytes()).expect("Failed to write a byte to UART 0!");
    };
}