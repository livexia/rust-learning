// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

//! Driver support.

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Driver interfaces.
pub mod interface {
    pub trait DeviceDriver {
        fn compatible(&self) -> &str;

        fn init(&self) -> Result<(), ()> {
            Ok(())
        }
    }

    pub trait DriverManager {
        fn all_device_drivers(&self) -> &[&'static (dyn DeviceDriver +Sync)];

        fn post_device_driver_init(&self);
    }
}