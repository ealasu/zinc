#![allow(missing_docs)]

use hal::tiva_c::sysctl;
use util::support::get_reg_ref;
use hal::cortex_m4::nvic;
use hal::tiva_c::pin::Pin;


macro_rules! pwm_gen {
  ($name:ident : $type_name:ident, $regs:expr, $periph:expr) => {
    #[derive(Clone, Copy)]
    pub struct $type_name;

    impl PwmGen for $type_name {
      fn periph(&self) -> sysctl::periph::PeripheralPwm {
        $periph
      }

      fn regs(&self) -> &'static reg::Pwm {
        get_reg_ref($regs)
      }
    }

    pub const $name: $type_name = $type_name;
  }
}

// TODO
pwm!(PWM1: Pwm1, reg::PWM_1, sysctl::periph::pwm::PWM_1);


pub trait PwmGen {
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Timer;

  fn configure<P: Pin>(&self, pin: P) {
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
