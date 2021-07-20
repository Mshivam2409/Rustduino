//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Ayush Agarwal, Indian Institute of Technology Kanpur
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>

//! This is the implementation for Analog Referencing in Integrated circuit of AVR Chips.  
//! This code is written taking into account the features available in ATMEGA328P.
//! This code implements the Analog Read function to read from the buffer using analog signals.
//! This code implements the Analog Write function to write into the buffer using analog signals.
//! Refer to section 22 and 23 of ATMEGA328P datasheet.
//! https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

/// Crates to be used for the implementation.
use volatile::Volatile;

/// Structure to control the implementation of Integrated Analog Circuit.
#[repr(C, packed)]
pub struct AnalogComparator {
    acsr: Volatile<u8>,
}

/// Structure to control the digital signal access.
#[repr(C, packed)]
pub struct Digital {
    didr0: Volatile<u8>,
    didr1: Volatile<u8>,
}

/// Structure to control data transfer from Analog to Digital signal conversions.
#[repr(C, packed)]
pub struct Analog {
    adcl: Volatile<u8>,
    adch: Volatile<u8>,
    adcsra: Volatile<u8>,
    adcsrb: Volatile<u8>,
    admux: Volatile<u8>,
}

pub enum RefType{
    DEFAULT,
    INTERNAL1V1,
    EXTERNAL,
}

impl AnalogComparator {
    /// New pointer object created for Analog Comparator Structure.
    pub unsafe fn new() -> &'static mut AnalogComparator {
        &mut *(0x50 as *mut AnalogComparator)
    }
}

impl Digital {
    /// New pointer object created for Digital Structure.
    pub unsafe fn new() -> &'static mut Digital {
        &mut *(0x7E as *mut Digital)
    }
}

impl Analog {
    /// New pointer object created for Analog Structure.
    pub unsafe fn new() -> &'static mut Analog {
        &mut *(0x78 as *mut Analog)
    }

    /// Function to create a reference for Analog signals.
    pub fn analog_reference() {
        match reftype{
            RefType::DEFAULT=>{
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b01);
                });
            }
            RefType::INTERNAL1V1=>{self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b10);
                });
            }
            RefType::EXTERNAL=>{self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b00);
                });
            }
        }
    }


    }

    /// Function to read data which is got as input to Analog Pins.
    pub fn analog_read() {

    }

    /// Function to write data as an output through Analog Pins.
    pub fn analog_write() {

    }
}
