#![allow(missing_docs)]

use hal::timer;
use util::support::get_reg_ref;

/// Disable module clock
#[inline(always)]
pub fn set_module_clock_disabled(disabled: bool) {
  get_reg_ref(reg::PIT_MCR).mcr.set_mdis(disabled);
}

/// Freeze in debug mode
#[inline(always)]
pub fn set_freeze(freeze: bool) {
  get_reg_ref(reg::PIT_MCR).mcr.set_frz(freeze);
}

pub trait Timer {
  fn get_regs(&self) -> &'static reg::Timer;

  #[inline(always)]
  fn set_start_value(&self, start_value: u32) {
    self.get_regs().ldval.set_tsv(start_value);
  }

  #[inline(always)]
  fn set_interrupts_enabled(&self, enabled: bool) {
    self.get_regs().ctrl.set_tie(enabled);
  }

  #[inline(always)]
  fn set_enabled(&self, enabled: bool) {
    self.get_regs().ctrl.set_ten(enabled);
  }

  #[inline(always)]
  fn interrupt_flag(&self) -> bool {
    self.get_regs().tflg.tif()
  }

  #[inline(always)]
  fn set_interrupt_flag(&self, flag: bool) {
    self.get_regs().tflg.set_tif(flag);
  }
}

impl timer::Timer for Timer {
  /// Retrieve the current timer value
  #[inline(always)]
  fn get_counter(&self) -> u32 {
    // We count down, however the trait code expects that the counter increases,
    // so we just complement the value to get an increasing counter.
    !self.get_regs().cval.tvl()
  }
}

macro_rules! impl_timer {
  ($name:ident, $regs:expr) => {
    #[derive(Clone, Copy)]
    pub struct $name;

    impl Timer for $name {
      #[inline(always)]
      fn get_regs(&self) -> &'static reg::Timer {
        get_reg_ref($regs)
      }
    }
  }
}

impl_timer!(Timer0, reg::PIT_0);
impl_timer!(Timer1, reg::PIT_1);
impl_timer!(Timer2, reg::PIT_2);
impl_timer!(Timer3, reg::PIT_3);

pub mod reg {
  //! Timer registers definition
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(TimerModuleCtrl = {
    0x00 => reg32 mcr {
      //= Disable module clock
      1 => mdis,
      //= Freeze in debug mode
      0 => frz, 
    }
  });

  ioregs!(Timer = {
    0x00 => reg32 ldval {
      //= Timer start value
      31..0 => tsv,
    }
    0x04 => reg32 cval {
      //= Current timer value
      0..31 => tvl,
    }
    0x08 => reg32 ctrl {
      //= Chained
      2 => chn,
      //= Timer interrupt enable
      1 => tie,
      //= Timer enable
      0 => ten,
    }
    0x0C => reg32 tflg {
      //= Timer interrupt flag
      0 => tif,
    }
  });

  pub const PIT_MCR: *const TimerModuleCtrl = 0x40037000 as *const TimerModuleCtrl;
  pub const PIT_0: *const Timer = 0x40037100 as *const Timer;
  pub const PIT_1: *const Timer = 0x40037110 as *const Timer;
  pub const PIT_2: *const Timer = 0x40037120 as *const Timer;
  pub const PIT_3: *const Timer = 0x40037130 as *const Timer;
}
