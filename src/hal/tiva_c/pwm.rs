#![allow(missing_docs)]

use hal::tiva_c::sysctl;
use util::support::get_reg_ref;
use hal::cortex_m4::nvic;


macro_rules! pwm_gen {
  ($name:ident : $type_name:ident, regs = $regs:expr, periph = $periph:expr) => {
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

    //impl timer::Pwm for $type_name {
      ///// Retrieve the current timer value
      //#[inline(always)]
      //fn get_counter(&self) -> u32 {
        //// We count down, however the trait code expects that the counter increases,
        //// so we just complement the value to get an increasing counter.
        // !self.regs().tav.v()
      //}
    //}

    pub const $name: $type_name = $type_name;
  }
}


pwm!(PWM1 : Pwm1, regs = reg::TIMER_1, periph = sysctl::periph::timer::TIMER_1);


pub trait TivaTimer {
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Timer;
  fn wide(&self) -> bool;
  fn irq_num(&self) -> usize;

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

  fn a_clear_interrupt(&self) {
    self.regs().icr.set_tatocint(true);
  }

  fn a_disable(&self) {
    self.regs().ctl.set_taen(false);
  }

  fn a_enable(&self) {
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
