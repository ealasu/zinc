#![allow(missing_docs)]

use hal::timer;
use util::support::get_reg_ref;

#[derive(Clone, Copy)]
pub enum TimerId {
  Timer0,
  Timer1,
  Timer2,
  Timer3,
}

#[derive(Clone, Copy)]
pub struct Timer {
  /// Timer register interface
  regs: &'static reg::Timer,
}

impl Timer {
  /// Create a Timer
  pub fn new(id: TimerId) -> Timer {
    let regs = match id {
      TimerId::Timer0  => reg::PIT_0,
      TimerId::Timer1  => reg::PIT_1,
      TimerId::Timer2  => reg::PIT_2,
      TimerId::Timer3  => reg::PIT_3,
    };

    Timer {
      regs: get_reg_ref(regs)
    }
  }

  pub fn set_start_value(&self, start_value: u32) {
    self.regs.ldval.set_tsv(start_value);
  }

  pub fn set_interrupts_enabled(&self, enabled: bool) {
    self.regs.ctrl.set_tie(enabled);
  }

  pub fn set_enabled(&self, enabled: bool) {
    self.regs.ctrl.set_ten(enabled);
  }
}

impl timer::Timer for Timer {
  /// Retrieve the current timer value
  #[inline(always)]
  fn get_counter(&self) -> u32 {
    // We count down, however the trait code expects that the counter increases,
    // so we just complement the value to get an increasing counter.
    !self.regs.cval.tvl()
  }
}


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

  pub const PIT_MCR: *const TimerModuleCtrl        = 0x40037000 as *const TimerModuleCtrl;
  pub const PIT_0: *const Timer = 0x40037100 as *const Timer;
  pub const PIT_1: *const Timer = 0x40037110 as *const Timer;
  pub const PIT_2: *const Timer = 0x40037120 as *const Timer;
  pub const PIT_3: *const Timer = 0x40037130 as *const Timer;
}
