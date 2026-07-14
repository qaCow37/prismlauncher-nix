use std::io;
use serde::ser::Impossible;

#[derive(Debug)]
pub enum Error {
	IO(io::Error),
	NonPrimitiveKey,
	Custom(String),
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
	    Self::IO(value)
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::IO(err)         => err.fmt(f),
			Self::NonPrimitiveKey => f.write_str("non primitive key in map"),
			Self::Custom(str)     => f.write_str(str),
		}
	}
}

impl std::error::Error for Error {}
impl serde::ser::Error for Error {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display
	{
		Self::Custom(msg.to_string())
	}
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Serializer<W, F = CompactFormatter> {
	w: W,
	f: F,
}

impl<W: io::Write> Serializer<W> {
	pub fn new(writer: W) -> Self {
		Serializer::with_formatter(
			writer,
			CompactFormatter
		)
	}
}
impl<'a, W: io::Write> Serializer<W, PrettyFormatter<'a>> {
	pub fn pretty(writer: W) -> Self {
		Self::with_formatter(
			writer,
			PrettyFormatter::new(),
		)
	}
	pub fn pretty_with_indent(writer: W, indent: &'a [u8]) -> Self {
		Self::with_formatter(
			writer,
			PrettyFormatter::with_indent(
				indent
			),
		)
	}
}
impl<W: io::Write, F: Formatter> Serializer<W, F> {
	pub fn with_formatter(writer: W, formatter: F) -> Self {
		Self {
			w: writer,
			f: formatter,
		}
	}

	pub fn into_innter(self) -> W {
		self.w
	}
}

impl<'a, W, F> serde::Serializer for &'a mut Serializer<W, F>
where
	W: io::Write,
	F: Formatter,
{
	type Ok = ();
	type Error = Error;
	type SerializeSeq           = Compound<'a, W, F>;
	type SerializeTuple         = Compound<'a, W, F>;
	type SerializeTupleStruct   = Compound<'a, W, F>;
	type SerializeTupleVariant  = Compound<'a, W, F>;
	type SerializeMap           = Compound<'a, W, F>;
	type SerializeStruct        = Compound<'a, W, F>;
	type SerializeStructVariant = Compound<'a, W, F>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
		self.f.write_bool(
			&mut self.w,
			v
		)
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
	    self.f.write_i8(
			&mut self.w,
			v
		)
	}
	fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
	    self.f.write_i16(
			&mut self.w,
			v
		)
	}
	fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
	    self.f.write_i32(
			&mut self.w,
			v
		)
	}
	fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
	    self.f.write_i64(
			&mut self.w,
			v
		)
	}
	fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
	    self.f.write_i128(
			&mut self.w,
			v
		)
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
	    self.f.write_u8(
			&mut self.w,
			v
		)
	}
	fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
	    self.f.write_u16(
			&mut self.w,
			v
		)
	}
	fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
	    self.f.write_u32(
			&mut self.w,
			v
		)
	}
	fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
	    self.f.write_u64(
			&mut self.w,
			v
		)
	}
	fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
	    self.f.write_u128(
			&mut self.w,
			v
		)
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
	    self.f.write_f32(
			&mut self.w,
			v
		)
	}
	fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
	    self.f.write_f64(
			&mut self.w,
			v
		)
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok> {
		self.f.begin_str(&mut self.w)?;
	    self.f.write_char(
			&mut self.w,
			v
		)?;
		self.f.end_str(&mut self.w)
	}
	fn serialize_str(self, v: &str) -> Result<Self::Ok> {
		self.f.begin_str(&mut self.w)?;
	    self.f.write_str(
			&mut self.w,
			v
		)?;
		self.f.end_str(&mut self.w)
	}

	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
		// serializing raw data bytes as list in nix
	    todo!()
	}

	fn serialize_none(self) -> Result<Self::Ok> {
	    self.f.write_null(
			&mut self.w
		)
	}
	fn serialize_some<T>(self, v: &T) -> Result<Self::Ok>
	where
	    T: ?Sized + serde::Serialize
	{
	    v.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok> {
	    self.f.write_null(
			&mut self.w
		)
	}
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
	    self.f.write_null(
			&mut self.w
		)
	}
	fn serialize_unit_variant(
	    self,
	    _name: &'static str,
	    _variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok>
	{
	    self.f.write_null(
			&mut self.w
		)
	}

	fn serialize_newtype_struct<T>(
		self,
		_name: &'static str,
		v: &T,
	) -> Result<Self::Ok>
	where
	    T: ?Sized + serde::Serialize
	{
	    v.serialize(self)
	}
	fn serialize_newtype_variant<T>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
	    v: &T,
	) -> Result<Self::Ok>
	where
	    T: ?Sized + serde::Serialize
	{
	    v.serialize(self)
	}

	fn serialize_tuple(self, _l: usize) -> Result<Self::SerializeTuple> {
		self.f.begin_list(&mut self.w)?;
	    Ok(Compound {
			ser: self,
			state: CompoundState::First,
		})
	}
	fn serialize_tuple_struct(
	    self,
	    _name: &'static str,
	    _len: usize,
	) -> Result<Self::SerializeTupleStruct>
	{
		self.f.begin_list(&mut self.w)?;
		Ok(Compound {
			ser: self,
			state: CompoundState::First,
		})
	}
	fn serialize_tuple_variant(
	    self,
	    _name: &'static str,
	    _variant_index: u32,
	    _variant: &'static str,
	    _len: usize,
	) -> Result<Self::SerializeTupleVariant>
	{
		self.f.begin_list(&mut self.w)?;
		Ok(Compound {
			ser: self,
			state: CompoundState::First,
		})
	}
	fn serialize_seq(self, _l: Option<usize>) -> Result<Self::SerializeSeq> {
		self.f.begin_list(&mut self.w)?;
	    Ok(Compound {
			ser: self,
			state: CompoundState::First,
		})
	}

	fn serialize_struct(
	    self,
	    _name: &'static str,
	    _len: usize,
	) -> Result<Self::SerializeStruct>
	{
		self.f.begin_attrset(&mut self.w)?;
	    Ok(Compound {
			ser: self,
			state: CompoundState::First,
		})
	}
	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant>
	{
		self.f.begin_attrset(&mut self.w)?;
	    Ok(Compound {
			ser: self,
			state: CompoundState::First,
		})
	}
	fn serialize_map(self, _l: Option<usize>) -> Result<Self::SerializeMap> {
		self.f.begin_attrset(&mut self.w)?;
	    Ok(Compound {
			ser: self,
			state: CompoundState::First,
		})
	}

}

