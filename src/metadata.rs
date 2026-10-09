pub mod ir {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct IR {
        pub blocks: &'static [Block],
        pub fieldsets: &'static [FieldSet],
        pub enums: &'static [Enum],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Block {
        pub name: &'static str,
        pub extends: Option<&'static str>,

        pub description: Option<&'static str>,
        pub items: &'static [BlockItem],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct BlockItem {
        pub name: &'static str,
        pub description: Option<&'static str>,

        pub array: Option<Array>,
        pub byte_offset: u32,

        pub inner: BlockItemInner,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum BlockItemInner {
        Block(BlockItemBlock),
        Register(Register),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Register {
        pub access: Access,
        pub bit_size: u32,
        pub fieldset: Option<&'static str>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct BlockItemBlock {
        pub block: &'static str,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Access {
        ReadWrite,
        Read,
        Write,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct FieldSet {
        pub name: &'static str,
        pub extends: Option<&'static str>,

        pub description: Option<&'static str>,
        pub bit_size: u32,
        pub fields: &'static [Field],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Field {
        pub name: &'static str,
        pub description: Option<&'static str>,

        pub bit_offset: BitOffset,
        pub bit_size: u32,
        pub array: Option<Array>,
        pub enumm: Option<&'static str>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Array {
        Regular(RegularArray),
        Cursed(CursedArray),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RegularArray {
        pub len: u32,
        pub stride: u32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct CursedArray {
        pub offsets: &'static [u32],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum BitOffset {
        Regular(RegularBitOffset),
        Cursed(CursedBitOffset),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RegularBitOffset {
        pub offset: u32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct CursedBitOffset {
        pub ranges: &'static [core::ops::RangeInclusive<u32>],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Enum {
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub bit_size: u32,
        pub variants: &'static [EnumVariant],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct EnumVariant {
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub value: u64,
    }
}

pub use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole, Mode};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Metadata {
    pub name: &'static str,
    pub family: &'static str,
    pub line: &'static str,
    pub memory: &'static [MemoryRegion],
    pub nv_structs: &'static [NvStructBinding],
    pub memory_options: &'static [MemoryOption],
    pub default_memory_option: &'static str,
    pub peripherals: &'static [Peripheral],
    // pub nvic_priority_bits: Option<u8>,
    pub interrupts: &'static [Interrupt],
    pub dma_channels: &'static [DmaChannel],
}

/// Binds `NvStruct` descriptors to a memory region by name. Split from
/// `MemoryRegion` so the always-available `mem_layout` module stays free of
/// `&'static ir::IR` references.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NvStructBinding {
    pub region: &'static str,
    pub structs: &'static [NvStruct],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemoryOption {
    pub name: &'static str,
    pub region_sizes: &'static [(&'static str, u32)],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NvStruct {
    pub name: &'static str,
    pub offset: u32,
    pub kind: &'static str,
    pub version: &'static str,
    pub block: &'static str,
    pub defaults: &'static [(&'static str, u32)],
    pub ir: &'static ir::IR,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Interrupt {
    pub name: &'static str,
    pub number: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Package {
    pub name: &'static str,
    pub package: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Peripheral {
    pub name: &'static str,
    pub address: u64,
    pub registers: Option<PeripheralRegisters>,
    pub rcc: Option<PeripheralRcc>,
    pub remap: Option<PeripheralRemapRegister>,
    pub pins: &'static [PeripheralPin],
    pub dma_channels: &'static [PeripheralDmaChannel],
    pub interrupts: &'static [PeripheralInterrupt],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRegisters {
    pub kind: &'static str,
    pub version: &'static str,
    pub block: &'static str,
    pub ir: &'static ir::IR,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralInterrupt {
    pub signal: &'static str,
    pub interrupt: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRcc {
    pub bus_clock: &'static str,
    pub kernel_clock: PeripheralRccKernelClock,
    pub enable: Option<PeripheralRccRegister>,
    pub reset: Option<PeripheralRccRegister>,
    pub stop_mode: StopMode,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRccRegister {
    pub register: &'static str,
    pub field: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRemapRegister {
    pub register: &'static str,
    pub field: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PeripheralRccKernelClock {
    Clock(&'static str),
    Mux(PeripheralRccRegister),
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum StopMode {
    #[default]
    Stop1, // Peripheral prevents chip from entering Stop1
    Stop2,   // Peripheral prevents chip from entering Stop2
    Standby, // Peripheral does not prevent chip from entering Stop
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralPin {
    pub pin: &'static str,
    pub signal: &'static str,
    /// Central PCFR remap group (V0/V1/V2/V3/X0/L1/641/643). Mutually exclusive with `af`.
    pub remap: Option<u8>,
    /// Per-pin AF number for AFR-style mux (CH32H4). Mutually exclusive with `remap`.
    pub af: Option<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DmaChannel {
    pub name: &'static str,
    pub dma: &'static str,
    pub channel: u32,
    pub dmamux: Option<&'static str>,
    pub dmamux_channel: Option<u32>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralDmaChannel {
    pub signal: &'static str,
    pub channel: Option<&'static str>,
    pub dmamux: Option<&'static str>,
    pub dma: Option<&'static str>,
    pub request: Option<u32>,
}
/// All chip/core identifiers included in this generated PAC, sorted and deduplicated.
pub const ALL_CHIPS: &[&str] = &[
    "CH32F103C6T6",
    "CH32F103C8T6",
    "CH32F103R8T6",
    "CH32H415REU6-qingke-v3f",
    "CH32H415REU6-qingke-v5f",
    "CH32H416RDU6-qingke-v3f",
    "CH32H416RDU6-qingke-v5f",
    "CH32H417MEU6-qingke-v3f",
    "CH32H417MEU6-qingke-v5f",
    "CH32H417QEU6-qingke-v3f",
    "CH32H417QEU6-qingke-v5f",
    "CH32H417WEU6-qingke-v3f",
    "CH32H417WEU6-qingke-v5f",
    "CH32L103C8T6",
    "CH32L103F7P6",
    "CH32L103F8P6",
    "CH32L103F8U6",
    "CH32L103G8R6",
    "CH32L103K8U6",
    "CH32M007x8x6",
    "CH32V002x4x6",
    "CH32V003A4M6",
    "CH32V003F4P6",
    "CH32V003F4U6",
    "CH32V003J4M6",
    "CH32V003x4x6",
    "CH32V004x6x1",
    "CH32V005x6x6",
    "CH32V006x8x6",
    "CH32V007x8x6",
    "CH32V103C6T6",
    "CH32V103C8T6",
    "CH32V103C8U6",
    "CH32V103R8T6",
    "CH32V203C6T6",
    "CH32V203C8T6",
    "CH32V203C8U6",
    "CH32V203F6P6",
    "CH32V203F8P6",
    "CH32V203F8U6",
    "CH32V203G6U6",
    "CH32V203G8R6",
    "CH32V203K6T6",
    "CH32V203K8T6",
    "CH32V203RBT6",
    "CH32V208CBU6",
    "CH32V208GBU6",
    "CH32V208RBT6",
    "CH32V208WBU6",
    "CH32V303CBT6",
    "CH32V303RBT6",
    "CH32V303RCT6",
    "CH32V303VCT6",
    "CH32V305FBP6",
    "CH32V305GBU6",
    "CH32V305RBT6",
    "CH32V307RCT6",
    "CH32V307VCT6",
    "CH32V307WCU6",
    "CH32V317VCT6",
    "CH32V317WCU6",
    "CH32X033F8P6",
    "CH32X034F8P6",
    "CH32X034F8U6",
    "CH32X035C8T6",
    "CH32X035F7P6",
    "CH32X035F8U6",
    "CH32X035G8R6",
    "CH32X035G8U6",
    "CH32X035R8T6",
    "CH641",
    "CH643",
];
/// All peripheral instances with registers, across the generated chips.
pub const ALL_PERIPHERAL_NAMES: &[&str] = &[
    "ADC",
    "ADC1",
    "ADC2",
    "AFIO",
    "AWU",
    "BLE_AES",
    "BLE_BB",
    "BLE_LLE",
    "BLE_RFEND",
    "CAN1",
    "CAN2",
    "CAN3",
    "CRC",
    "DAC",
    "DAC1",
    "DFSDM",
    "DMA1",
    "DMA2",
    "DMAMUX",
    "DVP",
    "ESIG",
    "ETH",
    "EXTEN",
    "EXTEND",
    "EXTI",
    "FLASH",
    "GPIOA",
    "GPIOB",
    "GPIOC",
    "GPIOD",
    "GPIOE",
    "GPIOF",
    "HSADC",
    "HSEM",
    "I2C1",
    "I2C2",
    "I2C3",
    "I2C4",
    "IPC",
    "IWDG",
    "LPTIM1",
    "LPTIM2",
    "OPA",
    "OTG_FS",
    "PFIC",
    "PIOC",
    "PWR",
    "RCC",
    "RNG",
    "RTC",
    "SDIO",
    "SPI1",
    "SPI2",
    "SPI3",
    "SPI4",
    "SYSTICK",
    "TIM1",
    "TIM10",
    "TIM11",
    "TIM12",
    "TIM2",
    "TIM3",
    "TIM4",
    "TIM5",
    "TIM6",
    "TIM7",
    "TIM8",
    "TIM9",
    "TKEY",
    "USART1",
    "USART2",
    "USART3",
    "USART4",
    "USART5",
    "USART6",
    "USART7",
    "USART8",
    "USBD",
    "USBFS",
    "USBHS",
    "USBPD",
    "USBRAM",
    "USBSS",
    "WWDG",
];
/// All peripheral kinds and their versions, across the generated chips.
pub const ALL_PERIPHERAL_VERSIONS: &[(&str, &[&str])] = &[
    (
        "adc",
        &["ch641", "h4", "l1", "v003", "v00x", "v1", "v3", "x0"],
    ),
    ("afio", &["ch641", "h4", "l1", "v003", "v00x", "v3", "x0"]),
    ("awu", &["x0"]),
    ("bleaes", &["v208"]),
    ("blebb", &["v208"]),
    ("blelle", &["v208"]),
    ("blerfend", &["v208"]),
    ("can", &["h4", "l1", "v3"]),
    ("crc", &["h4"]),
    ("dac", &["h4", "v3"]),
    ("dfsdm", &["h4"]),
    ("dma", &["h4", "v1"]),
    ("dmamux", &["h4"]),
    ("dvp", &["v3"]),
    ("emac", &["v1"]),
    ("esig", &["h4"]),
    ("eth", &["10m"]),
    ("exten", &["v00x"]),
    ("extend", &["ch641", "l1", "v0", "v1", "v3"]),
    ("exti", &["common"]),
    ("flash", &["h4", "l1", "v0", "v00x", "v1", "v3", "x0"]),
    ("gpio", &["v0", "v3", "x0"]),
    ("hsadc", &["h4"]),
    ("hsem", &["h4"]),
    ("i2c", &["v0", "v3"]),
    ("ipc", &["h4"]),
    ("iwdg", &["h4", "v00x"]),
    ("lptim", &["l1"]),
    ("opa", &["h4", "l1", "v00x", "v3", "x0"]),
    ("otg", &["v2"]),
    ("pfic", &["h4", "rv2", "rv3", "rv4"]),
    ("pioc", &["x0"]),
    ("pwr", &["h4", "v00x"]),
    (
        "rcc",
        &[
            "ch641", "h4", "l1", "v003", "v00x", "v1", "v3", "v3_d8c", "x0",
        ],
    ),
    ("rng", &["h4", "v3"]),
    ("rtc", &["common"]),
    ("sdio", &["v3"]),
    ("sltm", &["v00x"]),
    ("spi", &["h4", "v0", "v3"]),
    ("systick", &["rv2", "rv3", "rv4", "v3f_v5f"]),
    ("timer", &["h4", "v3", "x0"]),
    ("tkey", &["v00x"]),
    ("usart", &["common", "h4"]),
    ("usb", &["l1fs", "v1fs", "v2fs", "x0fs"]),
    ("usbd", &["v2"]),
    ("usbfs", &["h4"]),
    ("usbhs", &["h4", "v3"]),
    ("usbpd", &["ch641", "h4", "l1", "x0"]),
    ("usbram", &["16x1_512"]),
    ("usbss", &["h4"]),
    ("wwdg", &["common", "v00x"]),
];
