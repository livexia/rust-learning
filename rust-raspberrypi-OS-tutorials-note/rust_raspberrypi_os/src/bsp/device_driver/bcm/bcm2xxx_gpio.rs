// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

//! GPIO Driver.

use crate::{
    bsp::device_driver::common::MMIODerefWrapper, cpu, driver, synchronization,
    synchronization::NullLock,
}

use register::{mmio::*, register_bitfields, register_structs};