#[derive(PartialEq)]
enum CompoundState {
	First,
	Rest,
}
pub struct Compound<'a, W, F> {
	ser: &'a mut Serializer<W, F>,
	state: CompoundState,
}

impl<'a, W, F> serde::ser::SerializeTuple for Compound<'a, W, F>
where
	W: io::Write,
	F: Formatter
{
	type Ok = ();
	type Error = Error;

	fn serialize_element<T>(&mut self, value: &T) -> Result<()>
	where
	    T: ?Sized + serde::Serialize
	{
		serde::ser::SerializeSeq::serialize_element(
			self,
			value
		)
	}
	fn end(self) -> Result<Self::Ok> {
		serde::ser::SerializeSeq::end(
			self
		)
	}
}
impl<'a, W, F> serde::ser::SerializeTupleStruct for Compound<'a, W, F>
where
	W: io::Write,
	F: Formatter,
{
	type Ok = ();
	type Error = Error;

	fn serialize_field<T>(&mut self, value: &T) -> Result<()>
	where
	    T: ?Sized + serde::Serialize
	{
		serde::ser::SerializeTuple::serialize_element(
			self,
			value
		)
	}
	fn end(self) -> Result<Self::Ok> {
		serde::ser::SerializeTuple::end(
			self
		)
	}
}
impl<'a, W, F> serde::ser::SerializeTupleVariant for Compound<'a, W, F>
where
	W: io::Write,
	F: Formatter,
{
	type Ok = ();
	type Error = Error;

	fn serialize_field<T>(&mut self, value: &T) -> Result<()>
	where
	    T: ?Sized + serde::Serialize
	{
		serde::ser::SerializeTuple::serialize_element(
			self,
			value
		)
	}
	fn end(self) -> Result<Self::Ok> {
		serde::ser::SerializeTuple::end(
			self
		)
	}
}
impl<'a, W, F> serde::ser::SerializeSeq for Compound<'a, W, F>
where
	W: io::Write,
	F: Formatter,
{
	type Ok = ();
	type Error = Error;

	fn serialize_element<T>(&mut self, value: &T) -> Result<()>
	where
	    T: ?Sized + serde::Serialize
	{
		let first = self.state == CompoundState::First;
		self.state = CompoundState::Rest;

		self.ser.f.begin_list_val(
			&mut self.ser.w,
			first
		).map_err(|err| Into::<Self::Error>::into(err))?;

		value.serialize(&mut *self.ser)?;

		self.ser.f.end_list_val(
			&mut self.ser.w,
			first
		).map_err(|err| Into::<Self::Error>::into(err))?;

		Ok(())
	}
	fn end(self) -> Result<Self::Ok> {
		self.ser.f.end_list(
			&mut self.ser.w
		)
	}
}
impl<'a, W, F> serde::ser::SerializeStruct for Compound<'a, W, F>
where
	W: io::Write,
	F: Formatter,
{
	type Ok = ();
	type Error = Error;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
	where
	    T: ?Sized + serde::Serialize
	{
		// Key
		self.ser.f.begin_attr_key_idf(&mut self.ser.w)?;
		self.ser.f.write_str(
			&mut self.ser.w,
			key
		)?;
		self.ser.f.end_attr_key_idf(&mut self.ser.w)?;

		// Value
		self.ser.f.begin_attr_val(&mut self.ser.w)?;
		value.serialize(&mut *self.ser)?;
		self.ser.f.end_attr_val(&mut self.ser.w)?;

		Ok(())
	}
	fn end(self) -> Result<Self::Ok> {
		self.ser.f.end_attrset(
			&mut self.ser.w
		)
	}
}
impl<'a, W, F> serde::ser::SerializeStructVariant for Compound<'a, W, F>
where
	W: io::Write,
	F: Formatter,
{
	type Ok = ();
	type Error = Error;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
	where
	    T: ?Sized + serde::Serialize
	{
		serde::ser::SerializeStruct::serialize_field(
			self,
			key,
			value
		)
	}
	fn end(self) -> Result<Self::Ok> {
		serde::ser::SerializeStruct::end(
			self
		)
	}
}
impl<'a, W, F> serde::ser::SerializeMap for Compound<'a, W, F>
where
	W: io::Write,
	F: Formatter,
{
	type Ok = ();
	type Error = Error;

	fn serialize_key<T>(&mut self, key: &T) -> Result<()>
	where
	    T: ?Sized + serde::Serialize
	{
		self.ser.f.begin_attr_key_lit(&mut self.ser.w)?;
		key.serialize(MapKeySerializer {
			ser: self.ser
		})?;
		self.ser.f.end_attr_key_lit(&mut self.ser.w)
	}
	fn serialize_value<T>(&mut self, value: &T) -> Result<()>
	where
	    T: ?Sized + serde::Serialize
	{
		self.ser.f.begin_attr_val(&mut self.ser.w)?;
		value.serialize(&mut *self.ser)?;
		self.ser.f.end_attr_val(&mut self.ser.w)
	}
	fn end(self) -> Result<Self::Ok> {
		self.ser.f.end_attrset(
			&mut self.ser.w
		)
	}
}

