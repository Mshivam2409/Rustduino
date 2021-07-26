//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Prateek Kumar Pandey, Indian Institute of Technology Kanpur
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

use crate::{com, delay::delay_ms};
use bit_field::BitField;
use fixed_slice_vec::FixedSliceVec;

const MPU6050_ADDRESS: u8 = 0x68; // 0x69 when AD0 pin to Vcc
const MPU6050_REG_ACCEL_XOFFS_H: u8 = 0x06; //defining registers for accelerometer X,Y & Z axis for high(H) and low(L).
const MPU6050_REG_ACCEL_XOFFS_L: u8 = 0x07;
const MPU6050_REG_ACCEL_YOFFS_H: u8 = 0x08;
const MPU6050_REG_ACCEL_YOFFS_L: u8 = 0x09;
const MPU6050_REG_ACCEL_ZOFFS_H: u8 = 0x0A;
const MPU6050_REG_ACCEL_ZOFFS_L: u8 = 0x0B;
///Register for sample rate division
const MPU6050_REG_ACCEL_SMPLRT_DIV: u8 = 0x0C;
const MPU6050_REG_GYRO_XOFFS_H: u8 = 0x13; //Defining registers for gyroscope X,Y & Z axis for high(H) and low(L).
const MPU6050_REG_GYRO_XOFFS_L: u8 = 0x14;
const MPU6050_REG_GYRO_YOFFS_H: u8 = 0x15;
const MPU6050_REG_GYRO_YOFFS_L: u8 = 0x16;
const MPU6050_REG_GYRO_ZOFFS_H: u8 = 0x17;
const MPU6050_REG_GYRO_ZOFFS_L: u8 = 0x18;

///* This register configures the external Frame Synchronization (FSYNC) pin sampling and the Digital Low Pass Filter (DLPF) setting for both the gyroscopes and accelerometers.
///* Used in functions :`set_dhpf_mode()` , `set_dlpf_mode()`
const MPU6050_REG_CONFIG: u8 = 0x1A;

///* This register is used to trigger gyroscope self-test and configure the gyroscopes’ full scale range.
///* Used in functions : `set_scale()` , `get_scale()`
const MPU6050_REG_GYRO_CONFIG: u8 = 0x1B;

///* This register is used to trigger accelerometer self-test and to configure the accelerometers’ full scale range.
/// Used in functions : `set_range()` , `get_range()`
const MPU6050_REG_ACCEL_CONFIG: u8 = 0x1C;
const MPU6050_REG_FF_THRESHOLD: u8 = 0x1D;
const MPU6050_REG_FF_DURATION: u8 = 0x1E;
const MPU6050_REG_MOT_THRESHOLD: u8 = 0x1F;
const MPU6050_REG_MOT_DURATION: u8 = 0x20;
const MPU6050_REG_ZMOT_THRESHOLD: u8 = 0x21;
const MPU6050_REG_ZMOT_DURATION: u8 = 0x22;

///This register determines which sensor measurements are loaded into the FIFO buffer.
const MPU6050_REG_FIFO_EN: u8 = 0x23;

///This register configures the auxiliary I2C bus for single-master or multi-master control.
const MPU6050_REG_I2C_MST_CTRL: u8 = 0x24;
///Used to specify the I2C slave address of Slave 0
const MPU6050_REG_I2C_SLV0_ADDR: u8 = 0x25;
const MPU6050_REG_I2C_SLV0_REG: u8 = 0x26;
const MPU6050_REG_I2C_SLV0_CTRL: u8 = 0x27;
///Used to specify the I2C slave address of Slave 1.
const MPU6050_REG_I2C_SLV1_ADDR: u8 = 0x28;
const MPU6050_REG_I2C_SLV1_REG: u8 = 0x29;
const MPU6050_REG_I2C_SLV1_CTRL: u8 = 0x2A;
///Used to specify the I2C slave address of Slave 2.
const MPU6050_REG_I2C_SLV2_ADDR: u8 = 0x2B;
const MPU6050_REG_I2C_SLV2_REG: u8 = 0x2C;
const MPU6050_REG_I2C_SLV2_CTRL: u8 = 0x2D;
///Used to specify the I2C slave address of Slave 3.
const MPU6050_REG_I2C_SLV3_ADDR: u8 = 0x2E;
///Slave3 configuration registers
const MPU6050_REG_I2C_SLV3_REG: u8 = 0x2F;
const MPU6050_REG_I2C_SLV3_CTRL: u8 = 0x30;
///Used to specify the I2C slave address of Slave 4.
const MPU6050_REG_I2C_SLV4_ADDR: u8 = 0x31;
///Slave4 configuration registers
const MPU6050_REG_I2C_SLV4_REG: u8 = 0x32;
const MPU6050_REG_I2C_SLV4_DO: u8 = 0x33;
const MPU6050_REG_I2C_SLV4_CTRL: u8 = 0x34;
const MPU6050_REG_I2C_SLV4_DI: u8 = 0x35;
const MPU6050_REG_I2C_MST_STATUS: u8 = 0x36; //Indicates master control status

