// Copyright (c) 2020 SRI International.
// All rights reserved.
//
//    This file is part of the Parsley parser.
//
//    Parsley is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    Parsley is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// Transformations of parse buffers.

use super::parsebuffer::{
    ParseResult, ParseBufferT, ParseBuffer, ErrorKind,
    Location, locate_value
};

pub type TransformResult = ParseResult<ParseBuffer>;

pub trait BufferTransformT {
    // Transform the given parsebuffer into another.
    fn transform(&mut self, buf: &dyn ParseBufferT) -> TransformResult;
}

// View restrictions are a form of buffer transformations.

pub struct RestrictView {
    start: usize,
    size: usize,
}

impl RestrictView {
    pub fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }
}

impl BufferTransformT for RestrictView {
    fn transform(&mut self, buf: &dyn ParseBufferT) -> TransformResult {
        if self.start + self.size <= buf.size() {
            Ok(ParseBuffer::new_view(buf, self.start, self.size))
        } else {
            let err = ErrorKind::BoundsError;
            let loc = buf.get_location();
            Err(locate_value(err, loc.loc_start(), loc.loc_end()))
        }
    }
}

pub struct RestrictViewFrom {
    start: usize
}

impl RestrictViewFrom {
    pub fn new(start: usize) -> Self {
        Self { start }
    }
}

impl BufferTransformT for RestrictViewFrom {
    fn transform(&mut self, buf: &dyn ParseBufferT) -> TransformResult {
        if self.start < buf.size() {
            Ok(ParseBuffer::new_view(buf, self.start, buf.size() - self.start))
        } else {
            let err = ErrorKind::BoundsError;
            let loc = buf.get_location();
            Err(locate_value(err, loc.loc_start(), loc.loc_end()))
        }
    }
}

#[cfg(test)]
mod test_transforms {
    use super::super::parsebuffer::{ParseBuffer, ParseBufferT};
    use super::{BufferTransformT, RestrictView, RestrictViewFrom};

    #[test]
    fn test_restrict_view() {
        let v = Vec::from("0123456789".as_bytes());
        let mut pb = ParseBuffer::new(v);
        assert_eq!(pb.scan(b"9"), Ok(9));
        pb.set_cursor(0);
        assert_eq!(pb.scan(b"56"), Ok(5));
        let size = pb.remaining();
        let mut view = RestrictView::new(5, size);
        pb = view.transform(&pb).unwrap();
        assert_eq!(pb.get_cursor(), 0);
        assert_eq!(pb.remaining(), size);
        assert_eq!(pb.scan(b"56"), Ok(0));
        pb.set_cursor(0);
        assert_eq!(pb.scan(b"9"), Ok(4));

        // identical view
        pb.set_cursor(0);
        let size = pb.remaining();
        let mut view = RestrictView::new(0, size);
        pb = view.transform(&pb).unwrap();
        assert_eq!(pb.remaining(), size);
        assert_eq!(pb.scan(b"9"), Ok(4));

        // identical view
        let mut view = RestrictViewFrom::new(0);
        pb = view.transform(&pb).unwrap();
        assert_eq!(pb.remaining(), size);
        assert_eq!(pb.scan(b"9"), Ok(4));

        // view from
        let mut view = RestrictViewFrom::new(pb.size() - 1);
        pb = view.transform(&pb).unwrap();
        assert_eq!(pb.remaining(), 1);
        assert_eq!(pb.scan(b"9"), Ok(0));
        let mut view = RestrictViewFrom::new(1);
        let pb = view.transform(&pb);
        assert!(pb.is_err());
    }
}