struct MapKeySerializer<'a, W, F> {
	ser: &'a mut Serializer<W, F>,
}
impl<'a, W, F> serde::Serializer for MapKeySerializer<'a, W, F>
where
	W: io::Write,
	F: Formatter
{
	type Ok = ();
	type Error = Error;
	type SerializeSeq           = Impossible<(), Error>;
	type SerializeMap           = Impossible<(), Error>;
	type SerializeTuple         = Impossible<(), Error>;
	type SerializeStruct        = Impossible<(), Error>;
	type SerializeTupleStruct   = Impossible<(), Error>;
	type SerializeTupleVariant  = Impossible<(), Error>;
	type SerializeStructVariant = Impossible<(), Error>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
		self.ser.f.write_bool(
			&mut self.ser.w,
			v
		)
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
		self.ser.f.write_i8(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
	    self.ser.f.write_i16(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
	     self.ser.f.write_i32(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
	     self.ser.f.write_i64(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
	    self.ser.f.write_i128(
			&mut self.ser.w,
			v
		)
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
	    self.ser.f.write_u8(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
	    self.ser.f.write_u16(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
	    self.ser.f.write_u32(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
	    self.ser.f.write_u64(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
	    self.ser.f.write_u128(
			&mut self.ser.w,
			v
		)
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
		self.ser.f.write_f32(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
	    self.ser.f.write_f64(
			&mut self.ser.w,
			v
		)
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok> {
	    self.ser.f.write_char(
			&mut self.ser.w,
			v
		)
	}
	fn serialize_str(self, v: &str) -> Result<Self::Ok> {
	    self.ser.f.write_str(
			&mut self.ser.w,
			v
		)
	}

	fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok> {
		Err(Error::NonPrimitiveKey)
	}

	fn serialize_unit(self) -> Result<Self::Ok> {
	    Err(Error::NonPrimitiveKey)
	}
	fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok> {
	    Err(Error::NonPrimitiveKey)
	}
	fn serialize_unit_variant(
	    self,
	    _: &'static str,
	    _: u32,
	    _: &'static str,
	) -> Result<Self::Ok>
	{
		Err(Error::NonPrimitiveKey)
	}

	fn serialize_none(self) -> Result<Self::Ok> {
	    Err(Error::NonPrimitiveKey)
	}
	fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
	where
	    T: ?Sized + serde::Serialize
	{
	    value.serialize(
			self
		)
	}

	fn serialize_newtype_struct<T>(
	    self,
	    _name: &'static str,
	    value: &T,
	) -> Result<Self::Ok>
	where
	    T: ?Sized + serde::Serialize
	{
	    value.serialize(
			self
		)
	}
	fn serialize_newtype_variant<T>(
	    self,
	    _name: &'static str,
	    _variant_index: u32,
	    _variant: &'static str,
	    value: &T,
	) -> Result<Self::Ok>
	where
	    T: ?Sized + serde::Serialize
	{
	    value.serialize(
			self
		)
	}

	fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
	    Err(Error::NonPrimitiveKey)
	}
	fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
	    Err(Error::NonPrimitiveKey)
	}
	fn serialize_tuple_struct(
	    self,
	    _: &'static str,
	    _: usize,
	) -> Result<Self::SerializeTupleStruct>
	{
		Err(Error::NonPrimitiveKey)
	}
	fn serialize_tuple_variant(
	    self,
	    _: &'static str,
	    _: u32,
	    _: &'static str,
	    _: usize,
	) -> Result<Self::SerializeTupleVariant>
	{
	    Err(Error::NonPrimitiveKey)
	}

	fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
	    Err(Error::NonPrimitiveKey)
	}
	fn serialize_struct(
	    self,
	    _: &'static str,
	    _: usize,
	) -> Result<Self::SerializeStruct>
	{
	    Err(Error::NonPrimitiveKey)
	}
	fn serialize_struct_variant(
	    self,
	    _: &'static str,
	    _: u32,
	    _: &'static str,
	    _: usize,
	) -> Result<Self::SerializeStructVariant>
	{
	    Err(Error::NonPrimitiveKey)
	}
}

pub trait Formatter {
	fn write_null<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"null")
			.map_err(|e| e.into())
	}

	fn write_bool<W>(&mut self, w: &mut W, v: bool) -> Result<()>
	where
		W: io::Write
	{
		w.write_all({
			if v {
				b"true"
			}
			else {
				b"false"
			}
		}).map_err(|e| e.into())
	}

	fn write_i8<W>(&mut self, w: &mut W, v: i8) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_i16<W>(&mut self, w: &mut W, v: i16) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_i32<W>(&mut self, w: &mut W, v: i32) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_i64<W>(&mut self, w: &mut W, v: i64) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_i128<W>(&mut self, w: &mut W, v: i128) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}

	fn write_u8<W>(&mut self, w: &mut W, v: u8) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_u16<W>(&mut self, w: &mut W, v: u16) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_u32<W>(&mut self, w: &mut W, v: u32) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_u64<W>(&mut self, w: &mut W, v: u64) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_u128<W>(&mut self, w: &mut W, v: u128) -> Result<()>
	where
		W: io::Write
	{
		let mut b = itoa::Buffer::new();
		let s = b.format(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}

	fn write_f32<W>(&mut self, w: &mut W, v: f32) -> Result<()>
	where
		W: io::Write
	{
		let mut b = zmij::Buffer::new();
		let s = b.format_finite(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}
	fn write_f64<W>(&mut self, w: &mut W, v: f64) -> Result<()>
	where
		W: io::Write
	{
		let mut b = zmij::Buffer::new();
		let s = b.format_finite(v);
		w.write_all(s.as_bytes())
			.map_err(|e| e.into())
	}

	fn write_char<W>(&mut self, w: &mut W, v: char) -> Result<()>
	where
		W: io::Write
	{
		match v {
			'\n' => self.write_escaped_str(w, "\\n"),
			'\t' => self.write_escaped_str(w, "\\t"),
			'\r' => self.write_escaped_str(w, "\\r"),
			'\"' => self.write_escaped_str(w, "\\\""),
			'\\' => self.write_escaped_str(w, "\\\\"),
			v => {
				let mut b = [0; 4];
				self.write_escaped_str(
					w,
					v.encode_utf8(&mut b)
				)
			},
		}
	}
	fn write_str<W>(&mut self, w: &mut W, v: &str) -> Result<()>
	where
		W: io::Write
	{
		let bytes = v.as_bytes();
		let mut res = Vec::with_capacity(bytes.len());

		let mut i = 0;
		while i < bytes.len() {
			match bytes[i] {
				b'\n' => res.extend_from_slice(b"\\n"),
				b'\t' => res.extend_from_slice(b"\\t"),
				b'\r' => res.extend_from_slice(b"\\r"),
				b'"'  => res.extend_from_slice(b"\\\""),
				b'\\' => res.extend_from_slice(b"\\\\"),
				b'$' if bytes.get(i + 1) == Some(&b'{') => {
					res.extend_from_slice(b"\\${");
					// skip next '{' char
					i += 1;
				},
				b => res.push(b),
			}
			i += 1;
		}
		self.write_escaped_str(w, unsafe {
			// we either copied all bytes 1:1
			// or only wrote correct utf8 chars into it
			str::from_utf8_unchecked(&res)
		})
	}
	fn write_escaped_str<W>(&mut self, w: &mut W, v: &str) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(v.as_bytes())
			.map_err(|e| e.into())
	}
	fn begin_str<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"\"")
			.map_err(|e| e.into())
	}
	fn end_str<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"\"")
			.map_err(|e| e.into())
	}

	fn begin_list<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"[")
			.map_err(|e| e.into())
	}
	fn begin_list_val<W>(&mut self, w: &mut W, first: bool) -> Result<()>
	where
		W: io::Write
	{
		if first {
			Ok(())
		} else {
			w.write_all(b" ")
				.map_err(|e| e.into())
		}
	}
	#[allow(unused_variables)]
	fn end_list_val<W>(&mut self, w: &mut W, first: bool) -> Result<()>
	where
		W: io::Write
	{
		Ok(())
	}
	fn end_list<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"]")
			.map_err(|e| e.into())
	}

	fn begin_attrset<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"{")
			.map_err(|e| e.into())
	}
	#[allow(unused_variables)]
	fn begin_attr_key_idf<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		Ok(())
	}
	#[allow(unused_variables)]
	fn end_attr_key_idf<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		Ok(())
	}
	fn begin_attr_key_lit<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"\"")
			.map_err(|e| e.into())
	}
	fn end_attr_key_lit<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"\"")
			.map_err(|e| e.into())
	}
	fn begin_attr_val<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"=")
			.map_err(|e| e.into())
	}
	fn end_attr_val<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b";")
			.map_err(|e| e.into())
	}
	fn end_attrset<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		w.write_all(b"}")
			.map_err(|e| e.into())
	}

}

