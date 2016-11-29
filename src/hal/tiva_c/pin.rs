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

use hal::cortex_m4::nvic;
use hal::pin::{Gpio, GpioDirection, In, Out, GpioLevel, High, Low};
use hal::tiva_c::sysctl;

macro_rules! pin {
  ($name:ident : $type_name:ident, $periph:expr, $regs:expr, $index:expr, $irq_num:expr) => {
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

      #[inline(always)]
      fn irq_num(&self) -> usize {
        $irq_num
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
  pin!(PIN_A0: PinA0, sysctl::periph::gpio::PORT_A, reg::PORT_A,  0, 16);
  pin!(PIN_A1: PinA1, sysctl::periph::gpio::PORT_A, reg::PORT_A,  1, 16);
  pin!(PIN_A2: PinA2, sysctl::periph::gpio::PORT_A, reg::PORT_A,  2, 16);
  pin!(PIN_A3: PinA3, sysctl::periph::gpio::PORT_A, reg::PORT_A,  3, 16);
  pin!(PIN_A4: PinA4, sysctl::periph::gpio::PORT_A, reg::PORT_A,  4, 16);
  pin!(PIN_A5: PinA5, sysctl::periph::gpio::PORT_A, reg::PORT_A,  5, 16);
  pin!(PIN_A6: PinA6, sysctl::periph::gpio::PORT_A, reg::PORT_A,  6, 16);
  pin!(PIN_A7: PinA7, sysctl::periph::gpio::PORT_A, reg::PORT_A,  7, 16);

  pin!(PIN_B4: PinB4, sysctl::periph::gpio::PORT_B, reg::PORT_B,  4, 17);
  pin!(PIN_B6: PinB6, sysctl::periph::gpio::PORT_B, reg::PORT_B,  6, 17);

  pin!(PIN_C0: PinC0, sysctl::periph::gpio::PORT_C, reg::PORT_C,  0, 18);
  pin!(PIN_C1: PinC1, sysctl::periph::gpio::PORT_C, reg::PORT_C,  1, 18);
  pin!(PIN_C2: PinC2, sysctl::periph::gpio::PORT_C, reg::PORT_C,  2, 18);
  pin!(PIN_C3: PinC3, sysctl::periph::gpio::PORT_C, reg::PORT_C,  3, 18);
  pin!(PIN_C4: PinC4, sysctl::periph::gpio::PORT_C, reg::PORT_C,  4, 18);
  pin!(PIN_C5: PinC5, sysctl::periph::gpio::PORT_C, reg::PORT_C,  5, 18);
  pin!(PIN_C6: PinC6, sysctl::periph::gpio::PORT_C, reg::PORT_C,  6, 18);
  pin!(PIN_C7: PinC7, sysctl::periph::gpio::PORT_C, reg::PORT_C,  7, 18);

  pin!(PIN_D0: PinD0, sysctl::periph::gpio::PORT_D, reg::PORT_D,  0, 19);
  pin!(PIN_D1: PinD1, sysctl::periph::gpio::PORT_D, reg::PORT_D,  1, 19);
  pin!(PIN_D2: PinD2, sysctl::periph::gpio::PORT_D, reg::PORT_D,  2, 19);
  pin!(PIN_D3: PinD3, sysctl::periph::gpio::PORT_D, reg::PORT_D,  3, 19);
  pin!(PIN_D4: PinD4, sysctl::periph::gpio::PORT_D, reg::PORT_D,  4, 19);
  pin!(PIN_D5: PinD5, sysctl::periph::gpio::PORT_D, reg::PORT_D,  5, 19);
  pin!(PIN_D6: PinD6, sysctl::periph::gpio::PORT_D, reg::PORT_D,  6, 19);
  pin!(PIN_D7: PinD7, sysctl::periph::gpio::PORT_D, reg::PORT_D,  7, 19);

  pin!(PIN_E0: PinE0, sysctl::periph::gpio::PORT_E, reg::PORT_E,  0, 20);
  pin!(PIN_E1: PinE1, sysctl::periph::gpio::PORT_E, reg::PORT_E,  1, 20);
  pin!(PIN_E2: PinE2, sysctl::periph::gpio::PORT_E, reg::PORT_E,  2, 20);
  pin!(PIN_E3: PinE3, sysctl::periph::gpio::PORT_E, reg::PORT_E,  3, 20);
  pin!(PIN_E4: PinE4, sysctl::periph::gpio::PORT_E, reg::PORT_E,  4, 20);
  pin!(PIN_E5: PinE5, sysctl::periph::gpio::PORT_E, reg::PORT_E,  5, 20);
  pin!(PIN_E6: PinE6, sysctl::periph::gpio::PORT_E, reg::PORT_E,  6, 20);
  pin!(PIN_E7: PinE7, sysctl::periph::gpio::PORT_E, reg::PORT_E,  7, 20);

  pin!(PIN_F0: PinF0, sysctl::periph::gpio::PORT_F, reg::PORT_F,  0, 46);
  pin!(PIN_F1: PinF1, sysctl::periph::gpio::PORT_F, reg::PORT_F,  1, 46);
  pin!(PIN_F2: PinF2, sysctl::periph::gpio::PORT_F, reg::PORT_F,  2, 46);
  pin!(PIN_F3: PinF3, sysctl::periph::gpio::PORT_F, reg::PORT_F,  3, 46);
  pin!(PIN_F4: PinF4, sysctl::periph::gpio::PORT_F, reg::PORT_F,  4, 46);
}


pub trait Pin {
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Port;
  fn index(&self) -> usize;
  fn irq_num(&self) -> usize;

  fn unlock(&self) {
    self.regs().lock.set_lock(0x4C4F434B); // magic number
    self.regs().cr.set_cr(self.index(), true);
    self.regs().lock.set_lock(0);
  }

  /// Configure GPIO pin
  fn configure(&self, function: u8) {
    self.periph().ensure_enabled();

    // Unlock (only needed for certain pins)
    self.unlock();

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

  fn set_level(&self, level: GpioLevel) {
    let level = match level {
      Low => false,
      High => true,
    };
    self.regs().data.set_data(self.index(), level);
  }

  fn set_pull_up(&self, enabled: bool) {
    self.regs().pur.set_pur(self.index(), enabled);
  }

  fn set_pull_down(&self, enabled: bool) {
    self.regs().pdr.set_pdr(self.index(), enabled);
  }

  fn enable_interrupt(&self) {
    nvic::enable_irq(self.irq_num() - 16);
    self.regs().ibe.set_ibe(self.index(), reg::Port_ibe_ibe::BothEdges);
    self.regs().im.set_ime(self.index(), reg::Port_im_ime::Enable);
  }

  fn clear_interrupt(&self) {
    self.regs().icr.set_ic(self.index(), reg::Port_icr_ic::Clear);
  }
}

impl<T: Pin> Gpio for T {
  /// Sets output GPIO value to high.
  fn set_high(&self) {
    self.set_level(High);
  }

  /// Sets output GPIO value to low.
  fn set_low(&self) {
    self.set_level(Low);
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

    0x408 => reg32 ibe {
      7..0 => ibe[8] {
        0 => SingleEdge,
        1 => BothEdges,
      }
    }

    0x410 => reg32 im {
      //! Interrupt mask enable
      7..0 => ime[8] {
        0 => Mask,
        1 => Enable,
      }
    }

    0x41C => reg32 icr {
      //! Interrupt clear
      7..0 => ic[8] {
        0 => DoNothing,
        1 => Clear,
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

    0x520 => reg32 lock {
      //! Lock
      31..0 => lock
    }

    0x524 => reg32 cr {
      //! Commit
      7..0 => cr[8]
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
