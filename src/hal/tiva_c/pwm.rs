#![allow(missing_docs)]

use hal::tiva_c::sysctl;
use util::support::get_reg_ref;
use hal::cortex_m4::nvic;
use hal::tiva_c::pin::Pin;
use hal::tiva_c::pin::pins::*;


macro_rules! pwm_gen {
  ($name:ident : $type_name:ident, $regs:expr, $periph:expr, $pin_a:ident, $pin_b:ident) => {
    #[derive(Clone, Copy)]
    pub struct $type_name;

    impl PwmGen for $type_name {
      type PinA = $pin_a;
      type PinB = $pin_b;

      fn periph(&self) -> sysctl::periph::PeripheralClock {
        $periph
      }

      fn regs(&self) -> &'static reg::Pwm {
        get_reg_ref($regs)
      }

      fn pin_a(&self) -> Self::PinA { $pin_a }
      fn pin_b(&self) -> Self::PinB { $pin_b }
    }

    pub const $name: $type_name = $type_name;
  }
}

// TODO
pwm_gen!(PWM1: Pwm1, reg::PWM_1, sysctl::periph::pwm::PWM_1, PinF0, PinF1);


pub trait PwmGen {
  type PinA;
  type PinB;
  
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Pwm;
  fn pin_a(&self) -> Self::PinA;
  fn pin_b(&self) -> Self::PinB;

  fn configure(&self) {
  }
}


pub mod reg {
  //! Timer registers definition
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(Pwm = {
    0x00 => reg32 cfg {
      //! Timer configuration
      0..2 => cfg {
        0 => FullWidth,
        1 => Rtc,
        4 => HalfWidth,
      },
    }
  });

  pub const PWM_1: *const Pwm = 0x0 as *const Pwm;
}
