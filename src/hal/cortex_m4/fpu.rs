//! Interface to the Floating Point Unit

use util::support::get_reg_ref;

fn fpu_reg() -> &'static reg::Fpu {
  get_reg_ref(reg::FPU)
}

/// Enable FPU
pub fn enable() {
  fpu_reg().cpac.set_cp11(reg::Fpu_cpac_cp11::Full);
  fpu_reg().cpac.set_cp10(reg::Fpu_cpac_cp10::Full);
}

/// Enable FPU register stacking.
/// This allows you to use the FPU in an interrupt routine.
pub fn enable_stacking() {
  fpu_reg().fpcc.set_aspen(true);
  fpu_reg().fpcc.set_lspen(false);
}

/// Enable lazy stacking of the FPU registers.
/// This causes the FPU registers to be saved only if they're used.
pub fn enable_lazy_stacking() {
  fpu_reg().fpcc.set_aspen(true);
  fpu_reg().fpcc.set_lspen(true);
}

mod reg {
  use volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(Fpu = {
    0xD88 => reg32 cpac { //! Coprocessor acess control
      23..22 => cp11 {
        0 => Denied,
        1 => Privileged,
        3 => Full,
      }
      21..20 => cp10 {
        0 => Denied,
        1 => Privileged,
        3 => Full,
      }
    }
    0xF34 => reg32 fpcc { //! Floating point context control
      31 => aspen,
      30 => lspen,
      8 => mondry,
      6 => bfrdy,
      5 => mmrdy,
      4 => hfrdy,
      3 => thread,
      1 => user,
      0 => lspact,
    }
  });

  pub const FPU: *const Fpu = 0xE000_E000 as *const Fpu;
}
