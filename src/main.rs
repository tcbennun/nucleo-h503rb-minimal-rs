#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    Config,
};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    #[cfg(not(feature = "lsi"))]
    let config = {
        let mut cfg = Config::default();
        cfg.rcc.ls = embassy_stm32::rcc::LsConfig::off();
        cfg
    };

    #[cfg(feature = "lsi")]
    let config = Config::default();

    let p = embassy_stm32::init(config);

    let mut led = Output::new(p.PA5, Level::Low, Speed::Low);
    loop {
        led.toggle();
        for _ in 0..1000000 {
            cortex_m::asm::nop();
        }
    }
}
