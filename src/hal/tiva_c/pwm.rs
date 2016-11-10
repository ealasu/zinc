#![allow(missing_docs)]

use hal::tiva_c::sysctl;
use util::support::get_reg_ref;
use hal::cortex_m4::nvic;
use hal::tiva_c::pin::Pin;
use hal::tiva_c::pin::pins::*;


macro_rules! pwm_gen {
  ($name:ident : $type_name:ident, $ctl_regs:expr, $regs:expr, $periph:expr,
   $pin_a:ident, $pin_a_fn:expr, $pin_b:ident, $pin_b_fn:expr) => {
    #[derive(Clone, Copy)]
    pub struct $type_name;

    impl PwmGen for $type_name {
      type PinA = $pin_a;
      type PinB = $pin_b;

      fn periph(&self) -> sysctl::periph::PeripheralClock {
        $periph
      }

      fn regs(&self) -> &'static reg::PwmGen {
        get_reg_ref($regs)
      }

      fn ctl_regs(&self) -> &'static reg::PwmCtl {
        get_reg_ref($ctl_regs)
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
pwm_gen!(PWM1_GEN2: Pwm1Gen2, reg::PWM_1_CTL, reg::PWM_1_GEN_2, sysctl::periph::pwm::PWM_1, PinF0, 5, PinF1, 5);


pub trait PwmGen {
  type PinA: Pin;
  type PinB: Pin;
  
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn ctl_regs(&self) -> &'static reg::PwmCtl;
  fn regs(&self) -> &'static reg::PwmGen;
  fn pin_a(&self) -> Self::PinA;
  fn pin_a_function(&self) -> u8;
  fn pin_b(&self) -> Self::PinB;
  fn pin_b_function(&self) -> u8;

  fn configure(&self) {
    self.periph().ensure_enabled();
  }

  fn configure_b(&self) {
    self.pin_b().configure(self.pin_b_function());
    self.regs().gen[1].set_load(1);
    self.regs().gen[1].set_actcmpad(0);
  }
}


pub mod reg {
  //! Timer registers definition
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(PwmCtl = {
    0x00 => reg32 cfg {
      0..2 => cfg {
        0 => FullWidth,
        1 => Rtc,
        4 => HalfWidth,
      },
    }
  });

  ioregs!(PwmGen = {
    0x00 => reg32 cfg {
      18 => latch,
    }
    0x20 => reg32 gen[2] {
      7..6 => actcmpad,
      3..2 => load,
    }
  });

  pub const PWM_0_CTL: *const PwmCtl = 0x4002_8000 as *const PwmCtl;
  pub const PWM_0_GEN_0: *const PwmGen = 0x4002_8040 as *const PwmGen;
  pub const PWM_0_GEN_1: *const PwmGen = 0x4002_8080 as *const PwmGen;
  pub const PWM_0_GEN_2: *const PwmGen = 0x4002_80C0 as *const PwmGen;
  pub const PWM_0_GEN_3: *const PwmGen = 0x4002_8100 as *const PwmGen;
  pub const PWM_1_CTL: *const PwmCtl = 0x4002_9000 as *const PwmCtl;
  pub const PWM_1_GEN_0: *const PwmGen = 0x4002_9040 as *const PwmGen;
  pub const PWM_1_GEN_1: *const PwmGen = 0x4002_9080 as *const PwmGen;
  pub const PWM_1_GEN_2: *const PwmGen = 0x4002_90C0 as *const PwmGen;
  pub const PWM_1_GEN_3: *const PwmGen = 0x4002_9100 as *const PwmGen;
}
