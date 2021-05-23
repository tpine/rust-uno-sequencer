use arduino_uno::prelude::*;

use arduino_uno::hal::port::mode::Output;

use arduino_uno::hal::port::portb::PB2;
use arduino_uno::hal::port::portb::PB4;

use arduino_uno::hal::port::portd::PD2;
use arduino_uno::hal::port::portd::PD3;
use arduino_uno::hal::port::portd::PD4;
use arduino_uno::hal::port::portd::PD5;

// commands
static LCD_CLEARDISPLAY: u8 = 0x01;
static LCD_RETURNHOME: u8 = 0x02;
static LCD_ENTRYMODESET: u8 = 0x04;
static LCD_DISPLAYCONTROL: u8 = 0x08;
static LCD_CURSORSHIFT: u8 = 0x10;
static LCD_FUNCTIONSET: u8 = 0x20;
static LCD_SETCGRAMADDR: u8 = 0x40;
static LCD_SETDDRAMADDR: u8 = 0x80;

// flags for display entry mode
static LCD_ENTRYRIGHT: u8 = 0x00;
static LCD_ENTRYLEFT: u8 = 0x02;
static LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
static LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

// flags for display on/off control
static LCD_DISPLAYON: u8 = 0x04;
static LCD_DISPLAYOFF: u8 = 0x00;
static LCD_CURSORON: u8 = 0x02;
static LCD_CURSOROFF: u8 = 0x00;
static LCD_BLINKON: u8 = 0x01;
static LCD_BLINKOFF: u8 = 0x00;

// Function Set Flags
static LCD_8BITMODE: u8 = 0x10;
static LCD_4BITMODE: u8 = 0x00;
static LCD_2LINE: u8 = 0x08;
static LCD_1LINE: u8 = 0x00;
static LCD_5x10DOTS: u8 = 0x04;
static LCD_5x8DOTS: u8 = 0x00;

static HIGH: u8 = 1;
static LOW: u8 = 0;

pub struct LCD {
    rs: PB4<Output>,
    rw: u8,
    enable: PB2<Output>,
    d0: PD2<Output>,
    d1: PD3<Output>,
    d2: PD4<Output>,
    d3: PD5<Output>,
    d4: u8,
    d5: u8,
    d6: u8,
    d7: u8,

    display_function: u8,
    display_control: u8,
    display_mode: u8,
    num_lines: u8,

    row0: u8,
    row1: u8,
    row2: u8,
    row3: u8,
}

