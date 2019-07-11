/// Primitives for handling binary data.
use super::parsebuffer::{ParsleyParser, ParseBuffer, ErrorKind};

pub struct BinaryScanner {
    tag: Vec<u8>
}

impl BinaryScanner {
    pub fn new(tag: &[u8]) -> BinaryScanner {
        let mut t = Vec::new();
        for c in tag.iter() { t.push(*c); }
        BinaryScanner { tag: t }
    }
}

impl ParsleyParser for BinaryScanner {
    type T = usize;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        buf.scan(&self.tag)
    }
}

pub struct BinaryMatcher {
    tag: Vec<u8>
}

impl BinaryMatcher {
    pub fn new(tag: &[u8]) -> BinaryMatcher {
        let mut t = Vec::new();
        for c in tag.iter() { t.push(*c); }
        BinaryMatcher { tag: t }
    }
}

impl ParsleyParser for BinaryMatcher {
    type T = usize;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        buf.exact(&self.tag)
    }
}

// unit tests

#[cfg(test)]
mod test_binary {
    use super::{BinaryScanner, BinaryMatcher};
    use super::super::parsebuffer::{ParseBuffer, ParsleyParser, ErrorKind};

    #[test]
    fn scan() {
        // The fact that this has to be mutable is a defect in the current API.
        let mut s = BinaryScanner::new("%PDF-".as_bytes());

        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(0));
        assert_eq!(pb.get_cursor(), 0);

        let mut pb = ParseBuffer::new(Vec::from("garbage %PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(8));
        assert_eq!(pb.get_cursor(), 8);
        assert_eq!(s.parse(&mut pb), Ok(0));
        assert_eq!(pb.get_cursor(), 8);

        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        assert_eq!(s.parse(&mut pb), Err(ErrorKind::EndOfBuffer));
        assert_eq!(pb.get_cursor(), 0);
    }

    #[test]
    fn exact() {
        // The fact that this has to be mutable is a defect in the current API.
        let mut s = BinaryMatcher::new("%PDF-".as_bytes());

        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        assert_eq!(s.parse(&mut pb), Err(ErrorKind::GuardError("match")));
        assert_eq!(pb.get_cursor(), 0);

        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Ok(5));
        assert_eq!(pb.get_cursor(), 5);

        let mut pb = ParseBuffer::new(Vec::from(" %PDF-".as_bytes()));
        assert_eq!(s.parse(&mut pb), Err(ErrorKind::GuardError("match")));
        assert_eq!(pb.get_cursor(), 0);
    }
}
