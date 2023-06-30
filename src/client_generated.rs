// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod fastris {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod client {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_MINO_TYPE: i8 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_MINO_TYPE: i8 = 7;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_MINO_TYPE: [MinoType; 7] = [
  MinoType::T,
  MinoType::I,
  MinoType::L,
  MinoType::J,
  MinoType::S,
  MinoType::Z,
  MinoType::O,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct MinoType(pub i8);
#[allow(non_upper_case_globals)]
impl MinoType {
  pub const T: Self = Self(1);
  pub const I: Self = Self(2);
  pub const L: Self = Self(3);
  pub const J: Self = Self(4);
  pub const S: Self = Self(5);
  pub const Z: Self = Self(6);
  pub const O: Self = Self(7);

  pub const ENUM_MIN: i8 = 1;
  pub const ENUM_MAX: i8 = 7;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::T,
    Self::I,
    Self::L,
    Self::J,
    Self::S,
    Self::Z,
    Self::O,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::T => Some("T"),
      Self::I => Some("I"),
      Self::L => Some("L"),
      Self::J => Some("J"),
      Self::S => Some("S"),
      Self::Z => Some("Z"),
      Self::O => Some("O"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for MinoType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for MinoType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for MinoType {
    type Output = MinoType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for MinoType {
  type Scalar = i8;
  #[inline]
  fn to_little_endian(self) -> i8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i8) -> Self {
    let b = i8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for MinoType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for MinoType {}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_PLAYER_ACTIONS: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_PLAYER_ACTIONS: u8 = 7;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_PLAYER_ACTIONS: [PlayerActions; 8] = [
  PlayerActions::NONE,
  PlayerActions::RotateCW,
  PlayerActions::RotateCCW,
  PlayerActions::Rotate180,
  PlayerActions::Hold,
  PlayerActions::HardDrop,
  PlayerActions::SoftDrop,
  PlayerActions::Horizontal,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PlayerActions(pub u8);
#[allow(non_upper_case_globals)]
impl PlayerActions {
  pub const NONE: Self = Self(0);
  pub const RotateCW: Self = Self(1);
  pub const RotateCCW: Self = Self(2);
  pub const Rotate180: Self = Self(3);
  pub const Hold: Self = Self(4);
  pub const HardDrop: Self = Self(5);
  pub const SoftDrop: Self = Self(6);
  pub const Horizontal: Self = Self(7);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 7;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::RotateCW,
    Self::RotateCCW,
    Self::Rotate180,
    Self::Hold,
    Self::HardDrop,
    Self::SoftDrop,
    Self::Horizontal,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::RotateCW => Some("RotateCW"),
      Self::RotateCCW => Some("RotateCCW"),
      Self::Rotate180 => Some("Rotate180"),
      Self::Hold => Some("Hold"),
      Self::HardDrop => Some("HardDrop"),
      Self::SoftDrop => Some("SoftDrop"),
      Self::Horizontal => Some("Horizontal"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for PlayerActions {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for PlayerActions {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for PlayerActions {
    type Output = PlayerActions;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for PlayerActions {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for PlayerActions {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for PlayerActions {}
pub struct PlayerActionsUnionTableOffset {}

pub enum RotateCWOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RotateCW<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RotateCW<'a> {
  type Inner = RotateCW<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> RotateCW<'a> {

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    RotateCW { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    _args: &'args RotateCWArgs
  ) -> flatbuffers::WIPOffset<RotateCW<'bldr>> {
    let mut builder = RotateCWBuilder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for RotateCW<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct RotateCWArgs {
}
impl<'a> Default for RotateCWArgs {
  #[inline]
  fn default() -> Self {
    RotateCWArgs {
    }
  }
}

pub struct RotateCWBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RotateCWBuilder<'a, 'b> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RotateCWBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RotateCWBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<RotateCW<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for RotateCW<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("RotateCW");
      ds.finish()
  }
}
pub enum RotateCCWOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RotateCCW<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RotateCCW<'a> {
  type Inner = RotateCCW<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> RotateCCW<'a> {

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    RotateCCW { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    _args: &'args RotateCCWArgs
  ) -> flatbuffers::WIPOffset<RotateCCW<'bldr>> {
    let mut builder = RotateCCWBuilder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for RotateCCW<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct RotateCCWArgs {
}
impl<'a> Default for RotateCCWArgs {
  #[inline]
  fn default() -> Self {
    RotateCCWArgs {
    }
  }
}

pub struct RotateCCWBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RotateCCWBuilder<'a, 'b> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RotateCCWBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RotateCCWBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<RotateCCW<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for RotateCCW<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("RotateCCW");
      ds.finish()
  }
}
pub enum Rotate180Offset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Rotate180<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Rotate180<'a> {
  type Inner = Rotate180<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Rotate180<'a> {

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Rotate180 { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    _args: &'args Rotate180Args
  ) -> flatbuffers::WIPOffset<Rotate180<'bldr>> {
    let mut builder = Rotate180Builder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for Rotate180<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct Rotate180Args {
}
impl<'a> Default for Rotate180Args {
  #[inline]
  fn default() -> Self {
    Rotate180Args {
    }
  }
}

pub struct Rotate180Builder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> Rotate180Builder<'a, 'b> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> Rotate180Builder<'a, 'b> {
    let start = _fbb.start_table();
    Rotate180Builder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Rotate180<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Rotate180<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Rotate180");
      ds.finish()
  }
}
pub enum HoldOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Hold<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Hold<'a> {
  type Inner = Hold<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Hold<'a> {

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Hold { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    _args: &'args HoldArgs
  ) -> flatbuffers::WIPOffset<Hold<'bldr>> {
    let mut builder = HoldBuilder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for Hold<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct HoldArgs {
}
impl<'a> Default for HoldArgs {
  #[inline]
  fn default() -> Self {
    HoldArgs {
    }
  }
}

pub struct HoldBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HoldBuilder<'a, 'b> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HoldBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HoldBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Hold<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Hold<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Hold");
      ds.finish()
  }
}
pub enum HardDropOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct HardDrop<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for HardDrop<'a> {
  type Inner = HardDrop<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> HardDrop<'a> {

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    HardDrop { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    _args: &'args HardDropArgs
  ) -> flatbuffers::WIPOffset<HardDrop<'bldr>> {
    let mut builder = HardDropBuilder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for HardDrop<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct HardDropArgs {
}
impl<'a> Default for HardDropArgs {
  #[inline]
  fn default() -> Self {
    HardDropArgs {
    }
  }
}

