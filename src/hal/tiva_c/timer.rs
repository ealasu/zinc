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

use hal::timer;
use hal::cortex_m4::nvic;
use hal::tiva_c::sysctl;

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

pub mod timers {
  use super::*;
  use util::support::get_reg_ref;
  use hal::tiva_c::sysctl;
  use hal::pwm::PWMOutput;

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

      impl PWMOutput for $type_name {
        fn set_period_us(&mut self, period_us: u32) {
          self.a_set_interval(self.us_to_ticks(period_us));
        }

        fn get_period_us(&self) -> u32 {
          self.ticks_to_us(self.a_get_interval())
        }

        fn set_pulsewidth_us(&mut self, pulsewidth_us: u32) {
          let v = self.get_period_us() - pulsewidth_us;
          self.regs().tamatchr.set_tamr(self.us_to_ticks(v));
        }

        fn get_pulsewidth_us(&self) -> u32 {
          self.get_period_us() - self.ticks_to_us(self.regs().tamatchr.tamr())
        }
      }

      pub const $name: $type_name = $type_name;
    }
  }

  // There are 6 standard 16/32bit timers and 6 "wide" 32/64bit timers
  // TODO
  timer!(TIMER1, Timer1, reg::TIMER_1, sysctl::periph::timer::TIMER_1, false, 37);
  timer!(TIMER2, Timer2, reg::TIMER_2, sysctl::periph::timer::TIMER_2, false, 39);
  timer!(TIMER3, Timer3, reg::TIMER_3, sysctl::periph::timer::TIMER_3, false, 51);

  timer!(TIMERW0, TimerW0, reg::TIMER_W_0, sysctl::periph::timer::TIMER_W_0, true, 110);
  timer!(TIMERW1, TimerW1, reg::TIMER_W_1, sysctl::periph::timer::TIMER_W_1, true, 112);
  timer!(TIMERW2, TimerW2, reg::TIMER_W_2, sysctl::periph::timer::TIMER_W_2, true, 114);
}

pub trait TivaTimer {
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Timer;
  fn wide(&self) -> bool;
  fn irq_num(&self) -> usize;

  fn ticks_to_us(&self, v: u32) -> u32 {
    // TODO: account for prescaler
    v / (sysctl::clock::sysclk_get() as u32 / 1000000)
  }

  fn us_to_ticks(&self, v: u32) -> u32 {
    // TODO: account for prescaler
    v * (sysctl::clock::sysclk_get() as u32 / 1000000)
  }

  /// Configure timer registers
  /// TODO(simias): Only Periodic and OneShot modes are implemented so far
  fn configure(&self, cfg: reg::Timer_cfg_cfg, mode: Mode) {
    self.periph().ensure_enabled();

    // Make sure the timer is disabled before making changes.
    self.regs().ctl.set_taen(false);
    self.regs().ctl.set_tben(false);

    self.regs().cfg.set_cfg(cfg);

    self.regs().mr[0].set_pwmie(true);
    self.regs().mr[1].set_pwmie(true);

    self.regs().mr[0]
      .set_mr(match mode {
        Mode::OneShot  => reg::Timer_mr_mr::OneShot,
        Mode::Periodic => reg::Timer_mr_mr::Periodic,
        _              => panic!("Unimplemented timer mode"),
      })
      // We need to count down in order for the prescaler to work as a
      // prescaler. If we count up it becomes a timer extension (i.e. it becomes
      // the MSBs of the counter).
      .set_cdir(reg::Timer_mr_cdir::Down)
      // match interrupt enable
      .set_mie(true);

    // Set maximum timeout value to overflow as late as possible
    //self.regs().tailr.set_tailr(0xffffffff);

    // Timer is now configured, we can enable it
    //self.regs().ctl.set_taen(true);
  }

  fn prescale(&self, prescale: u32) {
    if !self.wide() && prescale > 0xffff {
      panic!("prescale is too wide for this timer");
    }
    self.regs().apr.set_psr(prescale as u32);
  }

  fn a_enable_timeout_interrupt(&self) {
    nvic::enable_irq(self.irq_num() - 16);
    self.regs().imr.set_tatoim(true);
  }

  fn a_set_interval(&self, interval: u32) {
    self.regs().tailr.set_tailr(interval);
  }

  fn a_get_interval(&self) -> u32 {
    self.regs().tailr.tailr()
  }

  fn a_clear_interrupt(&self) {
    self.regs().icr.set_tatocint(true);
  }

  fn a_disable(&self) {
    self.regs().ctl.set_taen(false);
  }

  fn a_enable(&self) {
    self.regs().ctl.set_taen(true);
  }

  fn set_counter(&self, value: u32) {
    self.regs().tav.set_v(value);
  }

  fn configure_pwm(&self) {
    self.periph().ensure_enabled();

    // Make sure the timer is disabled before making changes.
    self.regs().ctl.set_taen(false);
    self.regs().ctl.set_tben(false);

    self.regs().cfg.set_cfg(reg::Timer_cfg_cfg::HalfWidth);
    self.regs().mr[0].set_ams(true);
    self.regs().mr[0].set_cmr(false);
    self.regs().mr[0].set_mr(reg::Timer_mr_mr::Periodic);

  }

  fn enable_pwm(&self) {
    self.a_enable();
  }
}

impl<T: TivaTimer> timer::Timer for T {
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
    0x04 => reg32 mr[2] {
      //! Timer A Mode & Timer B mode
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
    0x30 => reg32 tamatchr {
      0..31 => tamr, //= Timer A match register
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
  pub const TIMER_W_2: *const Timer = 0x4004C000 as *const Timer;
  pub const TIMER_W_3: *const Timer = 0x4004D000 as *const Timer;
  pub const TIMER_W_4: *const Timer = 0x4004E000 as *const Timer;
  pub const TIMER_W_5: *const Timer = 0x4004F000 as *const Timer;
}