#[derive(Clone, Debug)]
pub struct CompactFormatter;
impl Formatter for CompactFormatter {}

#[derive(Clone, Debug)]
pub struct PrettyFormatter<'a> {
	cur_idn: usize,
	has_val: bool,
	idn_str: &'a [u8],
}

impl<'a> PrettyFormatter<'a> {
	pub fn new() -> Self {
		Self::with_indent(
			b"  "
		)
	}
	pub fn with_indent(indent: &'a [u8]) -> Self {
		Self {
			cur_idn: 0,
			has_val: false,
			idn_str: indent,
		}
	}

	fn do_indent<W: io::Write>(&self, w: &mut W) -> Result<()> {
		for _ in 0..self.cur_idn {
			w.write_all(self.idn_str)?;
		}
		Ok(())
	}
	fn do_newline<W: io::Write>(&self, w: &mut W) -> Result<()> {
		w.write_all(b"\n")
			.map_err(|e| e.into())
	}

	fn do_begin_struct<W: io::Write>(&mut self, w: &mut W, str: &[u8]) -> Result<()> {
		self.cur_idn += 1;
		w.write_all(str)
			.map_err(|e| e.into())
	}
	fn do_end_struct<W: io::Write>(&mut self, w: &mut W, str: &[u8]) -> Result<()> {
		self.cur_idn -= 1;

		if self.has_val {
			self.do_newline(w)?;
			self.do_indent(w)?;
			self.has_val = false;
		}

		w.write_all(str)
			.map_err(|e| e.into())
	}
}