pub struct HardDropBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HardDropBuilder<'a, 'b> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HardDropBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HardDropBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<HardDrop<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for HardDrop<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("HardDrop");
      ds.finish()
  }
}
pub enum SoftDropOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SoftDrop<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SoftDrop<'a> {
  type Inner = SoftDrop<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> SoftDrop<'a> {
  pub const VT_REPEATS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SoftDrop { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args SoftDropArgs
  ) -> flatbuffers::WIPOffset<SoftDrop<'bldr>> {
    let mut builder = SoftDropBuilder::new(_fbb);
    builder.add_repeats(args.repeats);
    builder.finish()
  }


  #[inline]
  pub fn repeats(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(SoftDrop::VT_REPEATS, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for SoftDrop<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u16>("repeats", Self::VT_REPEATS, false)?
     .finish();
    Ok(())
  }
}
pub struct SoftDropArgs {
    pub repeats: u16,
}
impl<'a> Default for SoftDropArgs {
  #[inline]
  fn default() -> Self {
    SoftDropArgs {
      repeats: 0,
    }
  }
}

pub struct SoftDropBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SoftDropBuilder<'a, 'b> {
  #[inline]
  pub fn add_repeats(&mut self, repeats: u16) {
    self.fbb_.push_slot::<u16>(SoftDrop::VT_REPEATS, repeats, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SoftDropBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SoftDropBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SoftDrop<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SoftDrop<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SoftDrop");
      ds.field("repeats", &self.repeats());
      ds.finish()
  }
}
pub enum HorizontalOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Horizontal<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Horizontal<'a> {
  type Inner = Horizontal<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Horizontal<'a> {
  pub const VT_RIGHT: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Horizontal { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args HorizontalArgs
  ) -> flatbuffers::WIPOffset<Horizontal<'bldr>> {
    let mut builder = HorizontalBuilder::new(_fbb);
    builder.add_right(args.right);
    builder.finish()
  }


  #[inline]
  pub fn right(&self) -> i8 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i8>(Horizontal::VT_RIGHT, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Horizontal<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i8>("right", Self::VT_RIGHT, false)?
     .finish();
    Ok(())
  }
}
pub struct HorizontalArgs {
    pub right: i8,
}
impl<'a> Default for HorizontalArgs {
  #[inline]
  fn default() -> Self {
    HorizontalArgs {
      right: 0,
    }
  }
}

