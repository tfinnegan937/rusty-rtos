pub const UARK_CLK: usize = 48000000usize;

/// These are the base addresses for UARTS 0-5 on the RP1 SoC.
/// The UART controller is extremely similar to the PL1011
const UART_ADDRESSES: [usize; 6] = [
    0x1F00030000, // UART0
    0x1F00034000, // UART1
    0x1F00038000, // UART2
    0x1F0003C000, // UART3
    0x1F00040000, // UART4
    0x1F00044000  // UART5
];

enum UartRegisterAccessType{
    ReadOnly,
    WriteOnly,
    ReadWrite
}

/// The definition of a UART register as found in the PL011 technical reference manual.
struct UartRegisterDefinition{
    /// The offset from the UART base address at which this UART register can be found.
    offset: usize,
    /// The bit size of this register. This includes all data bits and control bits
    bit_width: usize,
    /// The number of bits that are used for actual data. If a register has a bit
    /// width of 12, but a data width of 8, only 8 bits of the full 12-bit size
    /// are used for R/W data. The remaining bits are used for flags or other control
    /// structures
    data_width: usize,
    /// The access type of the register, ReadOnly, WriteOnly, or ReadWrite
    access_type: UartRegisterAccessType
}


//Define the various UART registers.

/// Data Register
const UARTDR: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x000,
    bit_width: 12,
    data_width: 8,
    access_type: UartRegisterAccessType::ReadWrite
};

/// Receive Status Register, coincides with the Error Clear Register (UARTECR) at offset 0x004
const UARTRSR: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x004,
    bit_width: 4,
    data_width: 0,
    access_type: UartRegisterAccessType::ReadOnly
};

/// Error Clear Register, coincides with the Receive Status Register (UARTRSR) at offset 0x004
const UARTECR: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x004, 
    bit_width: 4,
    data_width: 0,
    access_type: UartRegisterAccessType::WriteOnly
};

/// Flag Register
const UARTFR: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x018,
    bit_width: 9,
    data_width: 9,
    access_type: UartRegisterAccessType::ReadOnly
};

/// IrDA Low-Power Counter Register
const UARTILPR: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x20,
    bit_width: 8,
    data_width: 8,
    access_type: UartRegisterAccessType::ReadWrite
};

/// Integer Baud Rate Register
const UARTIBRD: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x24,
    bit_width: 16,
    data_width: 16,
    access_type: UartRegisterAccessType::ReadWrite
};

/// Fractional Baud Rate Register
const UARTFBRD: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x28,
    bit_width: 6,
    data_width: 6,
    access_type: UartRegisterAccessType::ReadWrite
};

/// Line Control Register
const UARTLCR_H: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x02C,
    bit_width: 8,
    data_width: 8,
    access_type: UartRegisterAccessType::ReadWrite
};

/// UART Control Register
const UARTCR: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x30,
    bit_width: 16,
    data_width: 16,
    access_type: UartRegisterAccessType::ReadWrite
};

/// Interrupt FIFO Level Select Register
const UARTIFLS: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x34,
    bit_width: 6,
    data_width: 6,
    access_type: UartRegisterAccessType::ReadWrite
};

/// Interrupt Mask Set/Clear Register
const UARTIMSC: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x38,
    bit_width: 11,
    data_width: 11,
    access_type: UartRegisterAccessType::ReadWrite
};

/// Raw Interrupt Status Register
const UARTRIS: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x03C,
    bit_width: 11,
    data_width: 11,
    access_type: UartRegisterAccessType::ReadOnly
};

/// Masked Interrupt Status Register
const UARTMIS: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x040,
    bit_width: 11,
    data_width: 11,
    access_type: UartRegisterAccessType::ReadOnly
};

/// Interrupt Clear Register
const UARTICR: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x040,
    bit_width: 11,
    data_width: 11,
    access_type: UartRegisterAccessType::WriteOnly
};

/// DMA Control Register
const UARTDMACR: UartRegisterDefinition = UartRegisterDefinition{
    offset: 0x48,
    bit_width: 3,
    data_width: 3,
    access_type: UartRegisterAccessType::ReadWrite
};

pub struct UartRegReadResult{
    pub value: usize,
    pub bit_width: usize
}

