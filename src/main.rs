#![no_std]
#![no_main]

mod lcd;
use crate::lcd::LCD;

extern crate panic_halt;
use arduino_uno::adc;
use arduino_uno::prelude::*;
use arduino_uno::pwm;
use arduino_uno::wdt;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut adc = adc::Adc::new(dp.ADC, Default::default());

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );

    let mut watchdog = wdt::Wdt::new(&dp.CPU.mcusr, dp.WDT);
    watchdog.disable();

    let mut timer2 = pwm::Timer2Pwm::new(dp.TC2, pwm::Prescaler::Prescale64);
    let mut output = pins.d11.into_output(&mut pins.ddr).into_pwm(&mut timer2);
    output.enable();

    let (vbg, gnd): (u16, u16) = (
        nb::block!(adc.read(&mut adc::channel::Vbg)).void_unwrap(),
        nb::block!(adc.read(&mut adc::channel::Gnd)).void_unwrap(),
    );

    let mut a0 = pins.a0.into_analog_input(&mut adc);
    let mut a1 = pins.a1.into_analog_input(&mut adc);
    let mut a2 = pins.a2.into_analog_input(&mut adc);

    let switch_state = false;
    let mut output_val: [u16; 4] = [1023, 1023, 1023, 1023];

    let steps = 3;
    let mut current_step = 0;
    let mut block_stepping = false;

    let mut delay = 60;

    let lcd_rw = pins.d12.into_output(&mut pins.ddr);
    let lcd_enable = pins.d10.into_output(&mut pins.ddr);

    let lcd_d0 = pins.d2.into_output(&mut pins.ddr);
    let lcd_d1 = pins.d3.into_output(&mut pins.ddr);
    let lcd_d2 = pins.d4.into_output(&mut pins.ddr);
    let lcd_d3 = pins.d5.into_output(&mut pins.ddr);

    ufmt::uwrite!(&mut serial, "Pre LCD\n\r").void_unwrap();
    let mut lcd = LCD::new(lcd_rw, lcd_enable, lcd_d0, lcd_d1, lcd_d2, lcd_d3);
    ufmt::uwrite!(&mut serial, "Called new\n\r").void_unwrap();
    lcd.begin(16, 2, None);
    lcd.print("test");
    ufmt::uwrite!(&mut serial, "Called begin\n\r").void_unwrap();

    loop {
        // watchdog.feed();

        let values: [u16; 3] = [
            nb::block!(adc.read(&mut a0)).void_unwrap(),
            nb::block!(adc.read(&mut a1)).void_unwrap(),
            nb::block!(adc.read(&mut a2)).void_unwrap(),
        ];

        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).void_unwrap();
        }
        ufmt::uwrite!(&mut serial, "Block Stepping: {}", block_stepping).void_unwrap();
        ufmt::uwrite!(&mut serial, "\n\r").void_unwrap();

        if values[0] >= 1000 {
            block_stepping = false;
            output_val[current_step] = values[1];
            delay = values[2]
        } else {
            block_stepping = true;
        }

        output.set_duty(output_val[current_step] as u8);

        if block_stepping == true {
            if current_step < steps {
                current_step = current_step + 1;
            } else {
                current_step = 0;
            }

            arduino_uno::delay_ms(delay as u16);
        }
        watchdog.feed();
    }
}