pub struct HorizontalBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HorizontalBuilder<'a, 'b> {
  #[inline]
  pub fn add_right(&mut self, right: i8) {
    self.fbb_.push_slot::<i8>(Horizontal::VT_RIGHT, right, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HorizontalBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HorizontalBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Horizontal<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Horizontal<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Horizontal");
      ds.field("right", &self.right());
      ds.finish()
  }
}
pub enum PlayerActionOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct PlayerAction<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for PlayerAction<'a> {
  type Inner = PlayerAction<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> PlayerAction<'a> {
  pub const VT_ACTION_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_ACTION: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    PlayerAction { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PlayerActionArgs
  ) -> flatbuffers::WIPOffset<PlayerAction<'bldr>> {
    let mut builder = PlayerActionBuilder::new(_fbb);
    if let Some(x) = args.action { builder.add_action(x); }
    builder.add_action_type(args.action_type);
    builder.finish()
  }


  #[inline]
  pub fn action_type(&self) -> PlayerActions {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<PlayerActions>(PlayerAction::VT_ACTION_TYPE, Some(PlayerActions::NONE)).unwrap()}
  }
  #[inline]
  pub fn action(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(PlayerAction::VT_ACTION, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn action_as_rotate_cw(&self) -> Option<RotateCW<'a>> {
    if self.action_type() == PlayerActions::RotateCW {
      self.action().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { RotateCW::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn action_as_rotate_ccw(&self) -> Option<RotateCCW<'a>> {
    if self.action_type() == PlayerActions::RotateCCW {
      self.action().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { RotateCCW::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn action_as_rotate_180(&self) -> Option<Rotate180<'a>> {
    if self.action_type() == PlayerActions::Rotate180 {
      self.action().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { Rotate180::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn action_as_hold(&self) -> Option<Hold<'a>> {
    if self.action_type() == PlayerActions::Hold {
      self.action().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { Hold::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn action_as_hard_drop(&self) -> Option<HardDrop<'a>> {
    if self.action_type() == PlayerActions::HardDrop {
      self.action().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { HardDrop::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn action_as_soft_drop(&self) -> Option<SoftDrop<'a>> {
    if self.action_type() == PlayerActions::SoftDrop {
      self.action().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SoftDrop::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn action_as_horizontal(&self) -> Option<Horizontal<'a>> {
    if self.action_type() == PlayerActions::Horizontal {
      self.action().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { Horizontal::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for PlayerAction<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<PlayerActions, _>("action_type", Self::VT_ACTION_TYPE, "action", Self::VT_ACTION, false, |key, v, pos| {
        match key {
          PlayerActions::RotateCW => v.verify_union_variant::<flatbuffers::ForwardsUOffset<RotateCW>>("PlayerActions::RotateCW", pos),
          PlayerActions::RotateCCW => v.verify_union_variant::<flatbuffers::ForwardsUOffset<RotateCCW>>("PlayerActions::RotateCCW", pos),
          PlayerActions::Rotate180 => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Rotate180>>("PlayerActions::Rotate180", pos),
          PlayerActions::Hold => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Hold>>("PlayerActions::Hold", pos),
          PlayerActions::HardDrop => v.verify_union_variant::<flatbuffers::ForwardsUOffset<HardDrop>>("PlayerActions::HardDrop", pos),
          PlayerActions::SoftDrop => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SoftDrop>>("PlayerActions::SoftDrop", pos),
          PlayerActions::Horizontal => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Horizontal>>("PlayerActions::Horizontal", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct PlayerActionArgs {
    pub action_type: PlayerActions,
    pub action: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for PlayerActionArgs {
  #[inline]
  fn default() -> Self {
    PlayerActionArgs {
      action_type: PlayerActions::NONE,
      action: None,
    }
  }
}

pub struct PlayerActionBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PlayerActionBuilder<'a, 'b> {
  #[inline]
  pub fn add_action_type(&mut self, action_type: PlayerActions) {
    self.fbb_.push_slot::<PlayerActions>(PlayerAction::VT_ACTION_TYPE, action_type, PlayerActions::NONE);
  }
  #[inline]
  pub fn add_action(&mut self, action: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PlayerAction::VT_ACTION, action);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PlayerActionBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PlayerActionBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<PlayerAction<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for PlayerAction<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("PlayerAction");
      ds.field("action_type", &self.action_type());
      match self.action_type() {
        PlayerActions::RotateCW => {
          if let Some(x) = self.action_as_rotate_cw() {
            ds.field("action", &x)
          } else {
            ds.field("action", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        PlayerActions::RotateCCW => {
          if let Some(x) = self.action_as_rotate_ccw() {
            ds.field("action", &x)
          } else {
            ds.field("action", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        PlayerActions::Rotate180 => {
          if let Some(x) = self.action_as_rotate_180() {
            ds.field("action", &x)
          } else {
            ds.field("action", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        PlayerActions::Hold => {
          if let Some(x) = self.action_as_hold() {
            ds.field("action", &x)
          } else {
            ds.field("action", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        PlayerActions::HardDrop => {
          if let Some(x) = self.action_as_hard_drop() {
            ds.field("action", &x)
          } else {
            ds.field("action", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        PlayerActions::SoftDrop => {
          if let Some(x) = self.action_as_soft_drop() {
            ds.field("action", &x)
          } else {
            ds.field("action", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        PlayerActions::Horizontal => {
          if let Some(x) = self.action_as_horizontal() {
            ds.field("action", &x)
          } else {
            ds.field("action", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("action", &x)
        },
      };
      ds.finish()
  }
}
}  // pub mod Client
}  // pub mod Fastris