fn get_uart_address(uart_index: usize) -> Result<usize, &'static str>{
    if uart_index >= UART_ADDRESSES.len()
    {
        return Err("Invalid UART index passed to write function. Values must be between 0 and 6.");
    }

    Ok(UART_ADDRESSES[uart_index])
}


impl UartRegisterDefinition{
    pub fn write(&self, uart_index: usize, value: usize, bits: usize) -> Result<usize, &'static str>{

        if let UartRegisterAccessType::ReadOnly = self.access_type{
            return Err("Attempted to write to ReadOnly UART register!");
        }

        if bits > self.bit_width {
            return Err("Attempted to write more bits to the UART register than the register can hold");
        }

        //Get the full address for this register
        let uart_addr_off = match get_uart_address(uart_index) {
            Ok(addr) => addr,
            Err(error) => {return Err(error)},
        };

        let reg_addr = uart_addr_off + self.offset;

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

    pub fn read(&self, uart_index: usize, bits: usize) -> Result<UartRegReadResult, &'static str>
    {

        if let UartRegisterAccessType::WriteOnly = self.access_type{
            return Err("Attempted to read from a write-only UART register");
        }

        if self.bit_width < bits{
            return Err("Attempted to read too many bytes from UART register");
        }

        let uart_addr_off = match get_uart_address(uart_index){
            Ok(addr) => addr,
            Err(error) => {return Err(error)},
        };

        let reg_addr = uart_addr_off + self.offset;

        let read_value: usize;
        let mask: usize = (1 << bits) - 1;

        let bytes_to_read = bits / 8 + (if bits % 8 == 0 {0} else {1});

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


        Ok(UartRegReadResult{
            value: read_value, 
            bit_width: bits
        })
    }
}

pub enum WordLength{
    Bits8,
    Bits7,
    Bits6,
    Bits5
}

pub enum ParitySelect{
    Even,
    Odd
}

pub enum StickParityEnableMode{
    Disabled,
    Enabled
}

pub enum FIFOEnableMode{
    Enabled,
    Disabled
}

pub enum ParityEnableMode{
    Disabled,
    Enabled(ParitySelect)
}


pub enum StopBitMode{
    OneStopBit,
    TwoStopBits
}

pub enum TransmitMode{
    TxOnly,
    RxOnly,
    Bidirectional
}

pub struct UartInstance{
    uart_index: usize
}

pub struct InstanceBuilder{
   pub(in crate::bsp::raspberry_pi_5::uart) uart_index: usize,
   pub(in crate::bsp::raspberry_pi_5::uart) baud_rate: usize,
   pub(in crate::bsp::raspberry_pi_5::uart) word_length: WordLength,
   pub(in crate::bsp::raspberry_pi_5::uart) fifo_enable_mode: FIFOEnableMode,
   pub(in crate::bsp::raspberry_pi_5::uart) parity_enable_mode: ParityEnableMode,
   pub(in crate::bsp::raspberry_pi_5::uart) stick_parity_enable_mode: StickParityEnableMode,
   pub(in crate::bsp::raspberry_pi_5::uart) stop_bit_mode: StopBitMode,
   pub(in crate::bsp::raspberry_pi_5::uart) transmit_mode: TransmitMode
}





impl InstanceBuilder{
    pub fn new(uart_index: usize) -> InstanceBuilder{
        InstanceBuilder{
            uart_index,
            ..InstanceBuilder::default()
        }
    }

    pub fn with_word_length(self, word_length: WordLength) -> InstanceBuilder{
        InstanceBuilder{
            word_length,
            ..self
        }
    }

    pub fn with_fifo(self) -> InstanceBuilder{
        InstanceBuilder{
            fifo_enable_mode: FIFOEnableMode::Enabled,
            ..self
        }
    }

    pub fn with_parity(self, parity_mode: ParitySelect) -> InstanceBuilder{
        InstanceBuilder{
            parity_enable_mode: ParityEnableMode::Enabled(parity_mode),
            ..self
        }
    }

    pub fn with_stick_parity(self) -> InstanceBuilder{
        InstanceBuilder{
            stick_parity_enable_mode: StickParityEnableMode::Enabled,
            ..self
        }
    }

    pub fn with_stop_bit_mode(self, stop_bit_mode: StopBitMode) -> InstanceBuilder{
        InstanceBuilder{
            stop_bit_mode,
            ..self
        }
    }

    pub fn with_baud_rate(self, baud_rate: usize) -> InstanceBuilder{
        InstanceBuilder{
            baud_rate,
            ..self
        }
    }

