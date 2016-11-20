// Zinc, the bare metal stack for rust.
// Copyright 2014 Lionel Flandrin <lionel@svkt.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(missing_docs)]

//! Pin configuration
//! Allows GPIO configuration
//! Pin muxing not implemented yet.

use hal::pin::{Gpio, GpioDirection, In, Out, GpioLevel, High, Low};
use hal::tiva_c::sysctl;

macro_rules! pin {
  ($name:ident : $type_name:ident, $periph:expr, $regs:expr, $index:expr) => {
    #[derive(Clone, Copy)]
    pub struct $type_name;

    impl Pin for $type_name {
      #[inline(always)]
      fn periph(&self) -> sysctl::periph::PeripheralClock {
        $periph
      }

      #[inline(always)]
      fn regs(&self) -> &'static reg::Port {
        get_reg_ref($regs)
      }

      #[inline(always)]
      fn index(&self) -> usize {
        $index
      }
    }

    pub const $name: $type_name = $type_name;
  }
}

pub mod pins {
  use super::*;
  use hal::tiva_c::sysctl;
  use util::support::get_reg_ref;

  // TODO
  pin!(PIN_B4: PinB4, sysctl::periph::gpio::PORT_B, reg::PORT_B,  4);

  pin!(PIN_C5: PinC5, sysctl::periph::gpio::PORT_C, reg::PORT_C,  5);
  pin!(PIN_C6: PinC6, sysctl::periph::gpio::PORT_C, reg::PORT_C,  6);

  pin!(PIN_D0: PinD0, sysctl::periph::gpio::PORT_D, reg::PORT_D,  0);
  pin!(PIN_D1: PinD1, sysctl::periph::gpio::PORT_D, reg::PORT_D,  1);
  pin!(PIN_D2: PinD2, sysctl::periph::gpio::PORT_D, reg::PORT_D,  2);
  pin!(PIN_D3: PinD3, sysctl::periph::gpio::PORT_D, reg::PORT_D,  3);
  pin!(PIN_D4: PinD4, sysctl::periph::gpio::PORT_D, reg::PORT_D,  4);
  pin!(PIN_D5: PinD5, sysctl::periph::gpio::PORT_D, reg::PORT_D,  5);
  pin!(PIN_D6: PinD6, sysctl::periph::gpio::PORT_D, reg::PORT_D,  6);
  pin!(PIN_D7: PinD7, sysctl::periph::gpio::PORT_D, reg::PORT_D,  7);

  pin!(PIN_E4: PinE4, sysctl::periph::gpio::PORT_E, reg::PORT_E,  4);
  pin!(PIN_E5: PinE5, sysctl::periph::gpio::PORT_E, reg::PORT_E,  5);

  pin!(PIN_F0: PinF0, sysctl::periph::gpio::PORT_F, reg::PORT_F,  0);
  pin!(PIN_F1: PinF1, sysctl::periph::gpio::PORT_F, reg::PORT_F,  1);
  pin!(PIN_F2: PinF2, sysctl::periph::gpio::PORT_F, reg::PORT_F,  2);
  pin!(PIN_F3: PinF3, sysctl::periph::gpio::PORT_F, reg::PORT_F,  3);
  pin!(PIN_F4: PinF4, sysctl::periph::gpio::PORT_F, reg::PORT_F,  4);
}


pub trait Pin {
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Port;
  fn index(&self) -> usize;

