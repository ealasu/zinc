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

//! Timer configuration
//! This code should support both standand and wide timers

use hal::tiva_c::sysctl;
use hal::timer;
use util::support::get_reg_ref;
use hal::cortex_m4::nvic;

/// Timer modes
#[derive(Clone, Copy)]
pub enum Mode {
  /// Periodic timer loops and restarts once the timeout is reached.
  Periodic,
  /// One shot timer is disabled once the timeout is reached.
  OneShot,
  /// RTC timer is based on the 32.768KHz clock and ticks at 1Hz
  RTC,
  /// EdgeCount timer counts rising/falling/both edge events on an
  /// external pin.
  EdgeCount,
  /// EdgeTime timer measures the time it takes for a rising/falling/both edge
  /// event to occur.
  EdgeTime,
  /// PWM mode can be used to generate a configurable square wave (frequence and
  /// duty cycle)
  PWM,
}

macro_rules! timer {
  ($name:ident, $type_name:ident, $regs:expr, $periph:expr, $wide:expr, $irq_num:expr) => {
    #[derive(Clone, Copy)]
    pub struct $type_name;

    impl TivaTimer for $type_name {
      fn periph(&self) -> sysctl::periph::PeripheralClock {
        $periph
      }

      fn regs(&self) -> &'static reg::Timer {
        get_reg_ref($regs)
      }

      fn wide(&self) -> bool {
        $wide
      }

      fn irq_num(&self) -> usize {
        $irq_num
      }
    }

    impl timer::Timer for $type_name {
      /// Retrieve the current timer value
      #[inline(always)]
      fn get_counter(&self) -> u32 {
        // We count down, however the trait code expects that the counter increases,
        // so we just complement the value to get an increasing counter.
        !self.regs().tav.v()
      }
    }

    pub const $name: $type_name = $type_name;
  }
}


/// There are 6 standard 16/32bit timers and 6 "wide" 32/64bit timers
// TODO
//timer!(TIMER1, Timer1, reg::TIMER_1, sysctl::periph::timer::TIMER_1, false, 37);
timer!(TIMERW0, TimerW0, reg::TIMER_W_0, sysctl::periph::timer::TIMER_W_0, true, 110);
timer!(TIMERW1, TimerW1, reg::TIMER_W_1, sysctl::periph::timer::TIMER_W_1, true, 112);


pub trait TivaTimer {
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Timer;
  fn wide(&self) -> bool;
  fn irq_num(&self) -> usize;

  /// Configure timer registers
  /// TODO(simias): Only Periodic and OneShot modes are implemented so far
  fn configure(&self, mode: Mode, prescale: u32) {
    self.periph().ensure_enabled();

    // Make sure the timer is disabled before making changes.
    self.regs().ctl.set_taen(false);

    // Configure the timer as half-width so that we can use the prescaler
    self.regs().cfg.set_cfg(reg::Timer_cfg_cfg::HalfWidth);

    self.regs().amr
      .set_mr(match mode {
        Mode::OneShot  => reg::Timer_amr_mr::OneShot,
        Mode::Periodic => reg::Timer_amr_mr::Periodic,
        _              => panic!("Unimplemented timer mode"),
      })
      // We need to count down in order for the prescaler to work as a
      // prescaler. If we count up it becomes a timer extension (i.e. it becomes
      // the MSBs of the counter).
      .set_cdir(reg::Timer_amr_cdir::Down);

    // Set maximum timeout value to overflow as late as possible
    self.regs().tailr.set_tailr(0xffffffff);

    // Set prescale value
    if !self.wide() && prescale > 0xffff {
      panic!("prescale is too wide for this timer");
    }

    self.regs().apr.set_psr(prescale as u32);

    // Timer is now configured, we can enable it
    self.regs().ctl.set_taen(true);
  }

  fn enable_timeout_interrupt_a(&self) {
    nvic::enable_irq(self.irq_num());
    self.regs().imr.set_tatoim(true);

    self.regs().imr.set_wueim(true); //= Write Update Error interrupt mask
    self.regs().imr.set_tbmim(true); //= Timer B match interrupt mask
    self.regs().imr.set_cbeim(true); //= Timer B capture mode event interrupt mask
    self.regs().imr.set_cbmim(true); //= Timer B capture mode match interrupt mask
    self.regs().imr.set_tbtoim(true); //= Timer B time-out interrupt mask
    self.regs().imr.set_tamim(true); //= Timer A match interrupt mask
    self.regs().imr.set_rtcim(true); //= RTC interrupt mask
    self.regs().imr.set_caeim(true); //= Timer A capture mode event interrupt mask
    self.regs().imr.set_camim(true); //= Timer A capture mode match interrupt mask
    self.regs().imr.set_tatoim(true); //= Timer A time-out interrupt mask
  }

