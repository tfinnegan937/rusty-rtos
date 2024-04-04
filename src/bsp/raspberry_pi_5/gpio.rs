const GPIO_BASE: usize = 0x1F000d0000;
const GPIO_COUNT: usize = 28;                         

const GPIO_ADDRESSES: [usize; GPIO_COUNT] = [
    GPIO_BASE,        //GPIO0
    GPIO_BASE + 0x08, //GPIO1
    GPIO_BASE + 0x10, //GPIO2
    GPIO_BASE + 0x18, //GPIO3
    GPIO_BASE + 0x20, //GPIO4
    GPIO_BASE + 0x28, //GPIO5
    GPIO_BASE + 0x30, //GPIO6
    GPIO_BASE + 0x38, //GPIO7
    GPIO_BASE + 0x40, //GPIO8
    GPIO_BASE + 0x48, //GPIO9
    GPIO_BASE + 0x50, //GPIO10
    GPIO_BASE + 0x58, //GPIO11
    GPIO_BASE + 0x60, //GPIO12
    GPIO_BASE + 0x68, //GPIO13
    GPIO_BASE + 0x70, //GPIO14
    GPIO_BASE + 0x78, //GPIO15
    GPIO_BASE + 0x80, //GPIO16
    GPIO_BASE + 0x88, //GPIO17
    GPIO_BASE + 0x90, //GPIO18
    GPIO_BASE + 0x98, //GPIO19
    GPIO_BASE + 0xA0, //GPIO20
    GPIO_BASE + 0xA8, //GPIO21
    GPIO_BASE + 0xB0, //GPIO22
    GPIO_BASE + 0xB8, //GPIO23
    GPIO_BASE + 0xC0, //GPIO24
    GPIO_BASE + 0xC8, //GPIO25
    GPIO_BASE + 0xD0, //GPIO26
    GPIO_BASE + 0xD8  //GPIO27
];

#[allow(dead_code)]
enum GpioRegisterAccessType{
    ReadOnly,
    WriteOnly,
    ReadWrite
}

#[allow(dead_code)]
/// The total list of all functions for all programmable Gpio pins.
enum GpioFunction{
    Spi0Sio0,
    Spi0Sio1,
    Spi0Sio2,
    Spi0Sio3,
    Spi0Csn0,
    Spi0Csn1,
    Spi0Csn2,
    Spi0Csn3,
    Spi0Sclk,
    Spi1Sio0,
    Spi1Sio1,
    Spi1Sio2,
    Spi1Sio3,
    Spi1Csn0,
    Spi1Csn1,
    Spi1Csn2,
    Spi1Csn3,
    Spi1Sclk,
    Spi2Sio0,
    Spi2Sio1,
    Spi2Sio2,
    Spi2Sio3,
    Spi2Csn0,
    Spi2Csn1,
    Spi2Csn2,
    Spi2Csn3,
    Spi2Sclk,
    Spi3Sio0,
    Spi3Sio1,
    Spi3Sio2,
    Spi3Sio3,
    Spi3Csn0,
    Spi3Csn1,
    Spi3Csn2,
    Spi3Csn3,
    Spi3Sclk,
    Spi4Sio0,
    Spi4Sio1,
    Spi4Sio2,
    Spi4Sio3,
    Spi4Csn0,
    Spi4Csn1,
    Spi4Csn2,
    Spi4Csn3,
    Spi4Sclk,
    Spi5Sio0,
    Spi5Sio1,
    Spi5Sio2,
    Spi5Sio3,
    Spi5Csn0,
    Spi5Csn1,
    Spi5Csn2,
    Spi5Csn3,
    Spi5Sclk,
    SdioDat0,
    Sdio0Dat1,
    Sdio0Dat2,
    Sdio0Dat3,
    GpClk0,
    GpClk1,
    GpClk2,
    Pwm00,
    Pwm01,
    Pwm02,
    Pwm03,
    DpiPclk,
    DpiDe,
    DpiVsync,
    DpiHsync,
    DpiD0,
    DpiD1,
    DpiD2,
    DpiD3,
    DpiD4,
    DpiD5,
    DpiD6,
    DpiD7,
    DpiD8,
    DpiD9,
    DpiD10,
    DpiD11,
    DpiD12,
    DpiD13,
    DpiD14,
    DpiD15,
    DpiD16,
    DpiD17,
    DpiD18,
    DpiD19,
    DpiD20,
    DpiD21,
    DpiD22,
    DpiD23,
    Uart0Tx,
    Uart0Rx,
    Uart0IrTx,
    Uart0IrRx,
    Uart0Ri,
    Uart0Dtr,
    Uart0Dcd,
    Uart0Dsr,
    Uart1Tx,
    Uart1Rx,
    Uart1Cts,
    Uart1Rts,
    Uart2Tx,
    Uart2Rx,
    Uart2Cts,
    Uart2Rts,
    Uart3Tx,
    Uart3Rx,
    Uart3Cts,
    Uart3Rts,
    Uart4Tx,
    Uart4Rx,
    Uart4Cts,
    Uart4Rts,
    Mpi0DsiTe,
    Mpi1DsiTe,
    I2c0Sclk,
    I2c0Ws,
    I2c0Sdi0,
    I2c0SdO0,
    I2c0Sda,
    I2c0Scl,
    I2c1Sclk,
    I2c1Ws,
    I2c1Sdi0,
    I2c1SdO0,
    I2c1Sda,
    I2c1Scl,
    I2c2Sda,
    I2c2Scl,
    I2c3Sda,
    I2c3Scl,
    SysRio0,
    SysRio1,
    SysRio2,
    SysRio3,
    SysRio4,
    SysRio5,
    SysRio6,
    SysRio7,
    SysRio8,
    SysRio9,
    SysRio10,
    SysRio11,
    SysRio12,
    SysRio13,
    SysRio14,
    SysRio15,
    SysRio16,
    SysRio17,
    SysRio18,
    SysRio19,
    SysRio20,
    SysRio21,
    SysRio22,
    SysRio23,
    SysRio24,
    SysRio25,
    SysRio26,
    SysRio27,
    ProcRio0,
    ProcRio1,
    ProcRio2,
    ProcRio3,
    ProcRio4,
    ProcRio5,
    ProcRio6,
    ProcRio7,
    ProcRio8,
    ProcRio9,
    ProcRio10,
    ProcRio11,
    ProcRio12,
    ProcRio13,
    ProcRio14,
    ProcRio15,
    ProcRio16,
    ProcRio17,
    ProcRio18,
    ProcRio19,
    ProcRio20,
    ProcRio21,
    ProcRio22,
    ProcRio23,
    ProcRio24,
    ProcRio25,
    ProcRio26,
    ProcRio27,   
    Pio0,
    Pio1,
    Pio2,
    Pio3,
    Pio4,
    Pio5,
    Pio6,
    Pio7,
    Pio8,
    Pio9,
    Pio10,
    Pio11,
    Pio12,
    Pio13,
    Pio14,
    Pio15,
    Pio16,
    Pio17,
    Pio18,
    Pio19,
    Pio20,
    Pio21,   
    Pio22,
    Pio23,
    Pio24,
    Pio25,
    Pio26,
    Pio27,
    AudioInClk,
    AudioInDat0,
    AudioInDat1
}

