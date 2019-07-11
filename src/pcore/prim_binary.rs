/// Primitives for handling binary data.
use super::parsebuffer::{ParsleyParser, ParseBuffer, ErrorKind};

pub struct BinaryScanner {
    tag: Vec<u8>
}

impl BinaryScanner {
    pub fn new(tag: &[u8]) -> BinaryScanner {
        let mut t = Vec::new();
        t.extend_from_slice(tag);
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
        t.extend_from_slice(tag);
        BinaryMatcher { tag: t }
    }
}

impl ParsleyParser for BinaryMatcher {
    type T = usize;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        buf.exact(&self.tag)
    }
}

/* Ideally, we would have a binary buffer that keeps a reference
   with the lifetime of the parser.  The naive approach runs into
   error E0207.

   A workaround could be to use PhantomData such as something like below.
   But this doesn't quite work, and variously crashes rustc!

   For now, just do the inefficient thing and copy the data, until
   we properly grok lifetimes and traits.

pub struct BinaryBuffer {
    len: usize,
    phantom: PhantomData<&'a [u8]>
}

impl<'a> BinaryBuffer<'a> {
    pub fn new(len: usize) -> BinaryBuffer<'a> {
        BinaryBuffer { len, phantom: PhantomData }
    }
}

impl<'a> ParsleyParser for BinaryBuffer<'a> {
    type T<'a> = &'a [u8];

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        buf.extract(self.len)
    }
}
*/

pub struct BinaryBuffer {
    len: usize
}

impl BinaryBuffer {
    pub fn new(len: usize) -> BinaryBuffer {
        BinaryBuffer { len }
    }
}

impl ParsleyParser for BinaryBuffer {
    type T = Vec<u8>;

    fn parse(&mut self, buf: &mut ParseBuffer) -> Result<Self::T, ErrorKind> {
        let bytes = buf.extract(self.len)?;
        let mut ret = Vec::new();
        ret.extend_from_slice(bytes);
        Ok(ret)
    }
}

// unit tests

#[cfg(test)]
mod test_binary {
    use super::{BinaryScanner, BinaryMatcher, BinaryBuffer};
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

    #[test]
    fn buffer() {
        let mut s = BinaryBuffer::new(3);
        let mut pb = ParseBuffer::new(Vec::from("".as_bytes()));
        assert_eq!(s.parse(&mut pb), Err(ErrorKind::EndOfBuffer));

        let mut s = BinaryBuffer::new(3);
        let mut pb = ParseBuffer::new(Vec::from("%PDF-".as_bytes()));
        let v = s.parse(&mut pb);
        let r = Vec::from("%PD".as_bytes());
        assert_eq!(v.unwrap(), r);
        assert_eq!(pb.get_cursor(), 3);
    }
}