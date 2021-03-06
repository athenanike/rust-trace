#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use core::fmt;

    #[derive(Debug)]
    struct Parser {
	result: String
    }

    struct ParseError { _priv: () }

    impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	    "cannot parse this context".fmt(f)
	}
    }

    impl FromStr for Parser {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
	    let marker_begin: &str = "<body>";
	    let marker_end: &str = "</body>";

	    if s.contains(marker_begin) && s.contains(marker_end) {
		let clipped: &str = s.trim_left_matches(marker_begin);
		let result: &str = clipped.trim_right_matches(marker_end);

		Ok::<Self, Self::Err>(
		    Parser {
			result: result.to_string()
		    }
		)
	    } else {
		Err::<Self, Self::Err>(ParseError { _priv: () })
	    }
	}
    }

    // impl StrExt for str {
    //     #[inline]
    //     fn contains<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool {
    //         pat.is_contained_in(self)
    //     }
    //
    //     #[inline]
    //     fn contains_char<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool {
    //         pat.is_contained_in(self)
    //     }
    //
    //     #[inline]
    //     fn chars(&self) -> Chars {
    //         Chars{iter: self.as_bytes().iter()}
    //     }
    //
    //     #[inline]
    //     fn bytes(&self) -> Bytes {
    //         Bytes(self.as_bytes().iter().map(BytesDeref))
    //     }
    //
    //     #[inline]
    //     fn char_indices(&self) -> CharIndices {
    //         CharIndices { front_offset: 0, iter: self.chars() }
    //     }
    //
    //     #[inline]
    //     fn split<'a, P: Pattern<'a>>(&'a self, pat: P) -> Split<'a, P> {
    //         Split(SplitInternal {
    //             start: 0,
    //             end: self.len(),
    //             matcher: pat.into_searcher(self),
    //             allow_trailing_empty: true,
    //             finished: false,
    //         })
    //     }
    //
    //     #[inline]
    //     fn rsplit<'a, P: Pattern<'a>>(&'a self, pat: P) -> RSplit<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RSplit(self.split(pat).0)
    //     }
    //
    //     #[inline]
    //     fn splitn<'a, P: Pattern<'a>>(&'a self, count: usize, pat: P) -> SplitN<'a, P> {
    //         SplitN(SplitNInternal {
    //             iter: self.split(pat).0,
    //             count: count,
    //         })
    //     }
    //
    //     #[inline]
    //     fn rsplitn<'a, P: Pattern<'a>>(&'a self, count: usize, pat: P) -> RSplitN<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RSplitN(self.splitn(count, pat).0)
    //     }
    //
    //     #[inline]
    //     fn split_terminator<'a, P: Pattern<'a>>(&'a self, pat: P) -> SplitTerminator<'a, P> {
    //         SplitTerminator(SplitInternal {
    //             allow_trailing_empty: false,
    //             ..self.split(pat).0
    //         })
    //     }
    //
    //     #[inline]
    //     fn rsplit_terminator<'a, P: Pattern<'a>>(&'a self, pat: P) -> RSplitTerminator<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RSplitTerminator(self.split_terminator(pat).0)
    //     }
    //
    //     #[inline]
    //     fn matches<'a, P: Pattern<'a>>(&'a self, pat: P) -> Matches<'a, P> {
    //         Matches(MatchesInternal(pat.into_searcher(self)))
    //     }
    //
    //     #[inline]
    //     fn rmatches<'a, P: Pattern<'a>>(&'a self, pat: P) -> RMatches<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RMatches(self.matches(pat).0)
    //     }
    //
    //     #[inline]
    //     fn match_indices<'a, P: Pattern<'a>>(&'a self, pat: P) -> MatchIndices<'a, P> {
    //         MatchIndices(MatchIndicesInternal(pat.into_searcher(self)))
    //     }
    //
    //     #[inline]
    //     fn rmatch_indices<'a, P: Pattern<'a>>(&'a self, pat: P) -> RMatchIndices<'a, P>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         RMatchIndices(self.match_indices(pat).0)
    //     }
    //     #[inline]
    //     fn lines(&self) -> Lines {
    //         Lines(self.split_terminator('\n'))
    //     }
    //
    //     #[inline]
    //     fn lines_any(&self) -> LinesAny {
    //         LinesAny(self.lines().map(LinesAnyMap))
    //     }
    //
    //     #[inline]
    //     fn char_len(&self) -> usize { self.chars().count() }
    //
    //     fn slice_chars(&self, begin: usize, end: usize) -> &str {
    //         assert!(begin <= end);
    //         let mut count = 0;
    //         let mut begin_byte = None;
    //         let mut end_byte = None;
    //
    //         // This could be even more efficient by not decoding,
    //         // only finding the char boundaries
    //         for (idx, _) in self.char_indices() {
    //             if count == begin { begin_byte = Some(idx); }
    //             if count == end { end_byte = Some(idx); break; }
    //             count += 1;
    //         }
    //         if begin_byte.is_none() && count == begin { begin_byte = Some(self.len()) }
    //         if end_byte.is_none() && count == end { end_byte = Some(self.len()) }
    //
    //         match (begin_byte, end_byte) {
    //             (None, _) => panic!("slice_chars: `begin` is beyond end of string"),
    //             (_, None) => panic!("slice_chars: `end` is beyond end of string"),
    //             (Some(a), Some(b)) => unsafe { self.slice_unchecked(a, b) }
    //         }
    //     }
    //
    //     #[inline]
    //     unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &str {
    //         mem::transmute(Slice {
    //             data: self.as_ptr().offset(begin as isize),
    //             len: end - begin,
    //         })
    //     }
    //
    //     #[inline]
    //     fn starts_with<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool {
    //         pat.is_prefix_of(self)
    //     }
    //
    //     #[inline]
    //     fn ends_with<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         pat.is_suffix_of(self)
    //     }
    //
    //     #[inline]
    //     fn trim_matches<'a, P: Pattern<'a>>(&'a self, pat: P) -> &'a str
    //         where P::Searcher: DoubleEndedSearcher<'a>
    //     {
    //         let mut i = 0;
    //         let mut j = 0;
    //         let mut matcher = pat.into_searcher(self);
    //         if let Some((a, b)) = matcher.next_reject() {
    //             i = a;
    //             j = b; // Rember earliest known match, correct it below if
    //                    // last match is different
    //         }
    //         if let Some((_, b)) = matcher.next_reject_back() {
    //             j = b;
    //         }
    //         unsafe {
    //             // Searcher is known to return valid indices
    //             self.slice_unchecked(i, j)
    //         }
    //     }
    //
    //     #[inline]
    //     fn trim_left_matches<'a, P: Pattern<'a>>(&'a self, pat: P) -> &'a str {
    //         let mut i = self.len();
    //         let mut matcher = pat.into_searcher(self);
    //         if let Some((a, _)) = matcher.next_reject() {
    //             i = a;
    //         }
    //         unsafe {
    //             // Searcher is known to return valid indices
    //             self.slice_unchecked(i, self.len())
    //         }
    //     }
    //
    //     #[inline]
    //     fn trim_right_matches<'a, P: Pattern<'a>>(&'a self, pat: P) -> &'a str
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         let mut j = 0;
    //         let mut matcher = pat.into_searcher(self);
    //         if let Some((_, b)) = matcher.next_reject_back() {
    //             j = b;
    //         }
    //         unsafe {
    //             // Searcher is known to return valid indices
    //             self.slice_unchecked(0, j)
    //         }
    //     }
    //
    //     #[inline]
    //     fn is_char_boundary(&self, index: usize) -> bool {
    //         if index == self.len() { return true; }
    //         match self.as_bytes().get(index) {
    //             None => false,
    //             Some(&b) => b < 128 || b >= 192,
    //         }
    //     }
    //
    //     #[inline]
    //     fn char_range_at(&self, i: usize) -> CharRange {
    //         let (c, n) = char_range_at_raw(self.as_bytes(), i);
    //         CharRange { ch: unsafe { mem::transmute(c) }, next: n }
    //     }
    //
    //     #[inline]
    //     fn char_range_at_reverse(&self, start: usize) -> CharRange {
    //         let mut prev = start;
    //
    //         prev = prev.saturating_sub(1);
    //         if self.as_bytes()[prev] < 128 {
    //             return CharRange{ch: self.as_bytes()[prev] as char, next: prev}
    //         }
    //
    //         // Multibyte case is a fn to allow char_range_at_reverse to inline cleanly
    //         fn multibyte_char_range_at_reverse(s: &str, mut i: usize) -> CharRange {
    //             // while there is a previous byte == 10......
    //             while i > 0 && s.as_bytes()[i] & !CONT_MASK == TAG_CONT_U8 {
    //                 i -= 1;
    //             }
    //
    //             let first= s.as_bytes()[i];
    //             let w = UTF8_CHAR_WIDTH[first as usize];
    //             assert!(w != 0);
    //
    //             let mut val = utf8_first_byte(first, w as u32);
    //             val = utf8_acc_cont_byte(val, s.as_bytes()[i + 1]);
    //             if w > 2 { val = utf8_acc_cont_byte(val, s.as_bytes()[i + 2]); }
    //             if w > 3 { val = utf8_acc_cont_byte(val, s.as_bytes()[i + 3]); }
    //
    //             return CharRange {ch: unsafe { mem::transmute(val) }, next: i};
    //         }
    //
    //         return multibyte_char_range_at_reverse(self, prev);
    //     }
    //
    //     #[inline]
    //     fn char_at(&self, i: usize) -> char {
    //         self.char_range_at(i).ch
    //     }
    //
    //     #[inline]
    //     fn char_at_reverse(&self, i: usize) -> char {
    //         self.char_range_at_reverse(i).ch
    //     }
    //
    //     #[inline]
    //     fn as_bytes(&self) -> &[u8] {
    //         unsafe { mem::transmute(self) }
    //     }
    //
    //     fn find<'a, P: Pattern<'a>>(&'a self, pat: P) -> Option<usize> {
    //         pat.into_searcher(self).next_match().map(|(i, _)| i)
    //     }
    //
    //     fn rfind<'a, P: Pattern<'a>>(&'a self, pat: P) -> Option<usize>
    //         where P::Searcher: ReverseSearcher<'a>
    //     {
    //         pat.into_searcher(self).next_match_back().map(|(i, _)| i)
    //     }
    //
    //     fn find_str<'a, P: Pattern<'a>>(&'a self, pat: P) -> Option<usize> {
    //         self.find(pat)
    //     }
    //
    //     #[inline]
    //     fn slice_shift_char(&self) -> Option<(char, &str)> {
    //         if self.is_empty() {
    //             None
    //         } else {
    //             let ch = self.char_at(0);
    //             let next_s = unsafe { self.slice_unchecked(ch.len_utf8(), self.len()) };
    //             Some((ch, next_s))
    //         }
    //     }
    //
    //     fn subslice_offset(&self, inner: &str) -> usize {
    //         let a_start = self.as_ptr() as usize;
    //         let a_end = a_start + self.len();
    //         let b_start = inner.as_ptr() as usize;
    //         let b_end = b_start + inner.len();
    //
    //         assert!(a_start <= b_start);
    //         assert!(b_end <= a_end);
    //         b_start - a_start
    //     }
    //
    //     #[inline]
    //     fn as_ptr(&self) -> *const u8 {
    //         self.repr().data
    //     }
    //
    //     #[inline]
    //     fn len(&self) -> usize { self.repr().len }
    //
    //     #[inline]
    //     fn is_empty(&self) -> bool { self.len() == 0 }
    //
    //     #[inline]
    //     fn parse<T: FromStr>(&self) -> Result<T, T::Err> { FromStr::from_str(self) }
    // }

    #[test]
    fn fromstr_test1() {
	let x: &str = "<body>Hello, World</body>";
	let result: Result<Parser, <Parser as FromStr>::Err> = x.parse::<Parser>();

	match result {
	    Ok(p) => assert_eq!(p.result, "Hello, World"),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn fromstr_test2() {
	let x: &str = "Hello, World";
	let parse: Result<Parser, <Parser as FromStr>::Err> = x.parse::<Parser>();
	let unwrap_err: ParseError = parse.unwrap_err();
	let result: String = format!("{}", unwrap_err);

	assert_eq!(result, "cannot parse this context");
    }
}
