#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::fmt::Formatter;
    use core::fmt::Debug;
    use core::fmt::Result;
    use core::fmt::DebugList;

    use collections::vec::Vec;

    struct A(Vec<T>);

    // pub struct Formatter<'a> {
    //     flags: u32,
    //     fill: char,
    //     align: rt::v1::Alignment,
    //     width: Option<usize>,
    //     precision: Option<usize>,
    //
    //     buf: &'a mut (Write+'a),
    //     curarg: slice::Iter<'a, ArgumentV1<'a>>,
    //     args: &'a [ArgumentV1<'a>],
    // }

    // impl<'a> Formatter<'a> {
    //
    //     // First up is the collection of functions used to execute a format string
    //     // at runtime. This consumes all of the compile-time statics generated by
    //     // the format! syntax extension.
    //     fn run(&mut self, arg: &rt::v1::Argument) -> Result {
    //         // Fill in the format parameters into the formatter
    //         self.fill = arg.format.fill;
    //         self.align = arg.format.align;
    //         self.flags = arg.format.flags;
    //         self.width = self.getcount(&arg.format.width);
    //         self.precision = self.getcount(&arg.format.precision);
    //
    //         // Extract the correct argument
    //         let value = match arg.position {
    //             rt::v1::Position::Next => { *self.curarg.next().unwrap() }
    //             rt::v1::Position::At(i) => self.args[i],
    //         };
    //
    //         // Then actually do some printing
    //         (value.formatter)(value.value, self)
    //     }
    //
    //     fn getcount(&mut self, cnt: &rt::v1::Count) -> Option<usize> {
    //         match *cnt {
    //             rt::v1::Count::Is(n) => Some(n),
    //             rt::v1::Count::Implied => None,
    //             rt::v1::Count::Param(i) => {
    //                 self.args[i].as_usize()
    //             }
    //             rt::v1::Count::NextParam => {
    //                 self.curarg.next().and_then(|arg| arg.as_usize())
    //             }
    //         }
    //     }
    //
    //     // Helper methods used for padding and processing formatting arguments that
    //     // all formatting traits can use.
    //
    //     /// Performs the correct padding for an integer which has already been
    //     /// emitted into a str. The str should *not* contain the sign for the
    //     /// integer, that will be added by this method.
    //     ///
    //     /// # Arguments
    //     ///
    //     /// * is_positive - whether the original integer was positive or not.
    //     /// * prefix - if the '#' character (Alternate) is provided, this
    //     ///   is the prefix to put in front of the number.
    //     /// * buf - the byte array that the number has been formatted into
    //     ///
    //     /// This function will correctly account for the flags provided as well as
    //     /// the minimum width. It will not take precision into account.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn pad_integral(&mut self,
    //                         is_positive: bool,
    //                         prefix: &str,
    //                         buf: &str)
    //                         -> Result {
    //         use char::CharExt;
    //
    //         let mut width = buf.len();
    //
    //         let mut sign = None;
    //         if !is_positive {
    //             sign = Some('-'); width += 1;
    //         } else if self.flags & (1 << (FlagV1::SignPlus as u32)) != 0 {
    //             sign = Some('+'); width += 1;
    //         }
    //
    //         let mut prefixed = false;
    //         if self.flags & (1 << (FlagV1::Alternate as u32)) != 0 {
    //             prefixed = true; width += prefix.char_len();
    //         }
    //
    //         // Writes the sign if it exists, and then the prefix if it was requested
    //         let write_prefix = |f: &mut Formatter| {
    //             if let Some(c) = sign {
    //                 let mut b = [0; 4];
    //                 let n = c.encode_utf8(&mut b).unwrap_or(0);
    //                 let b = unsafe { str::from_utf8_unchecked(&b[..n]) };
    //                 try!(f.buf.write_str(b));
    //             }
    //             if prefixed { f.buf.write_str(prefix) }
    //             else { Ok(()) }
    //         };
    //
    //         // The `width` field is more of a `min-width` parameter at this point.
    //         match self.width {
    //             // If there's no minimum length requirements then we can just
    //             // write the bytes.
    //             None => {
    //                 try!(write_prefix(self)); self.buf.write_str(buf)
    //             }
    //             // Check if we're over the minimum width, if so then we can also
    //             // just write the bytes.
    //             Some(min) if width >= min => {
    //                 try!(write_prefix(self)); self.buf.write_str(buf)
    //             }
    //             // The sign and prefix goes before the padding if the fill character
    //             // is zero
    //             Some(min) if self.flags & (1 << (FlagV1::SignAwareZeroPad as u32)) != 0 => {
    //                 self.fill = '0';
    //                 try!(write_prefix(self));
    //                 self.with_padding(min - width, Alignment::Right, |f| {
    //                     f.buf.write_str(buf)
    //                 })
    //             }
    //             // Otherwise, the sign and prefix goes after the padding
    //             Some(min) => {
    //                 self.with_padding(min - width, Alignment::Right, |f| {
    //                     try!(write_prefix(f)); f.buf.write_str(buf)
    //                 })
    //             }
    //         }
    //     }
    //
    //     /// This function takes a string slice and emits it to the internal buffer
    //     /// after applying the relevant formatting flags specified. The flags
    //     /// recognized for generic strings are:
    //     ///
    //     /// * width - the minimum width of what to emit
    //     /// * fill/align - what to emit and where to emit it if the string
    //     ///                provided needs to be padded
    //     /// * precision - the maximum length to emit, the string is truncated if it
    //     ///               is longer than this length
    //     ///
    //     /// Notably this function ignored the `flag` parameters
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn pad(&mut self, s: &str) -> Result {
    //         // Make sure there's a fast path up front
    //         if self.width.is_none() && self.precision.is_none() {
    //             return self.buf.write_str(s);
    //         }
    //         // The `precision` field can be interpreted as a `max-width` for the
    //         // string being formatted
    //         match self.precision {
    //             Some(max) => {
    //                 // If there's a maximum width and our string is longer than
    //                 // that, then we must always have truncation. This is the only
    //                 // case where the maximum length will matter.
    //                 let char_len = s.char_len();
    //                 if char_len >= max {
    //                     let nchars = ::cmp::min(max, char_len);
    //                     return self.buf.write_str(s.slice_chars(0, nchars));
    //                 }
    //             }
    //             None => {}
    //         }
    //         // The `width` field is more of a `min-width` parameter at this point.
    //         match self.width {
    //             // If we're under the maximum length, and there's no minimum length
    //             // requirements, then we can just emit the string
    //             None => self.buf.write_str(s),
    //             // If we're under the maximum width, check if we're over the minimum
    //             // width, if so it's as easy as just emitting the string.
    //             Some(width) if s.char_len() >= width => {
    //                 self.buf.write_str(s)
    //             }
    //             // If we're under both the maximum and the minimum width, then fill
    //             // up the minimum width with the specified string + some alignment.
    //             Some(width) => {
    //                 self.with_padding(width - s.char_len(), Alignment::Left, |me| {
    //                     me.buf.write_str(s)
    //                 })
    //             }
    //         }
    //     }
    //
    //     /// Runs a callback, emitting the correct padding either before or
    //     /// afterwards depending on whether right or left alignment is requested.
    //     fn with_padding<F>(&mut self, padding: usize, default: Alignment,
    //                        f: F) -> Result
    //         where F: FnOnce(&mut Formatter) -> Result,
    //     {
    //         use char::CharExt;
    //         let align = match self.align {
    //             Alignment::Unknown => default,
    //             _ => self.align
    //         };
    //
    //         let (pre_pad, post_pad) = match align {
    //             Alignment::Left => (0, padding),
    //             Alignment::Right | Alignment::Unknown => (padding, 0),
    //             Alignment::Center => (padding / 2, (padding + 1) / 2),
    //         };
    //
    //         let mut fill = [0; 4];
    //         let len = self.fill.encode_utf8(&mut fill).unwrap_or(0);
    //         let fill = unsafe { str::from_utf8_unchecked(&fill[..len]) };
    //
    //         for _ in 0..pre_pad {
    //             try!(self.buf.write_str(fill));
    //         }
    //
    //         try!(f(self));
    //
    //         for _ in 0..post_pad {
    //             try!(self.buf.write_str(fill));
    //         }
    //
    //         Ok(())
    //     }
    //
    //     /// Takes the formatted parts and applies the padding.
    //     /// Assumes that the caller already has rendered the parts with required precision,
    //     /// so that `self.precision` can be ignored.
    //     fn pad_formatted_parts(&mut self, formatted: &flt2dec::Formatted) -> Result {
    //         if let Some(mut width) = self.width {
    //             // for the sign-aware zero padding, we render the sign first and
    //             // behave as if we had no sign from the beginning.
    //             let mut formatted = formatted.clone();
    //             let mut align = self.align;
    //             let old_fill = self.fill;
    //             if self.flags & (1 << (FlagV1::SignAwareZeroPad as u32)) != 0 {
    //                 // a sign always goes first
    //                 let sign = unsafe { str::from_utf8_unchecked(formatted.sign) };
    //                 try!(self.buf.write_str(sign));
    //
    //                 // remove the sign from the formatted parts
    //                 formatted.sign = b"";
    //                 width = if width < sign.len() { 0 } else { width - sign.len() };
    //                 align = Alignment::Right;
    //                 self.fill = '0';
    //             }
    //
    //             // remaining parts go through the ordinary padding process.
    //             let len = formatted.len();
    //             let ret = if width <= len { // no padding
    //                 self.write_formatted_parts(&formatted)
    //             } else {
    //                 self.with_padding(width - len, align, |f| {
    //                     f.write_formatted_parts(&formatted)
    //                 })
    //             };
    //             self.fill = old_fill;
    //             ret
    //         } else {
    //             // this is the common case and we take a shortcut
    //             self.write_formatted_parts(formatted)
    //         }
    //     }
    //
    //     fn write_formatted_parts(&mut self, formatted: &flt2dec::Formatted) -> Result {
    //         fn write_bytes(buf: &mut Write, s: &[u8]) -> Result {
    //             buf.write_str(unsafe { str::from_utf8_unchecked(s) })
    //         }
    //
    //         if !formatted.sign.is_empty() {
    //             try!(write_bytes(self.buf, formatted.sign));
    //         }
    //         for part in formatted.parts {
    //             match *part {
    //                 flt2dec::Part::Zero(mut nzeroes) => {
    //                     const ZEROES: &'static str = // 64 zeroes
    //                         "0000000000000000000000000000000000000000000000000000000000000000";
    //                     while nzeroes > ZEROES.len() {
    //                         try!(self.buf.write_str(ZEROES));
    //                         nzeroes -= ZEROES.len();
    //                     }
    //                     if nzeroes > 0 {
    //                         try!(self.buf.write_str(&ZEROES[..nzeroes]));
    //                     }
    //                 }
    //                 flt2dec::Part::Num(mut v) => {
    //                     let mut s = [0; 5];
    //                     let len = part.len();
    //                     for c in s[..len].iter_mut().rev() {
    //                         *c = b'0' + (v % 10) as u8;
    //                         v /= 10;
    //                     }
    //                     try!(write_bytes(self.buf, &s[..len]));
    //                 }
    //                 flt2dec::Part::Copy(buf) => {
    //                     try!(write_bytes(self.buf, buf));
    //                 }
    //             }
    //         }
    //         Ok(())
    //     }
    //
    //     /// Writes some data to the underlying buffer contained within this
    //     /// formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn write_str(&mut self, data: &str) -> Result {
    //         self.buf.write_str(data)
    //     }
    //
    //     /// Writes some formatted information into this instance
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn write_fmt(&mut self, fmt: Arguments) -> Result {
    //         write(self.buf, fmt)
    //     }
    //
    //     /// Flags for formatting (packed version of rt::Flag)
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn flags(&self) -> u32 { self.flags }
    //
    //     /// Character used as 'fill' whenever there is alignment
    //     #[unstable(feature = "core", reason = "method was just created")]
    //     pub fn fill(&self) -> char { self.fill }
    //
    //     /// Flag indicating what form of alignment was requested
    //     #[unstable(feature = "core", reason = "method was just created")]
    //     pub fn align(&self) -> Alignment { self.align }
    //
    //     /// Optionally specified integer width that the output should be
    //     #[unstable(feature = "core", reason = "method was just created")]
    //     pub fn width(&self) -> Option<usize> { self.width }
    //
    //     /// Optionally specified precision for numeric types
    //     #[unstable(feature = "core", reason = "method was just created")]
    //     pub fn precision(&self) -> Option<usize> { self.precision }
    //
    //     /// Creates a `DebugStruct` builder designed to assist with creation of
    //     /// `fmt::Debug` implementations for structs.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```rust
    //     /// use std::fmt;
    //     ///
    //     /// struct Foo {
    //     ///     bar: i32,
    //     ///     baz: String,
    //     /// }
    //     ///
    //     /// impl fmt::Debug for Foo {
    //     ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //     ///         fmt.debug_struct("Foo")
    //     ///             .field("bar", &self.bar)
    //     ///             .field("baz", &self.baz)
    //     ///             .finish()
    //     ///     }
    //     /// }
    //     ///
    //     /// // prints "Foo { bar: 10, baz: "Hello World" }"
    //     /// println!("{:?}", Foo { bar: 10, baz: "Hello World".to_string() });
    //     /// ```
    //     #[stable(feature = "debug_builders", since = "1.2.0")]
    //     #[inline]
    //     pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b, 'a> {
    //         builders::debug_struct_new(self, name)
    //     }
    //
    //     /// Creates a `DebugTuple` builder designed to assist with creation of
    //     /// `fmt::Debug` implementations for tuple structs.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```rust
    //     /// use std::fmt;
    //     ///
    //     /// struct Foo(i32, String);
    //     ///
    //     /// impl fmt::Debug for Foo {
    //     ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //     ///         fmt.debug_tuple("Foo")
    //     ///             .field(&self.0)
    //     ///             .field(&self.1)
    //     ///             .finish()
    //     ///     }
    //     /// }
    //     ///
    //     /// // prints "Foo(10, "Hello World")"
    //     /// println!("{:?}", Foo(10, "Hello World".to_string()));
    //     /// ```
    //     #[stable(feature = "debug_builders", since = "1.2.0")]
    //     #[inline]
    //     pub fn debug_tuple<'b>(&'b mut self, name: &str) -> DebugTuple<'b, 'a> {
    //         builders::debug_tuple_new(self, name)
    //     }
    //
    //     /// Creates a `DebugList` builder designed to assist with creation of
    //     /// `fmt::Debug` implementations for list-like structures.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```rust
    //     /// use std::fmt;
    //     ///
    //     /// struct Foo(Vec<i32>);
    //     ///
    //     /// impl fmt::Debug for Foo {
    //     ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //     ///         fmt.debug_list().entries(self.0.iter()).finish()
    //     ///     }
    //     /// }
    //     ///
    //     /// // prints "[10, 11]"
    //     /// println!("{:?}", Foo(vec![10, 11]));
    //     /// ```
    //     #[stable(feature = "debug_builders", since = "1.2.0")]
    //     #[inline]
    //     pub fn debug_list<'b>(&'b mut self) -> DebugList<'b, 'a> {
    //         builders::debug_list_new(self)
    //     }
    //
    //     /// Creates a `DebugSet` builder designed to assist with creation of
    //     /// `fmt::Debug` implementations for set-like structures.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```rust
    //     /// use std::fmt;
    //     ///
    //     /// struct Foo(Vec<i32>);
    //     ///
    //     /// impl fmt::Debug for Foo {
    //     ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //     ///         fmt.debug_set().entries(self.0.iter()).finish()
    //     ///     }
    //     /// }
    //     ///
    //     /// // prints "{10, 11}"
    //     /// println!("{:?}", Foo(vec![10, 11]));
    //     /// ```
    //     #[stable(feature = "debug_builders", since = "1.2.0")]
    //     #[inline]
    //     pub fn debug_set<'b>(&'b mut self) -> DebugSet<'b, 'a> {
    //         builders::debug_set_new(self)
    //     }
    //
    //     /// Creates a `DebugMap` builder designed to assist with creation of
    //     /// `fmt::Debug` implementations for map-like structures.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```rust
    //     /// use std::fmt;
    //     ///
    //     /// struct Foo(Vec<(String, i32)>);
    //     ///
    //     /// impl fmt::Debug for Foo {
    //     ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    //     ///         fmt.debug_map().entries(self.0.iter().map(|&(ref k, ref v)| (k, v))).finish()
    //     ///     }
    //     /// }
    //     ///
    //     /// // prints "{"A": 10, "B": 11}"
    //     /// println!("{:?}", Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)]));
    //     /// ```
    //     #[stable(feature = "debug_builders", since = "1.2.0")]
    //     #[inline]
    //     pub fn debug_map<'b>(&'b mut self) -> DebugMap<'b, 'a> {
    //         builders::debug_map_new(self)
    //     }
    // }

    // struct DebugInner<'a, 'b: 'a> {
    //     fmt: &'a mut fmt::Formatter<'b>,
    //     result: fmt::Result,
    //     has_fields: bool,
    // }

    // impl<'a, 'b: 'a> DebugInner<'a, 'b> {
    //     fn entry(&mut self, entry: &fmt::Debug) {
    //         self.result = self.result.and_then(|_| {
    //             if self.is_pretty() {
    //                 let mut writer = PadAdapter::new(self.fmt);
    //                 let prefix = if self.has_fields { "," } else { "" };
    //                 fmt::write(&mut writer, format_args!("{}\n{:#?}", prefix, entry))
    //             } else {
    //                 let prefix = if self.has_fields { ", " } else { "" };
    //                 write!(self.fmt, "{}{:?}", prefix, entry)
    //             }
    //         });
    //
    //         self.has_fields = true;
    //     }
    //
    //     pub fn finish(&mut self) {
    //         let prefix = if self.is_pretty() && self.has_fields { "\n" } else { "" };
    //         self.result = self.result.and_then(|_| self.fmt.write_str(prefix));
    //     }
    //
    //     fn is_pretty(&self) -> bool {
    //         self.fmt.flags() & (1 << (FlagV1::Alternate as usize)) != 0
    //     }
    // }

    // pub fn debug_list_new<'a, 'b>(fmt: &'a mut fmt::Formatter<'b>) -> DebugList<'a, 'b> {
    //     let result = write!(fmt, "[");
    //     DebugList {
    //         inner: DebugInner {
    //             fmt: fmt,
    //             result: result,
    //             has_fields: false,
    //         }
    //     }
    // }

    // pub struct DebugList<'a, 'b: 'a> {
    //     inner: DebugInner<'a, 'b>,
    // }

    // impl<'a, 'b: 'a> DebugList<'a, 'b> {
    //     /// Adds a new entry to the list output.
    //     #[stable(feature = "debug_builders", since = "1.2.0")]
    //     pub fn entry(&mut self, entry: &fmt::Debug) -> &mut DebugList<'a, 'b> {
    //         self.inner.entry(entry);
    //         self
    //     }
    //
    //     /// Adds the contents of an iterator of entries to the list output.
    //     #[stable(feature = "debug_builders", since = "1.2.0")]
    //     pub fn entries<D, I>(&mut self, entries: I) -> &mut DebugList<'a, 'b>
    //             where D: fmt::Debug, I: IntoIterator<Item=D> {
    //         for entry in entries {
    //             self.entry(&entry);
    //         }
    //         self
    //     }
    //
    //     /// Finishes output and returns any error encountered.
    //     #[stable(feature = "debug_builders", since = "1.2.0")]
    //     pub fn finish(&mut self) -> fmt::Result {
    //         self.inner.finish();
    //         self.inner.result.and_then(|_| self.inner.fmt.write_str("]"))
    //     }
    // }

    // pub trait Debug {
    //     /// Formats the value using the given formatter.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn fmt(&self, &mut Formatter) -> Result;
    // }

    impl Debug for A {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
	    let mut debuglist: DebugList = fmt.debug_list();

	    debuglist.entries(self.0.iter()).finish()
	}
    }

    type T = u32;

    #[test]
    fn debug_list_test1() {
	let a: A = A(vec!(68, 500, 999));
	let output: String = format!("{:?}", a);

	assert_eq!(output, "[68, 500, 999]".to_string());
    }

    #[test]
    fn debug_list_test2() {
	let a: A = A(vec!(68, 500, 999));
	let output: String = format!("{:#?}", a);

	assert_eq!(output, "[\n    68,\n    500,\n    999\n]".to_string());
    }
}
