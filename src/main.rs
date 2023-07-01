#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_idf_hal::i2c::I2C0;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc};
use pwm_pca9685::{Address, Channel, Pca9685};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    println!("Hello world!");

    let address = Address::default();
    let mut pwm = Pca9685::new(I2C0, address).unwrap();

    pwm.set_prescale(100).unwrap();
    pwm.enable().unwrap();

    // Turn on channel 0 at 0.
    pwm.set_channel_on(Channel::C0, 0).unwrap();

    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`.
    pwm.set_channel_off(Channel::C0, 2047).unwrap();

    let _dev = pwm.destroy(); // Get the I2C device back

    println!("ran shit");

    loop {}
}