///* This register configures the behavior of the interrupt signals at the INT pins. This register is also used to enable the FSYNC Pin to be used as an interrupt to the host application processor, as well as to enable Bypass Mode on the I2C Master. This bit also enables the clock output
///* Used in functions :`set_i2c_byepass_enabled()`, `get_i2c_bypass_enabled()`
const MPU6050_REG_INT_PIN_CFG: u8 = 0x37;

///* This register enables interrupt generation by interrupt sources.
///* Used in functions :`set_int_motion_enable()` , `get_int_motion_enable()`, `set_int_free_fall_enabled()`, `get_int_free_fall_enabled()`, `set_int_zero_motion_enabled()`, `get_int_zero_motion_enabled()`.
const MPU6050_REG_INT_ENABLE: u8 = 0x38; // INT Enable

///* This register shows the interrupt status of each interrupt generation source.
///* Used in function : `get_int_status()`.
const MPU6050_REG_INT_STATUS: u8 = 0x3A;

///* These registers store the most recent accelerometer measurements
const MPU6050_REG_ACCEL_XOUT_H: u8 = 0x3B; // Accel XOUT High
const MPU6050_REG_ACCEL_XOUT_L: u8 = 0x3C; // Accel XOUT Low
const MPU6050_REG_ACCEL_YOUT_H: u8 = 0x3D; // Accel YOUT High
const MPU6050_REG_ACCEL_YOUT_L: u8 = 0x3E; // Accel YOUT Low
const MPU6050_REG_ACCEL_ZOUT_H: u8 = 0x3F; // Accel ZOUT High
const MPU6050_REG_ACCEL_ZOUT_L: u8 = 0x40; // Accel ZOUT Low
const MPU6050_REG_TEMP_OUT_H: u8 = 0x41;
const MPU6050_REG_TEMP_OUT_L: u8 = 0x42;
/// These registers store the most recent gyroscope measurements.
const MPU6050_REG_GYRO_XOUT_H: u8 = 0x43; //Registers for output of X,Y & Z axis.
const MPU6050_REG_GYRO_XOUT_L: u8 = 0x44;
const MPU6050_REG_GYRO_YOUT_H: u8 = 0x45;
const MPU6050_REG_GYRO_YOUT_L: u8 = 0x46;
const MPU6050_REG_GYRO_ZOUT_H: u8 = 0x47;
const MPU6050_REG_GYRO_ZOUT_L: u8 = 0x48;
const MPU6050_REG_EXT_SENS_DATA_00: u8 = 0x49; //These registers store data read from external sensors by the Slave 0, 1, 2, and 3 on the auxiliary I2C interface.
const MPU6050_REG_EXT_SENS_DATA_01: u8 = 0x4A;
const MPU6050_REG_EXT_SENS_DATA_02: u8 = 0x4B;
const MPU6050_REG_EXT_SENS_DATA_03: u8 = 0x4C;
const MPU6050_REG_EXT_SENS_DATA_04: u8 = 0x4D;
const MPU6050_REG_EXT_SENS_DATA_05: u8 = 0x4E;
const MPU6050_REG_EXT_SENS_DATA_06: u8 = 0x4F;
const MPU6050_REG_EXT_SENS_DATA_07: u8 = 0x50;
const MPU6050_REG_EXT_SENS_DATA_08: u8 = 0x51;
const MPU6050_REG_EXT_SENS_DATA_09: u8 = 0x52;
const MPU6050_REG_EXT_SENS_DATA_10: u8 = 0x53;
const MPU6050_REG_EXT_SENS_DATA_11: u8 = 0x54;
const MPU6050_REG_EXT_SENS_DATA_12: u8 = 0x55;
const MPU6050_REG_EXT_SENS_DATA_13: u8 = 0x56;
const MPU6050_REG_EXT_SENS_DATA_14: u8 = 0x57;
const MPU6050_REG_EXT_SENS_DATA_15: u8 = 0x58;
const MPU6050_REG_EXT_SENS_DATA_16: u8 = 0x59;
const MPU6050_REG_EXT_SENS_DATA_17: u8 = 0x5A;
const MPU6050_REG_EXT_SENS_DATA_18: u8 = 0x5B;
const MPU6050_REG_EXT_SENS_DATA_19: u8 = 0x5C;
const MPU6050_REG_EXT_SENS_DATA_20: u8 = 0x5D;
const MPU6050_REG_EXT_SENS_DATA_21: u8 = 0x5E;
const MPU6050_REG_EXT_SENS_DATA_22: u8 = 0x5F;
const MPU6050_REG_EXT_SENS_DATA_23: u8 = 0x60;
const MPU6050_REG_MOT_DETECT_STATUS: u8 = 0x61;
const MPU6050_REG_I2C_SLV0_DO: u8 = 0x63;
const MPU6050_REG_I2C_SLV1_DO: u8 = 0x64;
const MPU6050_REG_I2C_SLV2_DO: u8 = 0x65;
const MPU6050_REG_I2C_SLV3_DO: u8 = 0x66;
const MPU6050_REG_I2C_MST_DELAY_CTRL: u8 = 0x67;
const MPU6050_REG_SIGNAL_PATH_RESET: u8 = 0x68;
const MPU6050_REG_MOT_DETECT_CTRL: u8 = 0x69;
const MPU6050_REG_USER_CTRL: u8 = 0x6A; // User Control
const MPU6050_REG_PWR_MGMT_1: u8 = 0x6B; // Power Management 1
const MPU6050_REG_PWR_MGMT_2: u8 = 0x6C;
const MPU6050_REG_FIFO_COUNTH: u8 = 0x72;
const MPU6050_REG_FIFO_COUNTL: u8 = 0x73;
const MPU6050_REG_FIFO_R_W: u8 = 0x74;
const MPU6050_REG_WHO_AM_I: u8 = 0x75; // Who Am I

