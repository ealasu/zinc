#![allow(missing_docs)]

use hal::tiva_c::sysctl;
use util::support::get_reg_ref;
use hal::cortex_m4::nvic;
use hal::tiva_c::pin::Pin;
use hal::tiva_c::pin::pins::*;


macro_rules! pwm_gen {
  ($name:ident : $type_name:ident, $regs:expr, $periph:expr,
   $pin_a:ident, $pin_a_fn:expr, $pin_b:ident, $pin_b_fn:expr) => {
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
      fn pin_a_function(&self) -> u8 { $pin_a_fn }
      fn pin_b(&self) -> Self::PinB { $pin_b }
      fn pin_b_function(&self) -> u8 { $pin_b_fn }
    }

    pub const $name: $type_name = $type_name;
  }
}

// TODO
pwm_gen!(PWM1: Pwm1, reg::PWM_1, sysctl::periph::pwm::PWM_1, PinF0, 5, PinF1, 5);


pub trait PwmGen {
  type PinA: Pin;
  type PinB: Pin;
  
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Pwm;
  fn pin_a(&self) -> Self::PinA;
  fn pin_a_function(&self) -> u8;
  fn pin_b(&self) -> Self::PinB;
  fn pin_b_function(&self) -> u8;

  fn configure(&self) {
    self.periph().ensure_enabled();
  }

  fn configure_b(&self) {
    self.pin_b().configure(self.pin_b_function());
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
