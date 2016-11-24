#![allow(missing_docs)]
// TODO: add docs

pub trait QuadratureDecoder {
  fn maxpos(&self) -> u32;
  fn set_maxpos(&self, maxpos: u32);
  fn pos(&self) -> u32;
  fn set_pos(&self, pos: u32);
}
