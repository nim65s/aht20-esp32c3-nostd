#![no_std]
#![no_main]

use core::fmt::Write;

use esp32c3_hal::{
    clock::ClockControl, gpio::IO, i2c::*, peripherals::Peripherals, prelude::*, timer::TimerGroup,
    Delay, Rtc, Uart,
};
use nb::block;
use panic_halt as _;
use riscv_rt::entry;

use aht20::Aht20;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let mut uart0 = Uart::new(peripherals.UART0);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer0 = timer_group0.timer0;
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    writeln!(uart0, "hello !").unwrap();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create a new peripheral object with the described wiring
    // and standard I2C clock speed
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio1,
        io.pins.gpio2,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let delay = Delay::new(&clocks);
    match Aht20::new(i2c, delay) {
        Err(e) => {
            writeln!(uart0, "error: {:?}", e).unwrap();
            loop {
                block!(timer0.wait()).unwrap();
            }
        }
        Ok(mut dev) => {
            // Start timer (1 min interval)
            timer0.start(60u64.secs());

            loop {
                let (h, t) = dev.read().unwrap();

                writeln!(
                    uart0,
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
