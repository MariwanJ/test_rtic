#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtic::app;
use rtic_monotonics::systick::prelude::*;
use rtt_target::{rprintln, rtt_init_print};
use stm32f7xx_hal::gpio::{Output, PushPull, PB0, PB7,PB14};
use stm32f7xx_hal::prelude::*;

use stm32f7xx_hal::{
    gpio::{ self, GpioExt, PullDown },
    pac,
    rcc::{ self, BusClock, HSEClock, HSEClockMode, RccExt, APB1, PLLP },
    rtc::{ self, Rtc, RtcClock },
    serial::{ self, Config, Serial },
};

systick_monotonic!(Mono, 1000);

#[app(device = stm32f7xx_hal::pac, peripherals = true)]
mod app {
    use rtic_monotonics::fugit::HertzU32;
    use stm32f7xx_hal::{ rcc::{HSEClockMode, PLLP}};

    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led1: PB0<Output<PushPull>>,
        led2: PB7<Output<PushPull>>,
        led3: PB14<Output<PushPull>>,
        state1: bool,
        state2: bool,
        state3: bool,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Setup clocks
        let mut flash = cx.device.FLASH;

        // Initialize the systick interrupt & obtain the token to prove that we did
        Mono::start(cx.core.SYST, 216_000_000); 

        rtt_init_print!();
        rprintln!("init");

    let mut rcc = cx.device.RCC.constrain();
    // Configure the clock based on STM32IDECubeMX
    let clocks = rcc.cfgr
        // .lse(lse_var)   lse in HAL is not implemented
        .hse(stm32f7xx_hal::rcc::HSEClock::new(fugit::HertzU32::MHz(8), HSEClockMode::Oscillator)) //8 MHz HSE crystal
        .sysclk(HertzU32::MHz(216)) // Set system clock to 216 MHz
        .use_pll()
        .pclk1(fugit::HertzU32::MHz(54))
        .pllq(9) // Set PLLQ
        .plln(216)
        .pllp(PLLP::Div2)
        .pllm(4)
        .freeze();
    
        // Setup LED
        let mut gpiob = cx.device.GPIOB.split();
        let mut led1 = gpiob.pb0.into_push_pull_output();
        let mut led2 = gpiob.pb7.into_push_pull_output();
        let mut led3 = gpiob.pb14.into_push_pull_output();
        led1.set_high();
        led2.set_high();
        led3.set_high();
        // Initialize states
        let state1 = false;
        let state2 = false;
        let state3 = false;

        // Schedule the blinking task
        blink1::spawn().ok();
        blink2::spawn().ok();
        blink3::spawn().ok();

       (Shared {}, Local { led1, led2, led3, state1, state2, state3 })
    }

    #[task(local = [led1, state1])]
    async fn blink1(cx: blink1::Context) {
        loop {
            rprintln!("blink1");
            if *cx.local.state1 {
                cx.local.led1.set_high();
                *cx.local.state1 = false;
            } else {
                cx.local.led1.set_low();
                *cx.local.state1 = true;
            }
            Mono::delay(1000.millis()).await;
        }
    }
    
    #[task(local = [led2, state2])]
    async fn blink2(cx: blink2::Context) {
        loop {
            rprintln!("blink2");
            if *cx.local.state2 {
                cx.local.led2.set_high();
                *cx.local.state2 = false;
            } else {
                cx.local.led2.set_low();
                *cx.local.state2 = true;
            }
            Mono::delay(500.millis()).await;
        }
    }
     #[task(local = [led3, state3])]
    async fn blink3(cx: blink3::Context) {
        loop {
            rprintln!("blink3");
            if *cx.local.state3 {
                cx.local.led3.set_high();
                *cx.local.state3 = false;
            } else {
                cx.local.led3.set_low();
                *cx.local.state3 = true;
            }
            Mono::delay(250.millis()).await;
        }
    }
}