    pub fn with_transmit_mode(self, transmit_mode: TransmitMode) -> InstanceBuilder{
        InstanceBuilder{
            transmit_mode,
            ..self
        }
    }

    pub fn build(self) -> Result<UartInstance, &'static str> {

        UartInstance::new(self)
    }
    

    fn default() -> InstanceBuilder{
        InstanceBuilder{
            uart_index: UART_ADDRESSES.len() + 1, //By default, set it to the first invalid index. This prevents the default value from being useful, and forces a uart to be selected
            baud_rate: 115200,
            word_length: WordLength::Bits8,
            fifo_enable_mode: FIFOEnableMode::Disabled,
            parity_enable_mode: ParityEnableMode::Disabled,
            stick_parity_enable_mode: StickParityEnableMode::Disabled,
            stop_bit_mode: StopBitMode::OneStopBit,
            transmit_mode: TransmitMode::Bidirectional
        }
    }
}

pub struct Flags{
    /// Clear to Send
    cts: bool,
    /// Data Set Ready
    dsr: bool,
    /// Data Carrier Detect
    dcd: bool,
    /// Busy
    busy: bool,
    /// Receive FIFO Empty
    rxfe: bool,
    /// Transmit FIFO full
    txff: bool,
    // Receive FIFO Full
    rxff: bool,
    // Transmit FIFO Empty
    txfe: bool
}

impl Flags{
    pub(in crate::bsp::raspberry_pi_5::uart) fn read(uart_index: usize) -> Result<Flags, &'static str>{
        let flags = match UARTFR.read(uart_index, UARTFR.bit_width){
            Ok(result) => result.value,
            Err(error) => return Err(error),
        };

        //Now let's go down the list and get each flag

        let cts = (flags & (1usize)) > 0;
        let dsr = (flags & (1usize << 1usize)) > 0;
        let dcd = (flags & (1usize << 2usize)) > 0;
        let busy = (flags & (1usize << 3usize)) > 0;
        let rxfe = (flags & (1usize << 4usize)) > 0;
        let txff = (flags & (1usize << 5usize)) > 0;
        let rxff = (flags & (1usize << 6usize)) > 0;
        let txfe = (flags & (1usize << 7usize)) > 0;

        Ok(
            Flags{
                cts,
                dsr,
                dcd,
                busy,
                rxfe,
                txff,
                rxff,
                txfe
            }
        )

    }

    pub fn clear_to_send(&self) -> bool{
        self.cts
    }

    pub fn data_set_ready(&self) -> bool{
        self.dsr
    }

    pub fn data_carrier_detect(&self) -> bool{
        self.dcd
    }

    pub fn transmit_busy(&self) -> bool{
        self.busy
    }

    pub fn receive_fifo_empty(&self) -> bool{
        self.rxfe
    }

    pub fn transmit_fifo_empty(&self) -> bool{
        self.txfe
    }

    pub fn receive_fifo_full(&self) -> bool{
        self.rxff
    }

    pub fn transmit_fifo_full(&self) -> bool{
        self.txff
    }

}


impl UartInstance{

    fn disable_fifos(uart_index: usize) -> Result<(), &'static str>{

        //Get the current status of the line register
        let line_register_state = match UARTLCR_H.read(uart_index, UARTLCR_H.bit_width){
            Ok(result) => result,
            Err(error) => {return Err(error)},
        };

        //Disable bit 4, the FIFO enable bit.
        let new_line_register_state =  line_register_state.value & !(1usize << 4usize);