pub enum MPUClockSourceT {
    MPU6050ClockInternal8MHZ,
    MPU6050ClockPllGyrox,
    MPU6050ClockPllGyroy,
    MPU6050ClockPllGyroz,
    MPU6050ClockExternal32MHZ,
    MPU6050ClockExternal19MHZ,
    MPU6050ClockKeepReset,
}

pub enum MPUdpsT {
    MPU6050Scale2000DPS,
    MPU6050Scale1000DPS,
    MPU6050Scale500DPS,
    MPU6050Scale250DPS,
}

pub enum MPURangeT {
    MPU6050Range2G,
    MPU6050Range4G,
    MPU6050Range8G,
    MPU6050Range16G,
}

pub enum MPUOnDelayT {
    MPU6050Delay3MS,
    MPU6050Delay2MS,
    MPU6050Delay1MS,
    MPU6050NoDelay,
}

pub enum MPUdhpfT {
    MPU6050dhpfReset,
    MPU6050dhpf5HZ,
    MPU6050dhpf2_5HZ,
    MPU6050dhpf1_25HZ,
    MPU6050dhpf0_63HZ,
    MPU6050dhpfHold,
}

pub enum MPUdlpfT {
    MPU6050dlpf6,
    MPU6050dlpf5,
    MPU6050dlpf4,
    MPU6050dlpf3,
    MPU6050dlpf2,
    MPU6050dlpf1,
    MPU6050dlpf0,
}

pub struct MPU6050 {
    address: u8,
}

