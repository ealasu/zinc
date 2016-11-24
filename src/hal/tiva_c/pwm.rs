#![allow(missing_docs)]

use hal::tiva_c::sysctl;
use hal::tiva_c::pin::Pin;

pub mod pwms {
  use util::support::get_reg_ref;
  use hal::tiva_c::sysctl;
  use hal::tiva_c::pin::pins::*;
  use super::*;

  macro_rules! pwm {
    ($name:ident : $type_name:ident,
     periph=$periph:expr, ctl=$ctl_regs:expr, regs=$regs:expr, chan=$chan:expr,
     pin=$pin:ident, pin_fn=$pin_fn:expr) => {
      #[derive(Clone, Copy)]
      pub struct $type_name;

      impl PwmGen for $type_name {
        type Pin = $pin;

        fn periph(&self) -> sysctl::periph::PeripheralClock {
          $periph
        }

        fn regs(&self) -> &'static reg::PwmGen {
          get_reg_ref($regs)
        }

        fn ctl_regs(&self) -> &'static reg::PwmCtl {
          get_reg_ref($ctl_regs)
        }

        fn chan(&self) -> usize {
          $chan
        }

        fn pin(&self) -> Self::Pin { $pin }
        fn pin_function(&self) -> u8 { $pin_fn }
      }

      pub const $name: $type_name = $type_name;
    }
  }

  // TODO
  pwm!(PWM0_CHAN4: Pwm0Chan4, periph=sysctl::periph::pwm::PWM_0, ctl=reg::PWM_0_CTL, regs=reg::PWM_0_GEN_2, chan=4, pin=PinE4, pin_fn=4);
  pwm!(PWM0_CHAN6: Pwm0Chan6, periph=sysctl::periph::pwm::PWM_0, ctl=reg::PWM_0_CTL, regs=reg::PWM_0_GEN_3, chan=6, pin=PinD0, pin_fn=4);

  pwm!(PWM1_CHAN5: Pwm1Chan5, periph=sysctl::periph::pwm::PWM_1, ctl=reg::PWM_1_CTL, regs=reg::PWM_1_GEN_2, chan=5, pin=PinF1, pin_fn=5);
  pwm!(PWM1_CHAN6: Pwm1Chan6, periph=sysctl::periph::pwm::PWM_1, ctl=reg::PWM_1_CTL, regs=reg::PWM_1_GEN_3, chan=6, pin=PinF2, pin_fn=5);
}

pub trait PwmGen {
  type Pin: Pin;
  
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn ctl_regs(&self) -> &'static reg::PwmCtl;
  fn regs(&self) -> &'static reg::PwmGen;
  fn pin(&self) -> Self::Pin;
  fn pin_function(&self) -> u8;
  fn chan(&self) -> usize;

  fn index(&self) -> usize {
    self.chan() % 2
  }

  fn configure(&self) {
    self.periph().ensure_enabled();
    self.pin().configure(self.pin_function());

    self.regs().gen[self.index()].set_load(1);
    if self.index() == 0 {
      self.regs().gen[self.index()].set_actcmpad(reg::PwmGen_gen_actcmpad::Low);
    } else {
      self.regs().gen[self.index()].set_actcmpbd(reg::PwmGen_gen_actcmpbd::Low);
    }
  }

  fn clock(&self) -> u32 {
    sysctl::clock::sysclk_get() as u32 / 64
  }

  fn set_period(&self, period: u16) {
    self.regs().load.set_load(period as u32);
  }

  fn period(&self) -> u16 {
    self.regs().load.load() as u16
  }

  fn pulse_width(&self) -> u16 {
    let cmp = self.regs().cmp[self.index()].comp() as u16;
    self.period() - cmp
  }

  fn set_pulse_width(&self, pulse_width: u16) {
    let cmp = self.period() - pulse_width;
    self.regs().cmp[self.index()].set_comp(cmp as u32);
  }

  fn enable(&self) {
    // enable output
    self.ctl_regs().enable.set_enable(self.chan(), true);
    // enable gen
    self.regs().ctl.set_enable(true);
  }
}

pub mod reg {
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(PwmCtl = {
    0x08 => reg32 enable {
      7..0 => enable[8],
    }
  });

  ioregs!(PwmGen = {
    0x00 => reg32 ctl {
      18 => latch,
      0 => enable,
    }
    0x10 => reg32 load {
      15..0 => load,
    }
    0x18 => reg32 cmp[2] {
      15..0 => comp,
    }
    0x20 => reg32 gen[2] {
      11..10 => actcmpbd {
        0 => Nothing,
        1 => Invert,
        2 => Low,
        3 => High,
      },
      7..6 => actcmpad {
        0 => Nothing,
        1 => Invert,
        2 => Low,
        3 => High,
      },
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
