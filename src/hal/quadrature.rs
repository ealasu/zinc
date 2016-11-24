#![allow(missing_docs)]
// TODO: add docs

pub trait QuadratureDecoder {
  fn maxpos(&self) -> u32;
  fn set_maxpos(&self, maxpos: u32);
  fn pos(&self) -> u32;
  fn set_pos(&self, pos: u32);
}

impl<'a, T: QuadratureDecoder> QuadratureDecoder for &'a T {
  fn maxpos(&self) -> u32 { (**self).maxpos() }
  fn set_maxpos(&self, maxpos: u32) { (**self).set_maxpos(maxpos) }
  fn pos(&self) -> u32 { (**self).pos() }
  fn set_pos(&self, pos: u32) { (**self).set_pos(pos) }
}