        match UARTLCR_H.write(uart_index, new_line_register_state, 8){ //Bits 8-15 are labeled "do not modify", so we only write the first 8 bits of our data.
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn busy_wait_last_transmit(uart_index: usize){
        let mut tx_busy = true;

        while tx_busy{
            //1. Read the flag register.
            let flags = UARTFR.read(uart_index, UARTFR.bit_width).expect("Failed to read from the UART flag register");


            //2. Check the BUSY flag.
            let busy_flag = (flags.value >> 3) & 1;

            //Continue looping if we're busy.
            tx_busy = busy_flag != 0;
        }

        return;
    }

    /// Disables the UART. It also prepares the UART for being reprogrammed by:
    /// 1. Disabling TX and RX
    /// 2. Flushing the FIFOs
    /// 3. Busy waiting for the last transmission to complete.
    fn disable_uart(uart_index: usize) -> Result<(), &'static str>{

        // Get the current value of the register
        let current_value = match UARTCR.read(uart_index, UARTCR.bit_width) {
            Ok(result) => result.value,
            Err(error) =>{
                return Err(error);
            } 
        };

        //Set bit 0 to zero in order to disable the uart.
        let mut disable_value = current_value & !(0x1 as usize);

        //We should also disable TX and RX on bits 8 and 9
        disable_value &= !(1usize << 8usize);
        disable_value &= !(1usize << 9usize);

        //Write the new value
        let _ = match UARTCR.write(uart_index, disable_value, UARTCR.bit_width){
            Ok(_) => {},
            Err(error) => {
                return Err(error)
            }
        };


        //Now that it's disabled, we need to busy wait until the last transmission is finished;
        Self::busy_wait_last_transmit(uart_index);

        //Finally, let's flush the FIFO buffers by disabling FIFOS.
        Self::disable_fifos(uart_index).expect("Failed to disable FIFOs for UART");

        Ok(())
    }

    fn configure_line_control(
        uart_index: usize,
        word_length: WordLength,
        fifo_enable_mode: FIFOEnableMode,
        parity_enable_mode: ParityEnableMode,
        stick_parity_enable_mode: StickParityEnableMode,
        stop_bit_mode: StopBitMode
    ) -> Result<(), &'static str>
    {

        //Initialize the buffer...
        let mut line_control_value: usize = 0x00;

        //Now, let's set each value bit by bit depending upon our mode.

        //We'll skip bit 0 - the BRK bit. This is for forcing a break in the transmission.

        //Bits 1 and 2 are the partity enable bit and the parity select bit
        match parity_enable_mode{
            ParityEnableMode::Disabled => {
                line_control_value &= !(1usize << 1);
                line_control_value &= !(1usize << 2);
            },
            ParityEnableMode::Enabled(select) => {
                line_control_value |= 1usize << 1;

                match select{
                    ParitySelect::Odd => {
                        line_control_value &= !(1usize << 2)
                    },
                    ParitySelect::Even => {
                        line_control_value |= 1usize << 2
                    }
                }
            },
        };

        //Bit 3 is the STP2 flag. If enabled, we send two stop bits.
        match stop_bit_mode{
            StopBitMode::OneStopBit => {
                line_control_value &= !(1usize << 3);
            },
            StopBitMode::TwoStopBits => {  
                line_control_value |= 1usize << 3;
            },
        }

        //Bit 4 is the FIFO enable. If set to 0, the holding register is one byte deep.
        match fifo_enable_mode{
            FIFOEnableMode::Enabled => {
                line_control_value |= 1usize << 4;
            },
            FIFOEnableMode::Disabled => {
                line_control_value &= !(1usize << 4);
            },
        }

        //Bits 5 and 6 control the word length
        match word_length{
            WordLength::Bits8 => {
                line_control_value |= 0b11usize << 5;
            },
            WordLength::Bits7 => {
                line_control_value |= 0b10usize << 5
            },
            WordLength::Bits6 => {
                line_control_value |= 0b01usize << 5
            },
            WordLength::Bits5 => {
                line_control_value |= 0b00usize << 5
            },
        }

        //Finally, bit 7 controls the stick parity
        match stick_parity_enable_mode{
            StickParityEnableMode::Disabled => {
                line_control_value &= !(1usize << 7)
            },
            StickParityEnableMode::Enabled => {
                line_control_value |= 1usize << 7
            },
        }

        //Once we've set all of these values, we can write to the line control register.
        match UARTLCR_H.write(uart_index, line_control_value, 8){ //Bits 8-15 are labeled "Reserved, do not modify". As such, we only write the first 8 bits (0-7)
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn enable_uart(uart_index: usize, transmit_mode: TransmitMode) -> Result<(), &'static str>{

        //First, read the current value of the control register.
        // Get the current value of the register
        let current_value = match UARTCR.read(uart_index, UARTCR.bit_width) {
            Ok(result) => result.value,
            Err(error) =>{
                return Err(error);
            } 
        };

        //First, check if the UART is already enabled
        //We do this by checking bit 1
        let is_enabled = (current_value & 0x1usize) > 0;  

        //If it is enabled, throw an error. We don't want to doubly enable the UART -- that may indicate that somebody else has enabled it as we were initializing.

        if is_enabled{
            return Err("The UART has already been enabled.");
        }

        //If it's not enabled, we're golden. Set TX and RX, and then enable the UART.
        let mut new_value = current_value;

        match transmit_mode{
            TransmitMode::TxOnly => {
                new_value |= 1usize << 8usize;
                new_value &= !(1usize << 9usize);
            },
            TransmitMode::RxOnly => {
                new_value &= !(1usize << 8usize);
                new_value |= 1usize << 9usize;
            },
            TransmitMode::Bidirectional => {
                new_value |= 1usize << 8usize;
                new_value |= 1usize << 9usize;
            },
        }

        new_value |= 0x1; //Then enable bit 0, which enables the UART.

        //And finally, write our changes.

        match UARTCR.write(uart_index, new_value, UARTCR.bit_width){
            Ok(_) => {},
            Err(error) => {return Err(error);},
        }

        //We should quickly verify that our changes were actually made.

        let read_value = UARTCR.read(uart_index, UARTCR.bit_width).expect("Failed to read UARTCR").value;

        if read_value == new_value{
            Ok(())
        }else {
            Err("The expected bits were not set in UARTCR")
        }

    }

    fn set_baud_rate(uart_index: usize, baud_rate: usize) -> Result<(), &'static str>{


        if baud_rate == 0{
            return Err("A baud rate of zero is invalid");
        }

        //First, we need to calculate our divisor
        let divisor: f64 = (UARK_CLK as f64) / (16 * baud_rate) as f64;
        let bdri: u16 = divisor as u16;
        let fractional_value: f64 = divisor - bdri as f64;

        let bdrf = ((fractional_value * 64.0) + 0.5) as u8;


        //We now have our bdrf and bdri values. Let's write them.
        match UARTIBRD.write(uart_index, bdri as usize, UARTIBRD.bit_width){
            Ok(_) => {},
            Err(error) => return Err(error),
        }

        match UARTFBRD.write(uart_index, bdrf as usize, UARTFBRD.bit_width){
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }
    }

