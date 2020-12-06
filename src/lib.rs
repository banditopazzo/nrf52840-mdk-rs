#![no_std]

pub use nrf52840_hal as hal;
pub use nrf_softdevice;
pub use nrf_softdevice::pac;

use core::sync::atomic::{AtomicUsize, Ordering};
use hal::gpio::{p0, p1, Disconnected};

macro_rules! define_pins {
    ($(#[$topattr:meta])* struct $Type:ident,
    p0: {
     $( $(#[$attr:meta])* pin $name:ident = $pin_ident:ident : $pin_type:ident),+ ,
    },
    p1: {
     $( $(#[$attr1:meta])* pin $name1:ident = $pin_ident1:ident: $pin_type1:ident),+ ,
    }) => {

$(#[$topattr])*
pub struct $Type {
    $($(#[$attr])* pub $name: p0:: $pin_type <Disconnected>,)+
    $($(#[$attr1])* pub $name1: p1:: $pin_type1 <Disconnected>,)+
}

impl $Type {
    /// Returns the pins for the device
    pub fn new(pins0: p0::Parts, pins1: p1::Parts) -> Self {
        $Type {
            $($name: pins0.$pin_ident, )+
            $($name1: pins1.$pin_ident1, )+
        }
    }
}
}}

define_pins!(
    /// Maps the pins to the names printed on the device
    struct Pins,
    p0: {
        /// Uart RXD
        pin rxd = p0_19: P0_19,
        /// Uart TXD
        pin txd = p0_20: P0_20,

        pin p6 = p0_06: P0_06,
        pin p7 = p0_07: P0_07,
        pin p8 = p0_08: P0_08,
        pin p11 = p0_11: P0_11,
        pin p12 = p0_12: P0_12,
        pin p13 = p0_13: P0_13,
        pin p14 = p0_14: P0_14,
        pin p15 = p0_15: P0_15,
        pin p16 = p0_16: P0_16,
        pin p17 = p0_17: P0_17,
        pin p21 = p0_21: P0_21,
        pin p25 = p0_25: P0_25,
        pin p26 = p0_26: P0_26,
        pin p27 = p0_27: P0_27,


        pin ain0 = p0_02: P0_02,
        pin ain1 = p0_03: P0_03,
        pin ain2 = p0_04: P0_04,
        pin ain3 = p0_05: P0_05,
        pin ain4 = p0_28: P0_28,
        pin ain5 = p0_29: P0_29,
        pin ain6 = p0_30: P0_30,
        pin ain7 = p0_31: P0_31,

        pin nfc1 = p0_09: P0_09,
        pin nfc2 = p0_10: P0_10,

        pin red_led = p0_23: P0_23,
        pin green_led = p0_22: P0_22,
        pin blue_led = p0_24: P0_24,
    },
    p1: {
        pin button = p1_00: P1_00,

        /// ~RESET line to the QSPI flash
        pin qspi_reset = p1_01: P1_01,
        /// ~WP Write protect pin on the QSPI flash.
        pin qspi_wp = p1_02: P1_02,
        /// SPI SCLK for QSPI flash
        pin qspi_sclk = p1_03: P1_03,
        /// SPI MISO for QSPI flash
        pin qspi_miso = p1_04: P1_04,
        /// SPI MOSI for QSPI flash
        pin qspi_mosi = p1_05: P1_05,
        /// ~CS for the QSPI flash
        pin qspi_cs = p1_06: P1_06,
    }
);

// Take peripherals, split by softdevice and application
pub fn take_peripherals() -> (nrf_softdevice::Peripherals, Peripherals) {
    let p = pac::Peripherals::take().unwrap();

    (
        nrf_softdevice::Peripherals {
            AAR: p.AAR,
            ACL: p.ACL,
            CCM: p.CCM,
            CLOCK: p.CLOCK,
            ECB: p.ECB,
            EGU1: p.EGU1,
            EGU2: p.EGU2,
            EGU5: p.EGU5,
            MWU: p.MWU,
            NVMC: p.NVMC,
            POWER: p.POWER,
            RADIO: p.RADIO,
            RNG: p.RNG,
            RTC0: p.RTC0,
            SWI1: p.SWI1,
            SWI2: p.SWI2,
            SWI5: p.SWI5,
            TEMP: p.TEMP,
            TIMER0: p.TIMER0,
        },
        Peripherals {
            CC_HOST_RGF: p.CC_HOST_RGF,
            COMP: p.COMP,
            CRYPTOCELL: p.CRYPTOCELL,
            EGU0: p.EGU0,
            EGU3: p.EGU3,
            EGU4: p.EGU4,
            FICR: p.FICR,
            GPIOTE: p.GPIOTE,
            I2S: p.I2S,
            LPCOMP: p.LPCOMP,
            NFCT: p.NFCT,
            P0: p.P0,
            P1: p.P1,
            PDM: p.PDM,
            PPI: p.PPI,
            PWM0: p.PWM0,
            PWM1: p.PWM1,
            PWM2: p.PWM2,
            PWM3: p.PWM3,
            QDEC: p.QDEC,
            QSPI: p.QSPI,
            RTC2: p.RTC2,
            SAADC: p.SAADC,
            SPI0: p.SPI0,
            SPI1: p.SPI1,
            SPI2: p.SPI2,
            SPIM0: p.SPIM0,
            SPIM1: p.SPIM1,
            SPIM2: p.SPIM2,
            SPIM3: p.SPIM3,
            SPIS0: p.SPIS0,
            SPIS1: p.SPIS1,
            SPIS2: p.SPIS2,
            SWI0: p.SWI0,
            SWI3: p.SWI3,
            SWI4: p.SWI4,
            TIMER1: p.TIMER1,
            TIMER2: p.TIMER2,
            TIMER3: p.TIMER3,
            TIMER4: p.TIMER4,
            TWI0: p.TWI0,
            TWI1: p.TWI1,
            TWIM0: p.TWIM0,
            TWIM1: p.TWIM1,
            TWIS0: p.TWIS0,
            TWIS1: p.TWIS1,
            UART0: p.UART0,
            UARTE0: p.UARTE0,
            UARTE1: p.UARTE1,
            UICR: p.UICR,
            USBD: p.USBD,
            WDT: p.WDT,
        },
    )
}

#[allow(non_snake_case)]
pub struct Peripherals {
    pub CC_HOST_RGF: pac::CC_HOST_RGF,
    pub COMP: pac::COMP,
    pub CRYPTOCELL: pac::CRYPTOCELL,
    pub EGU0: pac::EGU0,
    pub EGU3: pac::EGU3,
    pub EGU4: pac::EGU4,
    pub FICR: pac::FICR,
    pub GPIOTE: pac::GPIOTE,
    pub I2S: pac::I2S,
    pub LPCOMP: pac::LPCOMP,
    pub NFCT: pac::NFCT,
    pub P0: pac::P0,
    pub P1: pac::P1,
    pub PDM: pac::PDM,
    pub PPI: pac::PPI,
    pub PWM0: pac::PWM0,
    pub PWM1: pac::PWM1,
    pub PWM2: pac::PWM2,
    pub PWM3: pac::PWM3,
    pub QDEC: pac::QDEC,
    pub QSPI: pac::QSPI,
    pub RTC2: pac::RTC2,
    pub SAADC: pac::SAADC,
    pub SPI0: pac::SPI0,
    pub SPI1: pac::SPI1,
    pub SPI2: pac::SPI2,
    pub SPIM0: pac::SPIM0,
    pub SPIM1: pac::SPIM1,
    pub SPIM2: pac::SPIM2,
    pub SPIM3: pac::SPIM3,
    pub SPIS0: pac::SPIS0,
    pub SPIS1: pac::SPIS1,
    pub SPIS2: pac::SPIS2,
    pub SWI0: pac::SWI0,
    pub SWI3: pac::SWI3,
    pub SWI4: pac::SWI4,
    pub TIMER1: pac::TIMER1,
    pub TIMER2: pac::TIMER2,
    pub TIMER3: pac::TIMER3,
    pub TIMER4: pac::TIMER4,
    pub TWI0: pac::TWI0,
    pub TWI1: pac::TWI1,
    pub TWIM0: pac::TWIM0,
    pub TWIM1: pac::TWIM1,
    pub TWIS0: pac::TWIS0,
    pub TWIS1: pac::TWIS1,
    pub UART0: pac::UART0,
    pub UARTE0: pac::UARTE0,
    pub UARTE1: pac::UARTE1,
    pub UICR: pac::UICR,
    pub USBD: pac::USBD,
    pub WDT: pac::WDT,
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}
