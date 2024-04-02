use core::panic::PanicInfo;


#[panic_handler]
fn panic(_i: &PanicInfo) -> !{
    loop{};
}