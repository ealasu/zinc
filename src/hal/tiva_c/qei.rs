#![allow(missing_docs)]

use hal::tiva_c::sysctl;
use hal::tiva_c::pin::Pin;
use hal::quadrature::QuadratureDecoder;

pub mod qeis {
  use util::support::get_reg_ref;
  use hal::tiva_c::sysctl;
  use hal::tiva_c::pin::pins::*;
  use super::*;

  macro_rules! qei {
    (
      $name:ident : $type_name:ident,
      periph=$periph:expr, regs=$regs:expr,
      pin_a=$pin_a:ident, pin_a_fn=$pin_a_fn:expr,
      pin_b=$pin_b:ident, pin_b_fn=$pin_b_fn:expr
    ) => {
      #[derive(Clone, Copy)]
      pub struct $type_name;

      impl Qei for $type_name {
        type PinA = $pin_a;
        type PinB = $pin_b;

        fn periph(&self) -> sysctl::periph::PeripheralClock { $periph }
        fn regs(&self) -> &'static reg::Qei { get_reg_ref($regs) }
        fn pin_a(&self) -> Self::PinA { $pin_a }
        fn pin_a_function(&self) -> u8 { $pin_a_fn }
        fn pin_b(&self) -> Self::PinB { $pin_b }
        fn pin_b_function(&self) -> u8 { $pin_b_fn }
      }

      pub const $name: $type_name = $type_name;
    }
  }

  qei!(QEI0: Qei0, periph=sysctl::periph::qei::QEI_0, regs=reg::QEI_0,
       pin_a=PinD6, pin_a_fn=6, pin_b=PinD7, pin_b_fn=6);
  qei!(QEI1: Qei1, periph=sysctl::periph::qei::QEI_1, regs=reg::QEI_1,
       pin_a=PinC5, pin_a_fn=6, pin_b=PinC6, pin_b_fn=6);
}

pub trait Qei {
  type PinA: Pin;
  type PinB: Pin;
  
  fn periph(&self) -> sysctl::periph::PeripheralClock;
  fn regs(&self) -> &'static reg::Qei;
  fn pin_a(&self) -> Self::PinA;
  fn pin_a_function(&self) -> u8;
  fn pin_b(&self) -> Self::PinB;
  fn pin_b_function(&self) -> u8;

  fn configure(&self) {
    self.periph().ensure_enabled();
    self.pin_a().configure(self.pin_a_function());
    self.pin_b().configure(self.pin_b_function());
    self.disable();

    self.regs().ctl.set_swap(reg::Qei_ctl_swap::NoSwap);
    self.regs().ctl.set_sigmode(reg::Qei_ctl_sigmode::Quadrature);
    self.regs().ctl.set_capmode(reg::Qei_ctl_capmode::AB);
    self.regs().ctl.set_resmode(reg::Qei_ctl_resmode::NoReset);
    self.regs().maxpos.set_maxpos(u32::max_value());

    self.enable();
  }

  fn disable(&self) {
    self.regs().ctl.set_enable(false);
  }

  fn enable(&self) {
    self.regs().ctl.set_enable(true);
  }

  fn enable_input_filter(&self) {
    self.regs().ctl.set_filten(true);
  }

  fn disable_input_filter(&self) {
    self.regs().ctl.set_filten(false);
  }
}

impl<T: Qei> QuadratureDecoder for T {
  fn maxpos(&self) -> u32 {
    self.regs().maxpos.maxpos()
  }

  fn set_maxpos(&self, maxpos: u32) {
    self.regs().maxpos.set_maxpos(maxpos);
  }

  fn pos(&self) -> u32 {
    self.regs().pos.pos()
  }

  fn set_pos(&self, pos: u32) {
    self.regs().pos.set_pos(pos);
  }
}


pub mod reg {
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(Qei = {
    0x00 => reg32 ctl {
      19..16 => filtcnt,
      13 => filten,
      12 => stallen,
      11 => invi,
      10 => invb,
      9 => inva,
      8..6 => veldiv,
      5 => velen,
      4 => resmode {
        0 => NoReset,
        1 => ResetIdx,
      }
      3 => capmode {
        0 => A,
        1 => AB,
      }
      2 => sigmode {
        0 => Quadrature,
        1 => ClockDir,
      }
      1 => swap {
        0 => NoSwap,
        1 => Swap,
      }
      0 => enable,
    }
    0x08 => reg32 pos {
      31..0 => pos
    }
    0x0C => reg32 maxpos {
      31..0 => maxpos
    }
  });

  pub const QEI_0: *const Qei = 0x4002_C000 as *const Qei;
  pub const QEI_1: *const Qei = 0x4002_D000 as *const Qei;
}