  fn set_interval_a(&self, interval: u32) {
    self.regs().tailr.set_tailr(interval);
  }

  fn clear_interrupt(&self) {
    self.regs().icr.set_tatocint(true);
  }

  fn set_config(&self, config: reg::Timer_cfg_cfg) {
    self.regs().cfg.set_cfg(config);
  }

  fn disable(&self) {
    self.regs().ctl.set_taen(false);
  }

  fn enable(&self) {
    self.regs().ctl.set_taen(true);
  }
}

impl timer::Timer for TivaTimer {
  /// Retrieve the current timer value
  #[inline(always)]
  fn get_counter(&self) -> u32 {
    // We count down, however the trait code expects that the counter increases,
    // so we just complement the value to get an increasing counter.
    !self.regs().tav.v()
  }
}

pub mod reg {
  //! Timer registers definition
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(Timer = {
    0x00 => reg32 cfg {
      //! Timer configuration
      0..2 => cfg {
        0 => FullWidth,
        1 => Rtc,
        4 => HalfWidth,
      },
    }
    0x04 => reg32 amr {
      //! Timer A mode
      0..1    => mr {      //! mode
        1 => OneShot,
        2 => Periodic,
        3 => Capture,
      },
      2       => cmr,      //= capture mode
      3       => ams,      //= alternate mode select
      4       => cdir {    //! Count direction
        0 => Down,
        1 => Up,
      },
      5       => mie,      //= match interrupt enable
      6       => wot,      //= wait on trigger
      7       => snaps,    //= snap-shot mode
      8       => ild,      //= interval load write
      9       => pwmie,    //= PWM interrupt enable
      10      => rsu,      //= match register update
      11      => plo,      //= PWM legacy operation
    }
    0x0C => reg32 ctl {
      0      => taen,      //= Timer A enable
      1      => tastall,   //= Timer A stall enable
      2..3   => taevent {  //! Timer A event mode
        0 => PosEdge,
        1 => NegEdge,
        3 => AnyEdge,
      },
      4      => rtcen,     //= RTC stall enable
      5      => taote,     //= Timer B output trigger enable
      6      => tapwml,    //= Timer B PWM output level

      8      => tben,      //= Timer B enable
      9      => tbstall,   //= Timer B stall enable
      10..11 => tbevent,   //= Timer B event mode
      13     => tbote,     //= Timer B output trigger enable
      14     => tbpwml,    //= Timer B PWM output level
    }
    0x18 => reg32 imr {
      16 => wueim, //= Write Update Error interrupt mask
      11 => tbmim, //= Timer B match interrupt mask
      10 => cbeim, //= Timer B capture mode event interrupt mask
      9 => cbmim, //= Timer B capture mode match interrupt mask
      8 => tbtoim, //= Timer B time-out interrupt mask
      4 => tamim, //= Timer A match interrupt mask
      3 => rtcim, //= RTC interrupt mask
      2 => caeim, //= Timer A capture mode event interrupt mask
      1 => camim, //= Timer A capture mode match interrupt mask
      0 => tatoim, //= Timer A time-out interrupt mask
    }
    0x24 => reg32 icr {
      0 => tatocint, //= Timer A time-out raw interrupt
    }
    0x28 => reg32 tailr {
      0..31 => tailr,      //= Timer A interval load
    }
    0x38 => reg32 apr {
      0..15 => psr,        //= Timer A prescale value
                           //= Only 8bit for 16/32bit timers
    }
    0x50 => reg32 tav {
      0..31 => v,          // Timer A counter value
    }
  });

  pub const TIMER_0:   *const Timer = 0x40030000 as *const Timer;
  pub const TIMER_1:   *const Timer = 0x40031000 as *const Timer;
  pub const TIMER_2:   *const Timer = 0x40032000 as *const Timer;
  pub const TIMER_3:   *const Timer = 0x40033000 as *const Timer;
  pub const TIMER_4:   *const Timer = 0x40034000 as *const Timer;
  pub const TIMER_5:   *const Timer = 0x40035000 as *const Timer;

  pub const TIMER_W_0: *const Timer = 0x40036000 as *const Timer;
  pub const TIMER_W_1: *const Timer = 0x40037000 as *const Timer;
  pub const TIMER_W_2: *const Timer = 0x4003C000 as *const Timer;
  pub const TIMER_W_3: *const Timer = 0x4003D000 as *const Timer;
  pub const TIMER_W_4: *const Timer = 0x4003E000 as *const Timer;
  pub const TIMER_W_5: *const Timer = 0x4003F000 as *const Timer;
}
