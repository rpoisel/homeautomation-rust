#![crate_name = "roof"]

mod automation;
mod bitop;
use crate::automation::*;
use crate::bitop::*;

extern crate docopt;
extern crate i2cdev;

use i2cdev::core::{I2CMessage, I2CTransfer};
use i2cdev::linux::{LinuxI2CBus, LinuxI2CMessage};

use docopt::Docopt;
use std::env::args;

use std::{thread, time};

const USAGE: &str = "
This executable contains the logic of the roof floor.

Usage:
  roof <device>
  roof (-h | --help)
  roof --version

Options:
  -h --help    Show this help text.
  --version    Show version.
";

// root@raspberry-d:~# i2cdetect -y 1
//      0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
// 00:                         -- -- -- -- -- -- -- --
// 10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 20: 20 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 30: -- -- -- -- -- -- -- -- 38 39 3a 3b -- -- -- --
// 40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
// 70: -- -- -- -- -- -- -- --

// address of the MAX7311 chip (outputs)
const _ADDR_MAX7311: u16 = 0x20;
const ADDR_PCF8574: u16 = 0x38;

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.argv(args()).parse())
        .unwrap_or_else(|e| e.exit());
    let path = args.get_str("<device>");
    let mut bus = match LinuxI2CBus::new(path) {
        Ok(bus) => bus,
        Err(_e) => {
            println!("Error opening I2C Bus {} {}", path, _e);
            return;
        }
    };
    println!("Opened I2C Bus OK: {}", path);

    let mut blind = Blind::new();

    loop {
        // read inputs
        let mut data = [0; 1];
        let mut msgs = [LinuxI2CMessage::read(&mut data).with_address(ADDR_PCF8574)];

        // Send the messages to the kernel to process
        match bus.transfer(&mut msgs) {
            Ok(rc) => println!("Successful transfer call: {} messages processed", rc),
            Err(_e) => {
                println!("Error reading/writing {}", _e);
                return; // continue
            }
        }

        // Print the data read from the device.  A recently reset root should
        // return:
        // 0x8005000000000000ff00
        let mut output = "Result: 0x".to_string();
        for byte in &data {
            output = format!("{}{:02x}", output, byte);
        }
        println!("{}", output);

        let input = BitValue { val: data[0] };
        match blind.update(
            time::Instant::now(),
            input.val.bit_is_set(0),
            input.val.bit_is_set(1),
        ) {
            None => println!("Whatever"),
            Some((up, down)) => println!("{}, {}", up, down),
        }

        // // write outputs
        // msgs = [LinuxI2CMessage::write(&[0b1000_0000]).with_address(ADDR_MAX7311)];

        // // Send the messages to the kernel to process
        // match bus.transfer(&mut msgs) {
        //     Ok(rc) => println!("Successful transfer call: {} messages processed", rc),
        //     Err(_e) => {
        //         println!("Error reading/writing {}", _e);
        //         return;
        //     }
        // }

        let ten_millis = time::Duration::from_millis(20);
        thread::sleep(ten_millis);
    }
}