    /// Create a new instance of the Uart for reading or writing
    pub(in crate::bsp::raspberry_pi_5) fn new(builder: InstanceBuilder) -> Result<UartInstance, &'static str>{

        //Disable the UART, which will flush the FIFO, wait for the last transmit (if we're still running)
        //and disable TX and RX
        match Self::disable_uart(builder.uart_index){
            Ok(_) => {},
            Err(error) => {
                panic!("Failed to disable the UART: {}", error);
            },
        };

        //Now we can validate the values stored within our builder, and then program the UART.

        //First, validate that we have a valid index.
        if builder.uart_index >= UART_ADDRESSES.len() {
            return Err("Invalid UART index");
        }

        //Second, we should program our line control register.
        match Self::configure_line_control(
            builder.uart_index, 
            builder.word_length, 
            builder.fifo_enable_mode, 
            builder.parity_enable_mode, 
            builder.stick_parity_enable_mode, 
            builder.stop_bit_mode
        ){
            Ok(_) => {},
            Err(error) => {
                return Err(error);
            }
        };

        //Third, we should set our baud rate.
        match Self::set_baud_rate(builder.uart_index, builder.baud_rate){
            Ok(_) => {},
            Err(error) => {return Err(error)}
        }

        //Finally, enable the UART
        match Self::enable_uart(builder.uart_index, builder.transmit_mode){
            Ok(_) => {},
            Err(error) => {return Err(error)}
        }

        Ok(UartInstance{
            uart_index: builder.uart_index
        })
    }



    pub fn flags(&self) -> Flags{
        Flags::read(self.uart_index).expect("Failed to read the UART's flags. This indicates an issue with the kernel.")
    }

    pub fn poll_write(&self, array: &[u8]) -> Result<usize, &'static str> {

        //TO-DO -- Verify that the  UART is in a valid state.

        for byte in array{
            let mut flags = self.flags();
            while flags.transmit_fifo_full(){
                flags = self.flags();
            }; //Block until the transmit buffer has space

            //Write to the fifo.
            let buffer = *byte as usize;

            match UARTDR.write(self.uart_index, buffer, 8){
                Ok(_) => {},
                Err(error) => return Err(error),
            }
        }

        Ok(array.len())

    }


    //Finally, we should enable our UART.
}