impl LCD {
    pub fn new(
        rs: PB4<Output>,
        enable: PB2<Output>,
        d0: PD2<Output>,
        d1: PD3<Output>,
        d2: PD4<Output>,
        d3: PD5<Output>,
    ) -> LCD {
        LCD {
            rs: rs,
            rw: 255,
            enable: enable,

            d0: d0,
            d1: d1,
            d2: d2,
            d3: d3,
            d4: 0,
            d5: 0,
            d6: 0,
            d7: 0,

            display_function: LCD_4BITMODE | LCD_1LINE | LCD_5x8DOTS,
            display_control: 0,
            display_mode: 0,
            num_lines: 1,

            row0: 0x00,
            row1: 0x40,
            row2: 0x00,
            row3: 0x40,
        }
    }
    pub fn begin(&mut self, cols: u8, lines: u8, dotsize: Option<u8>) {
        if (lines > 1) {
            self.display_function |= LCD_2LINE;
        }
        self.num_lines = lines;
        self.set_row_offset(0x00, 0x40, 0x00 + cols, 0x40 + cols);

        // // for some 1 line displays you can select a 10 pixel high font
        // if ((dotsize != LCD_5x8DOTS) && (lines == 1)) {
        //   _displayfunction |= LCD_5x10DOTS;
        // }
        // pinMode(_rs_pin, OUTPUT);
        // // we can save 1 pin by not using RW. Indicate by passing 255 instead of pin#
        // if (_rw_pin != 255) {
        //   pinMode(_rw_pin, OUTPUT);
        // }
        // pinMode(_enable_pin, OUTPUT);

        // // Do these once, instead of every time a character is drawn for speed reasons.
        // for (int i=0; i<((_displayfunction & LCD_8BITMODE) ? 8 : 4); ++i)
        // {
        //   pinMode(_data_pins[i], OUTPUT);
        //  }

        // // SEE PAGE 45/46 FOR INITIALIZATION SPECIFICATION!
        // // according to datasheet, we need at least 40 ms after power rises above 2.7 V
        // // before sending commands. Arduino can turn on way before 4.5 V so we'll wait 50
        // delayMicroseconds(50000);
        arduino_uno::delay_us(50000 as u16);
        // // // Now we pull both RS and R/W low to begin commands
        // // digitalWrite(_rs_pin, LOW);
        self.rs.set_low();
        // // digitalWrite(_enable_pin, LOW);
        self.enable.set_low();
        // if (_rw_pin != 255) {
        //   digitalWrite(_rw_pin, LOW);
        // }

        // //put the LCD into 4 bit or 8 bit mode
        // if (! (_displayfunction & LCD_8BITMODE)) {
        //   // this is according to the Hitachi HD44780 datasheet
        //   // figure 24, pg 46
        if (!((self.display_function & LCD_8BITMODE) == 1)) {
            //   // we start in 8bit mode, try to set 4 bit mode
            //   write4bits(0x03);
            //   delayMicroseconds(4500); // wait min 4.1ms
            self.write_4_bits(0x03);
            arduino_uno::delay_us(4500 as u16);

            //   // second try
            //   write4bits(0x03);
            //   delayMicroseconds(4500); // wait min 4.1ms
            self.write_4_bits(0x03);
            arduino_uno::delay_us(4500 as u16);

            //   // third go!
            //   write4bits(0x03);
            //   delayMicroseconds(150);
            self.write_4_bits(0x03);
            arduino_uno::delay_us(150 as u16);

            //   // finally, set to 4-bit interface
            //   write4bits(0x02);
            self.write_4_bits(0x02);
            arduino_uno::delay_us(150 as u16);
        }
        // } else {
        //   // this is according to the Hitachi HD44780 datasheet
        //   // page 45 figure 23

        //   // Send function set command sequence
        //   command(LCD_FUNCTIONSET | _displayfunction);
        //   delayMicroseconds(4500);  // wait more than 4.1 ms

        //   // second try
        //   command(LCD_FUNCTIONSET | _displayfunction);
        //   delayMicroseconds(150);

        //   // third go
        //   command(LCD_FUNCTIONSET | _displayfunction);
        // }

        // // finally, set # lines, font size, etc.
        // command(LCD_FUNCTIONSET | _displayfunction);
        self.command(LCD_FUNCTIONSET | self.display_function);
        // // turn the display on with no cursor or blinking default
        // _displaycontrol = LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF;
        self.display_control = LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF;
        // display();
        self.display();
        // // clear it off
        self.clear();
        // clear();

        // // Initialize to default text direction (for romance languages)
        // _displaymode = LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT;
        self.display_mode = LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT;
        // // set the entry mode
        // command(LCD_ENTRYMODESET | _displaymode);
        self.command(LCD_ENTRYMODESET | self.display_mode);
    }

    pub fn print(&mut self, line: &str) {
        for letter in line.as_bytes() {
            self.write(*letter);
        }
    }

    fn set_row_offset(&mut self, row0: u8, row1: u8, row2: u8, row3: u8) {
        self.row0 = row0;
        self.row1 = row1;
        self.row2 = row2;
        self.row3 = row3;
    }

    fn clear(&mut self) {
        self.command(LCD_CLEARDISPLAY);
        arduino_uno::delay_us(2000 as u16);
    }
    fn display(&mut self) {
        self.display_control |= LCD_DISPLAYON;
        self.command(LCD_DISPLAYCONTROL | self.display_control);
    }

    fn command(&mut self, value: u8) {
        self.send(value, LOW);
    }
    fn write(&mut self, value: u8) {
        self.send(value, HIGH)
    }
    fn send(&mut self, value: u8, mode: u8) {
        if (mode == LOW) {
            self.rs.set_low();
        } else {
            self.rs.set_high();
        }

        self.write_4_bits(value >> 4);
        self.write_4_bits(value);
    }

    fn pulse_enable(&mut self) {
        self.enable.set_low();
        arduino_uno::delay_us(1 as u16);
        self.enable.set_high();
        arduino_uno::delay_us(1 as u16);
        self.enable.set_low();
        arduino_uno::delay_us(100 as u16);
    }

    fn write_4_bits(&mut self, value: u8) {
        if ((value >> 0) & 0x01) == HIGH {
            self.d0.set_high();
        } else {
            self.d0.set_low();
        }
        if ((value >> 1) & 0x01) == HIGH {
            self.d1.set_high();
        } else {
            self.d1.set_low();
        }
        if ((value >> 2) & 0x01) == HIGH {
            self.d2.set_high();
        } else {
            self.d2.set_low();
        }
        if ((value >> 3) & 0x01) == HIGH {
            self.d3.set_high();
        } else {
            self.d3.set_low();
        }

        self.pulse_enable();
    }
}