  /// Configure GPIO pin
  fn configure(&self, function: u8) {
    self.periph().ensure_enabled();

    // Disable the GPIO during reconfig
    self.regs().den.set_den(self.index(), false);

    // Configure the "alternate function". AFSEL 0 means GPIO, 1 means the port
    // is driven by another peripheral. When AFSEL is 1 the actual function
    // config goes into the CTL register.
    match function {
      0 => {
        self.regs().afsel.set_afsel(self.index(),
                                  reg::Port_afsel_afsel::GPIO);
      },
      f => {
        self.regs().afsel.set_afsel(self.index(),
                                  reg::Port_afsel_afsel::PERIPHERAL);

        self.regs().pctl.set_pctl(self.index(), f as u32);
      }
    }

    // We can chose to drive each GPIO at either 2, 4 or 8mA. Default to 2mA for
    // now.
    // TODO(simias): make that configurable
    self.regs().dr2r.set_dr2r(self.index(), true);
    self.regs().dr4r.set_dr4r(self.index(), false);
    self.regs().dr8r.set_dr8r(self.index(), false);

    // TODO(simias): configure open drain/pull up/pull down/slew rate if necessary

    self.regs().odr.set_odr(self.index(), false);
    self.regs().pur.set_pur(self.index(), false);
    self.regs().pdr.set_pdr(self.index(), false);

    // Enable GPIO
    self.regs().den.set_den(self.index(), true);
  }

  fn set_level(&self, level: bool) {
    self.regs().data.set_data(self.index(), level);
  }
}

impl<T: Pin> Gpio for T {
  /// Sets output GPIO value to high.
  fn set_high(&self) {
    self.set_level(true);
  }

  /// Sets output GPIO value to low.
  fn set_low(&self) {
    self.set_level(false);
  }

  /// Returns input GPIO level.
  fn level(&self) -> GpioLevel {
    match self.regs().data.data(self.index()) {
      true  => High,
      false => Low,
    }
  }

  /// Sets output GPIO direction.
  fn set_direction(&self, dir: GpioDirection) {
    // Disable the GPIO during reconfig
    self.regs().den.set_den(self.index(), false);

    self.regs().dir.set_dir(self.index(),
                          match dir {
                            In  => reg::Port_dir_dir::INPUT,
                            Out => reg::Port_dir_dir::OUTPUT,
                          });

    self.regs().den.set_den(self.index(), true);
  }
}

pub mod reg {
  //! Pin registers definition
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(Port = {
    0x3FC => reg32 data {
      //! Pin value
      0..7 => data[8]
    }

    0x400 => reg32 dir {
      //! Pin direction
      0..7 => dir[8] {
        0 => INPUT,
        1 => OUTPUT,
      }
    }

    0x420 => reg32 afsel {
      //! Pin alternate function
      0..7 => afsel[8] {
        0 => GPIO,
        1 => PERIPHERAL,
      }
    }

    0x500 => reg32 dr2r {
      //! Select 2mA drive strength
      0..7 => dr2r[8]
    }

    0x504 => reg32 dr4r {
      //! Select 4mA drive strength
      0..7 => dr4r[8]
    }

    0x508 => reg32 dr8r {
      //! Select 8mA drive strength
      0..7 => dr8r[8]
    }

    0x50C => reg32 odr {
      //! Configure pin as open drain
      0..7 => odr[8]
    }

    0x510 => reg32 pur {
      //! Enable pin pull-up
      0..7 => pur[8]
    }

    0x514 => reg32 pdr {
      //! Enable pin pull-down
      0..7 => pdr[8]
    }

    0x518 => reg32 slr {
      //! Slew rate control enable (only available for 8mA drive strength)
      0..7 => slr[8]
    }

    0x51C => reg32 den {
      //! Enable pin
      0..7 => den[8]
    }

    0x52C => reg32 pctl {
      //! Pin function selection when afsel is set for the pin.
      0..31 => pctl[8]
    }
  });

  pub const PORT_A: *const Port = 0x40004000 as *const Port;
  pub const PORT_B: *const Port = 0x40005000 as *const Port;
  pub const PORT_C: *const Port = 0x40006000 as *const Port;
  pub const PORT_D: *const Port = 0x40007000 as *const Port;
  pub const PORT_E: *const Port = 0x40024000 as *const Port;
  pub const PORT_F: *const Port = 0x40025000 as *const Port;
}