impl MPU6050 {
    ///Returns a mutable refernce to the struct to be used in the implementations
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0x75 as *mut Self) }
    }

    fn readregister(&mut self, reg: u8) -> u8 {
        let mut vec1: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        vec1.push(reg);
        let i2c = com::i2c::Twi::new();
        i2c.read_from_slave(MPU6050_ADDRESS, 1, &mut vec1);
        return vec1[1];
    }

    fn writeregister(&mut self, reg: u8, value: u8) {
        let mut vec2: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        vec2.push(reg);
        vec2.push(value);
        let i2c = com::i2c::Twi::new();
        i2c.write_to_slave(MPU6050_ADDRESS, &vec2);
    }

    fn writeregister_bit(&mut self, reg: u8, pos: u8, state: bool) {
        let mut value: u8;
        value = self.readregister(reg);
        if state {
            value |= 1 << pos;
        } else {
            value &= !(1 << pos);
        }
        self.writeregister(reg, value);
    }

    pub fn set_dlpf_mode(&mut self, dlpf: MPUdlpfT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_CONFIG);
        value &= 0b11111000;
        value |= match dlpf {
            MPUdlpfT::MPU6050dlpf6 => 0b110,
            MPUdlpfT::MPU6050dlpf5 => 0b101,
            MPUdlpfT::MPU6050dlpf4 => 0b100,
            MPUdlpfT::MPU6050dlpf3 => 0b011,
            MPUdlpfT::MPU6050dlpf2 => 0b010,
            MPUdlpfT::MPU6050dlpf1 => 0b001,
            MPUdlpfT::MPU6050dlpf0 => 0b000,
        };
        self.writeregister(MPU6050_REG_CONFIG, value);
    }

    pub fn set_dhpf_mode(&mut self, dhpf: MPUdhpfT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_CONFIG);
        value &= 0b11111100;
        value |= match dhpf {
            MPUdhpfT::MPU6050dhpfReset => 0b000,
            MPUdhpfT::MPU6050dhpf5HZ => 0b001,
            MPUdhpfT::MPU6050dhpf2_5HZ => 0b010,
            MPUdhpfT::MPU6050dhpf1_25HZ => 0b011,
            MPUdhpfT::MPU6050dhpf0_63HZ => 0b100,
            MPUdhpfT::MPU6050dhpfHold => 0b101,
        };
        self.writeregister(MPU6050_REG_CONFIG, value);
    }

    pub fn set_scale(&mut self, scale: MPUdpsT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_GYRO_CONFIG);
        value &= 0b11100111;
        value |= (match scale {
            MPUdpsT::MPU6050Scale2000DPS => 3,
            MPUdpsT::MPU6050Scale1000DPS => 2,
            MPUdpsT::MPU6050Scale500DPS => 1,
            MPUdpsT::MPU6050Scale250DPS => 0,
        } << 3);
        self.writeregister(MPU6050_REG_GYRO_CONFIG, value);
    }

    pub fn get_scale(&mut self) -> MPUdpsT {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_GYRO_CONFIG);
        value &= 0b00011000;
        value >>= 3;
        if value == 3 {
            return MPUdpsT::MPU6050Scale2000DPS;
        } else if value == 2 {
            return MPUdpsT::MPU6050Scale1000DPS;
        } else if value == 1 {
            return MPUdpsT::MPU6050Scale500DPS;
        } else {
            return MPUdpsT::MPU6050Scale250DPS;
        }
    }

    pub fn set_range(&mut self, range: MPURangeT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_ACCEL_CONFIG);
        value &= 0b11100111;
        value |= (match range {
            MPURangeT::MPU6050Range2G => 0,
            MPURangeT::MPU6050Range4G => 1,
            MPURangeT::MPU6050Range8G => 2,
            MPURangeT::MPU6050Range16G => 3,
        } << 3);
        self.writeregister(MPU6050_REG_ACCEL_CONFIG, value);
    }

    pub fn get_range(&mut self) -> MPURangeT {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_ACCEL_CONFIG);
        value &= 0b00011000;
        value >>= 3;
        if value == 3 {
            return MPURangeT::MPU6050Range16G;
        } else if value == 2 {
            return MPURangeT::MPU6050Range8G;
        } else if value == 1 {
            return MPURangeT::MPU6050Range4G;
        } else {
            return MPURangeT::MPU6050Range2G;
        }
    }

    pub fn set_clock_source(&mut self, source: MPUClockSourceT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        value &= 0b11111000;
        value |= match source {
            MPUClockSourceT::MPU6050ClockInternal8MHZ => 0,
            MPUClockSourceT::MPU6050ClockPllGyrox => 1,
            MPUClockSourceT::MPU6050ClockPllGyroy => 2,
            MPUClockSourceT::MPU6050ClockPllGyroz => 3,
            MPUClockSourceT::MPU6050ClockExternal32MHZ => 4,
            MPUClockSourceT::MPU6050ClockExternal19MHZ => 5,
            MPUClockSourceT::MPU6050ClockKeepReset => 7,
        };
        self.writeregister(MPU6050_REG_PWR_MGMT_1, value);
    }

    pub fn get_clock_source(&mut self) -> MPUClockSourceT {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        value &= 0b00000111;
        if value == 0 {
            return MPUClockSourceT::MPU6050ClockInternal8MHZ;
        } else if value == 1 {
            return MPUClockSourceT::MPU6050ClockPllGyrox;
        } else if value == 2 {
            return MPUClockSourceT::MPU6050ClockPllGyroy;
        } else if value == 3 {
            return MPUClockSourceT::MPU6050ClockPllGyroz;
        } else if value == 4 {
            return MPUClockSourceT::MPU6050ClockExternal32MHZ;
        } else if value == 5 {
            return MPUClockSourceT::MPU6050ClockExternal19MHZ;
        } else {
            return MPUClockSourceT::MPU6050ClockKeepReset;
        }
    }

    pub fn set_int_free_fall_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_INT_ENABLE, 7, state);
    }

    pub fn get_int_free_fall_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_INT_ENABLE);
        return value.get_bit(6);
    }

    pub fn set_accel_power_on_delay(&mut self, delay: MPUOnDelayT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_MOT_DETECT_CTRL);
        value &= 0b11001111;
        value |= match delay {
            MPUOnDelayT::MPU6050Delay3MS => 3,
            MPUOnDelayT::MPU6050Delay2MS => 2,
            MPUOnDelayT::MPU6050Delay1MS => 1,
            MPUOnDelayT::MPU6050NoDelay => 0,
        };
        self.writeregister(MPU6050_REG_MOT_DETECT_CTRL, value);
    }

    pub fn get_accel_power_on_delay(&mut self) -> MPUOnDelayT {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_MOT_DETECT_CTRL);
        value &= 0b00110000;
        if value == 3 {
            return MPUOnDelayT::MPU6050Delay3MS;
        } else if value == 2 {
            return MPUOnDelayT::MPU6050Delay2MS;
        } else if value == 1 {
            return MPUOnDelayT::MPU6050Delay1MS;
        } else {
            return MPUOnDelayT::MPU6050NoDelay;
        }
    }

    pub fn set_motion_detection_threshold(&mut self, threshold: u8) {
        self.writeregister(MPU6050_REG_MOT_THRESHOLD, threshold);
    }

    pub fn get_motion_detection_threshold(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_MOT_THRESHOLD);
    }

    pub fn set_motion_detection_duration(&mut self, duration: u8) {
        self.writeregister(MPU6050_REG_MOT_DURATION, duration);
    }

    pub fn get_motion_detection_duration(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_MOT_DURATION);
    }

    pub fn set_zero_motion_detection_threshold(&mut self, threshold: u8) {
        self.writeregister(MPU6050_REG_ZMOT_THRESHOLD, threshold);
    }

    pub fn get_zero_motion_detection_threshold(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_ZMOT_THRESHOLD);
    }

    pub fn set_zero_motion_detection_duration(&mut self, duration: u8) {
        self.writeregister(MPU6050_REG_ZMOT_DURATION, duration);
    }

    pub fn get_zero_motion_detection_duration(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_ZMOT_DURATION);
    }

    pub fn set_free_fall_detection_threshold(&mut self, threshold: u8) {
        self.writeregister(MPU6050_REG_FF_THRESHOLD, threshold);
    }

    pub fn get_free_fall_detection_threshold(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_FF_THRESHOLD);
    }

    pub fn set_free_fall_detection_duration(&mut self, duration: u8) {
        self.writeregister(MPU6050_REG_FF_DURATION, duration);
    }

    pub fn get_free_fall_detection_duration(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_FF_DURATION);
    }

    pub fn set_sleep_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_PWR_MGMT_1, 6, state);
    }

    pub fn get_sleep_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        return value.get_bit(6);
    }

    pub fn get_int_zero_motion_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_INT_ENABLE);
        return value.get_bit(5);
    }

    pub fn set_int_zero_motion_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_INT_ENABLE, 5, state);
    }

    pub fn get_int_motion_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_INT_ENABLE);
        return value.get_bit(6);
    }

    pub fn set_int_motion_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_INT_ENABLE, 6, state);
    }

    pub fn set_i2c_master_mode_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_USER_CTRL, 5, state);
    }

    pub fn get_i2c_master_mode_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_USER_CTRL);
        return value.get_bit(5);
    }

    pub fn set_i2c_byepass_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_INT_PIN_CFG, 1, state);
    }

    pub fn get_i2c_byepass_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_INT_PIN_CFG);
        return value.get_bit(1);
    }

    pub fn get_int_status(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_INT_STATUS);
    }

    ///* Reads the three, two-byte accelerometer values from the sensor.
    ///* Returns the two-byte raw accelerometer values as a 32-bit float.
    ///* The vec accel_output stores the raw values of the accelerometer where `accel_output[0]` is the x-axis, `accel_output[1]` is the y-axis and `accel_output[2]` is the z-axis output respectively. These raw values are then converted to g's per second according to the scale given as input in `begin()` function.
    pub fn read_accel(&mut self) -> FixedSliceVec<f32> {
        let mut v: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        v.push(MPU6050_REG_ACCEL_XOUT_H);
        let i2c = com::i2c::Twi::new();
        i2c.read_from_slave(MPU6050_ADDRESS, 6, &mut v); //input from slave
        let mut accel_output: FixedSliceVec<f32> = FixedSliceVec::new(&mut []);
        accel_output.push((((v[1] as u16) << 8) | (v[2] as u16)) as f32); //input of X axis
        accel_output.push((((v[3] as u16) << 8) | (v[4] as u16)) as f32); //input of Y axis
        accel_output.push((((v[5] as u16) << 8) | (v[6] as u16)) as f32); //input of Z axis
        return accel_output;
    }

    ///* Reads the three, two-byte gyroscope values from the sensor.
    ///* Returns the two-byte raw gyroscope values as a 32-bit float.
    ///* The vec gyro_output stores the raw values of the gyroscope where `gyro_output[0]` is the x-axis, `gyro_output[1]` is the y-axis and `gyro_output[2]` is the z-axis output respectively. These raw values are then converted to degrees per second according to the scale given as input in `begin()` function.
    pub fn read_gyro(&mut self) -> FixedSliceVec<f32> {
        let mut v: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        v.push(MPU6050_REG_GYRO_XOUT_H);
        let i2c = com::i2c::Twi::new();
        i2c.read_from_slave(MPU6050_ADDRESS, 6, &mut v); //input from slave
        let mut gyro_output: FixedSliceVec<f32> = FixedSliceVec::new(&mut []);
        gyro_output.push((((v[1] as u16) << 8) | (v[2] as u16)) as f32); //input of X axis
        gyro_output.push((((v[3] as u16) << 8) | (v[4] as u16)) as f32); //input of Y axis
        gyro_output.push((((v[5] as u16) << 8) | (v[6] as u16)) as f32); //input of Z axis
        return gyro_output;
    }

    /// Starts the sensor by setting the device to active mode ,setting the accelerometer range and gyroscope scale.
    pub fn begin(&mut self, scale: MPUdpsT, range: MPURangeT) -> bool {
        delay_ms(5);

        //Set clock source.
        self.set_clock_source(MPUClockSourceT::MPU6050ClockPllGyrox);

        //Set scale and range.
        self.set_range(range);
        self.set_scale(scale);

        //disable sleep mode.
        self.set_sleep_enabled(false);

        return true;
    }
}
