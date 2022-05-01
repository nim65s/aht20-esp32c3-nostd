#![no_std]
#![no_main]

use core::fmt::Write;

use esp32c3_hal::{gpio::IO, i2c::*, pac::Peripherals, prelude::*, Delay, RtcCntl, Serial, Timer};
use nb::block;
use panic_halt as _;
use riscv_rt::entry;

use aht20::Aht20;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
    let mut serial0 = Serial::new(peripherals.UART0).unwrap();
    let mut timer0 = Timer::new(peripherals.TIMG0);
    let mut timer1 = Timer::new(peripherals.TIMG1);

    // Disable watchdog timers
    rtc_cntl.set_super_wdt_enable(false);
    rtc_cntl.set_wdt_enable(false);
    timer0.disable();
    timer1.disable();

    writeln!(serial0, "hello !").unwrap();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create a new peripheral object with the described wiring
    // and standard I2C clock speed
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio1,
        io.pins.gpio2,
        100_000,
        &mut peripherals.SYSTEM,
    )
    .unwrap();

    let delay = Delay::new(peripherals.SYSTIMER);
    match Aht20::new(i2c, delay) {
        Err(e) => {
            writeln!(serial0, "error: {:?}", e).unwrap();
            loop {
                block!(timer0.wait()).unwrap();
            }
        }
        Ok(mut dev) => {
            // Start timer (1 min interval)
            timer0.start(600_000_000u64);

            loop {
                let (h, t) = dev.read().unwrap();

                writeln!(
                    serial0,
                    "relative humidity={}%; temperature={}C",
                    h.rh(),
                    t.celsius()
                )
                .unwrap();

                // Wait 1 min
                block!(timer0.wait()).unwrap();
            }
        }
    };
}