impl<'a> Formatter for PrettyFormatter<'a> {
	fn begin_list<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		self.do_begin_struct(
			w,
			b"["
		)
	}
	fn begin_list_val<W>(&mut self, w: &mut W, _first: bool) -> Result<()>
	where
		W: io::Write
	{
		self.do_newline(w)?;
		self.do_indent(w)
	}
	fn end_list_val<W>(&mut self, _w: &mut W, _first: bool) -> Result<()>
		where
			W: io::Write
	{
	    self.has_val = true;
		Ok(())
	}
	fn end_list<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		self.do_end_struct(
			w,
			b"]"
		)
	}

	fn begin_attrset<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		self.do_begin_struct(
			w,
			b"{"
		)
	}
	fn begin_attr_key_idf<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		self.do_newline(w)?;
		self.do_indent(w)
	}
	fn end_attr_key_idf<W>(&mut self, _w: &mut W) -> Result<()>
	where
		W: io::Write
	{
	    Ok(())
	}
	fn begin_attr_key_lit<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		self.do_newline(w)?;
		self.do_indent(w)?;
		w.write_all(b"\"")
			.map_err(|e| e.into())
	}
	fn end_attr_key_lit<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
	    w.write_all(b"\"")
			.map_err(|e| e.into())
	}
	fn begin_attr_val<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
	    w.write_all(b" = ")
			.map_err(|e| e.into())
	}
	fn end_attr_val<W>(&mut self, w: &mut W) -> Result<()>
		where
			W: io::Write
	{
	    self.has_val = true;
		w.write_all(b";")
			.map_err(|e| e.into())
	}
	fn end_attrset<W>(&mut self, w: &mut W) -> Result<()>
	where
		W: io::Write
	{
		self.do_end_struct(
			w,
			b"}"
		)
	}
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
	W: io::Write,
	T: ?Sized + serde::Serialize,
{
	let mut ser = Serializer::new(writer);
	value.serialize(&mut ser)
}
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> Result<()>
where
	W: io::Write,
	T: ?Sized + serde::Serialize,
{
	let mut ser = Serializer::pretty(writer);
	value.serialize(&mut ser)
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
	T: ?Sized + serde::Serialize
{
	let mut w = Vec::with_capacity(128);
	to_writer(&mut w, value)?;
	Ok(w)
}
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>
where
	T: ?Sized + serde::Serialize
{
	let mut w = Vec::with_capacity(128);
	to_writer(&mut w, value)?;
	Ok(w)
}

pub fn to_string<T>(value: &T) -> Result<String>
where
	T: ?Sized + serde::Serialize
{
	let v = to_vec(value)?;
	Ok(unsafe {
		// We do not write invalid utf8 chars into the Vec
		String::from_utf8_unchecked(
			v
		)
	})
}
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
	T: ?Sized + serde::Serialize
{
	let v = to_vec_pretty(value)?;
	Ok(unsafe {
		// We do not write invalid utf8 chars into the Vec
		String::from_utf8_unchecked(
			v
		)
	})
}
