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

struct GpioRegisterDefinition{
    offset: usize,
    bit_width: usize
}

const GPIO_STATUS: GpioRegisterDefinition = GpioRegisterDefinition{
    offset: 0x00,
    bit_width: 32
};

const GPIO_CTRL: GpioRegisterDefinition = GpioRegisterDefinition{
    offset: 0x04,
    bit_width: 32
};

impl GpioRegisterDefinition{
    
}