/// This is an implementation of the Gpio Function Select Table as shown in section 3.1.1 of the RP1 peripherals document.
/// A None in this table indicates that either that function has not been implemented by this BSP, OR that the function select
/// value is reserved by the board manufacturer
#[allow(dead_code)]
const GPIO_FUNCTION_TABLE: [[Option<GpioFunction>; 9]; GPIO_COUNT] = [
    // Is a large table like this tedious and unwieldy? Totally...But it matches the documentation (making it easy to validate)
    // and it allows for me to create a relatively easy-to-use interface that forces a loud and obvious error if some invalid
    // combination of functions are selected.

    /*         a0                            a1                           a2                           a3                           a4                           a5                           a6                            a7                        a8                         */
    /*Gpio0*/ [Some(GpioFunction::Spi0Sio0), Some(GpioFunction::DpiPclk), Some(GpioFunction::Uart1Tx), Some(GpioFunction::I2c0Sda), None,                        Some(GpioFunction::SysRio0), Some(GpioFunction::ProcRio0), Some(GpioFunction::Pio0), Some(GpioFunction::Spi2Csn0)],
    /*Gpio1*/ [Some(GpioFunction::Spi0Sio2), Some(GpioFunction::DpiDe),   Some(GpioFunction::Uart1Rx), Some(GpioFunction::I2c0Scl), None,                        Some(GpioFunction::SysRio1), Some(GpioFunction::ProcRio1), Some(GpioFunction::Pio1), Some(GpioFunction::Spi2Sio1)],
    /*Gpio2*/ [None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio3*/ [None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio4*/ [None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio5*/ [None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio6*/ [None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio7*/ [None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio8*/ [None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio9*/ [None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio10*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio11*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio12*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio13*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio14*/[None,                         None,                        None,                        None,                        Some(GpioFunction::Uart0Tx), None,                        None,                         None,                     None                        ],
    /*Gpio15*/[None,                         None,                        None,                        None,                        Some(GpioFunction::Uart0Rx), None,                        None,                         None,                     None                        ],
    /*Gpio16*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio17*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio18*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio19*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio20*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio21*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio22*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio23*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio24*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio25*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio26*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
    /*Gpio27*/[None,                         None,                        None,                        None,                        None,                        None,                        None,                         None,                     None                        ],
];                       


#[allow(dead_code)]
struct GpioRegisterDefinition{
    offset: usize,
    bit_width: usize,
    access_type: GpioRegisterAccessType
}

#[allow(dead_code)]
pub struct GpioRegReadResult{
    pub value: usize,
    pub bit_width: usize
}

#[allow(dead_code)]
/// Contains the properties of a GpioStatus register
const GPIO_STATUS: GpioRegisterDefinition = GpioRegisterDefinition{
    offset: 0x00,
    bit_width: 32,
    access_type: GpioRegisterAccessType::ReadOnly
};

/// Contains the properties of a GpioControl register
#[allow(dead_code)]
const GPIO_CTRL: GpioRegisterDefinition = GpioRegisterDefinition{
    offset: 0x04,
    bit_width: 32,
    access_type: GpioRegisterAccessType::ReadWrite
};

#[allow(dead_code)]
fn get_gpio_address(gpio_index: usize) -> Result<usize, &'static str>{
    if gpio_index >= GPIO_ADDRESSES.len()
    {
        return Err("Invalid GPIO index passed to write function. Values must be between 0 and 6.");
    }

    Ok(GPIO_ADDRESSES[gpio_index])
}

#[allow(dead_code)]
impl GpioRegisterDefinition{
    pub fn write(&self, gpio_index: usize, value: usize, bits: usize) -> Result<usize, &'static str>{

        if let GpioRegisterAccessType::ReadOnly = self.access_type{
            return Err("Attempted to write to ReadOnly GPIO register!");
        }

        if bits > self.bit_width {
            return Err("Attempted to write more bits to the GPIO register than the register can hold");
        }

        //Get the full address for this register
        let gpio_addr_off = match get_gpio_address(gpio_index) {
            Ok(addr) => addr,
            Err(error) => {return Err(error)},
        };

        let reg_addr = gpio_addr_off + self.offset;

        //Get the mask to cut off any additional bits
        let mask: usize = (1usize << bits) - 1usize;

        //Mask our data
        let masked_data = value & mask;
        
        //Get the number of bytes to write.
        let bytes_to_write = (bits / 8) + (if bits % 8 > 0 {1} else {0});
 

        //Finally, let's write our value
        //We only want to write our data up to the size of the register.
        unsafe{

            if bytes_to_write == 1 {
                core::ptr::write_volatile(reg_addr as *mut u8, masked_data as u8)
            } else if bytes_to_write == 2 {
                core::ptr::write_volatile(reg_addr as *mut u16, masked_data as u16)
            } else if bytes_to_write == 3 {
                core::ptr::write_volatile(reg_addr as *mut u32, masked_data as u32)
            } else if bytes_to_write == 4 {
                core::ptr::write_volatile(reg_addr as *mut u64, masked_data as u64)
            } else {
                return Err("Unexpected register size.");
            }
            
        }

        Ok(bytes_to_write)
    }

    pub fn read(&self, gpio_index: usize, bits: usize) -> Result<GpioRegReadResult, &'static str>
    {

        if let GpioRegisterAccessType::WriteOnly = self.access_type{
            return Err("Attempted to read from a write-only GPIO register");
        }

        if self.bit_width < bits{
            return Err("Attempted to read too many bytes from GPIO register");
        }

        let gpio_addr_off = match get_gpio_address(gpio_index){
            Ok(addr) => addr,
            Err(error) => {return Err(error)},
        };

        let reg_addr = gpio_addr_off + self.offset;

        let mask: usize = (1 << bits) - 1;

        let bytes_to_read = bits / 8 + (if bits % 8 == 0 {0} else {1});

        let read_value: usize;
        unsafe{
            let raw_value;

            if bytes_to_read == 1{
                raw_value = core::ptr::read_volatile(reg_addr as *const u8) as usize;
            } else if bytes_to_read == 2{
                raw_value = core::ptr::read_volatile(reg_addr as *const u16) as usize;
            } else if bytes_to_read == 4{
                raw_value = core::ptr::read_volatile(reg_addr as *const u32) as usize;
            } else if bytes_to_read == 8{
                raw_value = core::ptr::read_volatile(reg_addr as *const u64) as usize;
            } else{
                return Err("Unexpected Register Size");
            }
            
            read_value = raw_value & mask;
        }


        Ok(GpioRegReadResult{
            value: read_value, 
            bit_width: bits
        })
    }
}

#[allow(dead_code)]
struct GpioCtrlConfigBuilder{

}