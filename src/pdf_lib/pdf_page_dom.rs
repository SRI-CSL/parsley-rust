// Copyright (c) 2019-2021 SRI International.
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

// This is a truncated DOM for PDF that highlights only the elements
// relevant to text-extraction from pages.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::{DictKey, DictT, ObjectId, PDFObjContext, PDFObjT};

// DOM context, a more specialized version of PDFObjContext
pub struct DOMContext {
    // pages does not contain the root page, since that is contained
    // within the Catalog.
    pages:       BTreeMap<ObjectId, PageKid>,
    font_dicts:  BTreeMap<ObjectId, Rc<FontDictionary>>,
    font_descrs: BTreeMap<ObjectId, Rc<FontDescriptor>>,
}
impl DOMContext {
    pub fn new() -> Self {
        Self {
            pages:       BTreeMap::new(),
            font_dicts:  BTreeMap::new(),
            font_descrs: BTreeMap::new(),
        }
    }
    pub fn pages(&self) -> &BTreeMap<ObjectId, PageKid> { &self.pages }
    pub fn font_dicts(&self) -> &BTreeMap<ObjectId, Rc<FontDictionary>> { &self.font_dicts }
    pub fn font_descrs(&self) -> &BTreeMap<ObjectId, Rc<FontDescriptor>> { &self.font_descrs }
}
impl Default for DOMContext {
    fn default() -> Self { DOMContext::new() }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PageKid {
    Node(Rc<PageTreeNode>),
    Leaf(Rc<Page>),
}

// DOM types

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Catalog {
    root_page: Rc<RootPageTreeNode>,
}
impl Catalog {
    pub fn new(root_page: Rc<RootPageTreeNode>) -> Self { Self { root_page } }
    pub fn root_page(&self) -> &RootPageTreeNode { &self.root_page }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct RootPageTreeNode {
    count:     usize,
    resources: Option<Rc<Resources>>,
    kids:      Vec<ObjectId>,
}
impl RootPageTreeNode {
    pub fn new(resources: Option<Rc<Resources>>, count: usize, kids: Vec<ObjectId>) -> Self {
        Self {
            resources,
            count,
            kids,
        }
    }
    pub fn resources(&self) -> &Option<Rc<Resources>> { &self.resources }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PageTreeNode {
    parent:    ObjectId,
    resources: Option<Rc<Resources>>,
    count:     usize,
    kids:      Vec<ObjectId>,
}
impl PageTreeNode {
    pub fn new(
        parent: ObjectId, resources: Option<Rc<Resources>>, count: usize, kids: Vec<ObjectId>,
    ) -> Self {
        Self {
            parent,
            count,
            kids,
            resources,
        }
    }
    pub fn kids(&self) -> &[ObjectId] { self.kids.as_slice() }
    pub fn resources(&self) -> &Option<Rc<Resources>> { &self.resources }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Page {
    parent:    ObjectId,
    resources: Rc<Resources>, // after inheritance processing
    contents:  Vec<Rc<LocatedVal<PDFObjT>>>,
}

impl Page {
    pub fn new(
        parent: ObjectId, resources: Rc<Resources>, contents: Vec<Rc<LocatedVal<PDFObjT>>>,
    ) -> Self {
        Self {
            parent,
            contents,
            resources,
        }
    }
    pub fn contents(&self) -> &[Rc<LocatedVal<PDFObjT>>] { self.contents.as_slice() }
    pub fn resources(&self) -> &Resources { &self.resources }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Resources {
    fonts: BTreeMap<DictKey, Rc<FontDictionary>>,
}
impl Resources {
    pub fn new(fonts: BTreeMap<DictKey, Rc<FontDictionary>>) -> Self { Self { fonts } }
    pub fn fonts(&self) -> &BTreeMap<DictKey, Rc<FontDictionary>> { &self.fonts }
    /*
    pub fn has_nonembedded_fonts(&self, ctxt: &PDFObjContext) -> bool {
        let mut has = false;
        for (_, f) in self.fonts.iter() {
            match f {
                FontResource::Id(id) =>
        }
    }
     */
}

// ternary value for feature presence
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FeaturePresence {
    True,
    False,
    Unknown,
}
impl From<bool> for FeaturePresence {
    fn from(f: bool) -> FeaturePresence {
        if f {
            FeaturePresence::True
        } else {
            FeaturePresence::False
        }
    }
}
impl From<Option<bool>> for FeaturePresence {
    fn from(f: Option<bool>) -> FeaturePresence {
        match f {
            Some(true) => FeaturePresence::True,
            Some(false) => FeaturePresence::False,
            None => FeaturePresence::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FontType {
    Type0,
    Type1,
    MMType1,
    Type3,
    TrueType,
    CIDFontType0,
    CIDFontType1,
    Unknown(Vec<u8>),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FontEncoding {
    MacRoman,
    MacExpert,
    WinAnsi,
    Unknown(String),
    Dict(Rc<LocatedVal<PDFObjT>>),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FontFlag {
    FixedPitch,
    Serif,
    Symbolic,
    Script,
    Nonsymbolic,
    Italic,
    AllCap,
    SmallCap,
    ForceBold,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FontDescriptor {
    fontname:  Vec<u8>,
    flags:     BTreeSet<FontFlag>,
    fontfile:  Option<ObjectId>,
    fontfile2: Option<ObjectId>,
    fontfile3: Option<ObjectId>,
}
impl FontDescriptor {
    pub fn fontname(&self) -> &[u8] { &self.fontname }
    // FIXME: this is necessary but not sufficient: the ObjectIds may
    // not exist, or may point to invalid streams.
    pub fn is_embedded(&self) -> bool {
        self.fontfile.is_some() || self.fontfile2.is_some() || self.fontfile3.is_some()
    }
    pub fn is_symbolic(&self) -> bool { self.flags.contains(&FontFlag::Symbolic) }
}

pub const STANDARD_FONTS: &[&str] = &[
    "Times-Roman",
    "Times-Bold",
    "Times-Italic",
    "Times-BoldItalic",
    "Helvetica",
    "Helvetica-Bold",
    "Helvetica-Oblique",
    "Helvetica-BoldOblique",
    "Courier",
    "Courier-Bold",
    "Courier-Oblique",
    "Courier-BoldOblique",
    "Symbol",
    "ZapfDingbats",
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FontDictionary {
    subtype:        FontType,
    basefont:       Vec<u8>,
    fontdescriptor: Option<Rc<FontDescriptor>>,
    encoding:       Option<FontEncoding>,
    // ToUnicode
}
impl FontDictionary {
    pub fn basefont(&self) -> &[u8] { &self.basefont }
    pub fn encoding(&self) -> &Option<FontEncoding> { &self.encoding }
    pub fn is_embedded(&self) -> FeaturePresence {
        if self.is_base_font() {
            FeaturePresence::from(Some(true))
        } else {
            FeaturePresence::from(
                self.fontdescriptor
                    .as_ref()
                    .map(|fd| fd.is_embedded()),
            )
        }
    }
    pub fn is_symbolic(&self) -> FeaturePresence {
        FeaturePresence::from(
            self.fontdescriptor
                .as_ref()
                .map(|fd| fd.is_symbolic()),
        )
    }
    pub fn is_base_font(&self) -> bool {
        match (&self.subtype, std::str::from_utf8(&self.basefont)) {
            (FontType::Type1, Ok(s)) => {
                for f in STANDARD_FONTS {
                    if f == &s {
                        return true
                    }
                }
                false
            },
            _ => false,
        }
    }
}

// Converters from basic PDF types.
//
// These converters should operate on basic PDF types that have passed
// the 'type-checker', so there should ideally be no conversion errors
// and we should be able to get by with only an option return type.
// The converters will always operate within a PDFObjContext, even if
// they don't use it.
//
// To avoid deep recursion that could blow the stack, the converters
// maintain a queue of nested objects that need conversion, using
// ObjectIds for indirection.
//
// To implement inheritance, the queue keeps reference-counted copies
// of the inheritable attributes that were in scope (currently, only
// "Resources") at the time an object was added to the conversion
// queue.  These in-scope attributes are provided to the converters.
// The leaf objects retain a copy of these attributes if they need to.

type QEntry = (ObjectId, Option<Rc<Resources>>, Rc<LocatedVal<PDFObjT>>);
pub struct ConversionQ {
    page_nodes: VecDeque<QEntry>,
    examined:   BTreeSet<ObjectId>,
}
impl ConversionQ {
    fn new() -> Self {
        Self {
            page_nodes: VecDeque::new(),
            examined:   BTreeSet::new(),
        }
    }

    fn add(&mut self, id: ObjectId, r: Option<Rc<Resources>>, o: Rc<LocatedVal<PDFObjT>>) {
        // There might be cycles in the page tree.  Our type-checker
        // cannot currently catch them, so we just handle them by
        // ensuring we don't keep adding the pages in the cycle to the
        // queue.
        if !self.examined.contains(&id) {
            self.page_nodes.push_back((id, r, o));
            self.examined.insert(id);
        }
    }

    fn next(&mut self) -> Option<QEntry> {
        self.page_nodes.pop_front()
    }

    fn is_empty(&self) -> bool { self.page_nodes.is_empty() }
}

// Table 29, page 98 (2020 edn)
pub fn to_catalog(
    ctxt: &PDFObjContext, q: &mut ConversionQ, dom: &mut DOMContext, o: &LocatedVal<PDFObjT>,
) -> Result<Catalog, LocatedVal<PageDOMError>> {
    match o.val() {
        PDFObjT::Dict(d) => match d.get_ref(b"Pages") {
            Some(r) => match ctxt.lookup_obj(r.id()) {
                Some(o) => {
                    let root = to_root_page_tree_node(ctxt, q, dom, o)?;
                    Ok(Catalog::new(Rc::new(root)))
                },
                None => Err(o.place(PageDOMError::CatalogConversionPagesIdNotFound)),
            },
            None => Err(o.place(PageDOMError::CatalogConversionNoPages)),
        },
        _ => Err(o.place(PageDOMError::CatalogConversionBadCatalog)),
    }
}

// Table 30, page 103 (2020 edn)
fn to_page_kids(
    ctxt: &PDFObjContext, q: &mut ConversionQ, r: &Option<Rc<Resources>>, o: &LocatedVal<PDFObjT>,
) -> Option<Vec<ObjectId>> {
    match o.val() {
        PDFObjT::Reference(rf) => {
            // handle reference to an array
            ctxt.lookup_obj(rf.id())
                .and_then(|o| to_page_kids(ctxt, q, r, o))
        },
        PDFObjT::Array(a) => {
            let mut kids = Vec::new();
            for o in a.objs() {
                match o.val() {
                    PDFObjT::Reference(rf) => {
                        if let Some(o) = ctxt.lookup_obj(rf.id()) {
                            match r {
                                None => q.add(rf.id(), None, Rc::clone(o)),
                                Some(r) => q.add(rf.id(), Some(Rc::clone(r)), Rc::clone(o)),
                            }
                        };
                        // kids is updated for completeness, even
                        // though the object may not exist in the ctxt.
                        kids.push(rf.id())
                    },
                    _ => {
                        // type-checker should forbid this
                        println!(
                            "Kid conversion error: expected ref, but found {:?}\n",
                            o.val()
                        )
                    },
                }
            }
            Some(kids)
        },
        _ => {
            // type-checker should forbid this
            println!(
                "Kids conversion error: expected array of refs, but found {:?}\n",
                o.val()
            );
            None
        },
    }
}

// Table 30, page 103 (2020 edn)
fn to_root_page_tree_node(
    ctxt: &PDFObjContext, q: &mut ConversionQ, dom: &mut DOMContext, o: &LocatedVal<PDFObjT>,
) -> Result<RootPageTreeNode, LocatedVal<PageDOMError>> {
    match o.val() {
        PDFObjT::Dict(d) => {
            let res = match d.get_resolved_dict(ctxt, b"Resources") {
                None => None,
                Some(d) => match to_resources(ctxt, dom, d, o) {
                    Ok(res) => Some(Rc::new(res)),
                    Err(e) => return Err(e),
                },
            };
            let count = match d.get_usize(b"Count") {
                None => return Err(o.place(PageDOMError::PageTreeNodeConversionNoCount)),
                Some(i) => i,
            };
            let kids = match d.get(b"Kids") {
                None => return Err(o.place(PageDOMError::PageTreeNodeConversionNoKids)),
                Some(o) => to_page_kids(ctxt, q, &res, o),
            };
            match kids {
                None => Err(o.place(PageDOMError::PageTreeNodeConversionBadKids)),
                Some(v) => Ok(RootPageTreeNode::new(res, count, v)),
            }
        },
        _ => Err(o.place(PageDOMError::PageTreeNodeConversionBadRoot)),
    }
}

// Table 30, page 103 (2020 edn)
fn to_page_tree_node(
    ctxt: &PDFObjContext, q: &mut ConversionQ, dom: &mut DOMContext, r: &Option<Rc<Resources>>,
    o: &LocatedVal<PDFObjT>,
) -> Result<PageTreeNode, LocatedVal<PageDOMError>> {
    match o.val() {
        PDFObjT::Dict(d) => {
            let parent = match d.get_ref(b"Parent") {
                None => return Err(o.place(PageDOMError::PageTreeNodeConversionNoParent)),
                Some(p) => p.id(),
            };
            let res = match (d.get_resolved_dict(ctxt, b"Resources"), r) {
                (Some(d), _) => match to_resources(ctxt, dom, d, o) {
                    Ok(res) => Some(Rc::new(res)),
                    Err(e) => return Err(e),
                },
                (_, Some(r)) => Some(Rc::clone(r)),
                (_, _) => None,
            };
            let count = match d.get_usize(b"Count") {
                None => return Err(o.place(PageDOMError::PageTreeNodeConversionNoCount)),
                Some(i) => i,
            };
            let kids = match d.get(b"Kids") {
                None => return Err(o.place(PageDOMError::PageTreeNodeConversionNoKids)),
                Some(o) => to_page_kids(ctxt, q, &res, o),
            };
            match kids {
                None => Err(o.place(PageDOMError::PageTreeNodeConversionBadKids)),
                Some(v) => Ok(PageTreeNode::new(parent, res, count, v)),
            }
        },
        _ => Err(o.place(PageDOMError::PageTreeNodeConversionBadNode)),
    }
}

fn to_page_content(
    ctxt: &PDFObjContext, o: &Rc<LocatedVal<PDFObjT>>,
) -> Option<Rc<LocatedVal<PDFObjT>>> {
    match o.val() {
        PDFObjT::Reference(r) => ctxt
            .lookup_obj(r.id())
            .and_then(|o| to_page_content(ctxt, o)),
        PDFObjT::Stream(_) => Some(Rc::clone(o)),
        _ => None,
    }
}

fn to_page_contents(
    ctxt: &PDFObjContext, o: &Rc<LocatedVal<PDFObjT>>,
) -> Option<Vec<Rc<LocatedVal<PDFObjT>>>> {
    match o.val() {
        PDFObjT::Reference(r) => ctxt
            .lookup_obj(r.id())
            .and_then(|o| to_page_contents(ctxt, o)),
        PDFObjT::Stream(_) => Some(vec![Rc::clone(o)]),
        PDFObjT::Array(a) => {
            let mut v = Vec::new();
            for o in a.objs() {
                match to_page_content(ctxt, o) {
                    None => return None,
                    Some(cs) => v.push(Rc::clone(&cs)),
                }
            }
            Some(v)
        },
        _ => None,
    }
}

fn to_resource_font_value(
    ctxt: &PDFObjContext, dom: &mut DOMContext, o: &LocatedVal<PDFObjT>,
) -> Result<BTreeMap<DictKey, Rc<FontDictionary>>, LocatedVal<PageDOMError>> {
    let mut fonts = BTreeMap::new();
    match o.val() {
        // The value should be a dictionary mapping font resource
        // names to font dictionaries.
        PDFObjT::Dict(d) => {
            for (frn, fr) in d.map().iter() {
                match fr.val() {
                    PDFObjT::Reference(r) => match ctxt.lookup_obj(r.id()) {
                        None => return Err(o.place(PageDOMError::FontDictUnresolvedId(r.id()))),
                        Some(o) => {
                            let fd = match obj_to_font_dict(ctxt, dom, o) {
                                Err(e) => return Err(e),
                                Ok(fd) => Rc::new(fd),
                            };
                            dom.font_dicts.insert(r.id(), Rc::clone(&fd));
                            fonts.insert(frn.clone(), fd);
                        },
                    },
                    PDFObjT::Dict(dd) => {
                        let fd = match to_font_dict(ctxt, dom, dd) {
                            Err(e) => return Err(o.place(e)),
                            Ok(fd) => fd,
                        };
                        fonts.insert(frn.clone(), Rc::new(fd));
                    },
                    _ => return Err(o.place(PageDOMError::FontResourceNotDict)),
                }
            }
        },
        PDFObjT::Reference(r) => match ctxt.lookup_obj(r.id()) {
            Some(o) => return to_resource_font_value(ctxt, dom, o),
            None => return Err(o.place(PageDOMError::ResourceFontValueUnknownObjectId(r.id()))),
        },
        _ => return Err(o.place(PageDOMError::ResourceFontValueNotDict)),
    }
    Ok(fonts)
}

// Table 34, page 113 (2020 edn)
fn to_resources(
    ctxt: &PDFObjContext, dom: &mut DOMContext, rd: &DictT, _container: &LocatedVal<PDFObjT>,
) -> Result<Resources, LocatedVal<PageDOMError>> {
    let mut fonts = None;
    for (k, v) in rd.map().iter() {
        if k.as_slice() == b"Font" {
            match to_resource_font_value(ctxt, dom, v) {
                Err(e) => return Err(e),
                Ok(f) => fonts = Some(f),
            }
        }
    }
    match fonts {
        None => Ok(Resources::new(BTreeMap::new())),
        Some(f) => Ok(Resources::new(f)),
    }
}

// Table 31, page 104 (2020 edn)
fn to_page(
    ctxt: &PDFObjContext, dom: &mut DOMContext, r: &Option<Rc<Resources>>, o: &LocatedVal<PDFObjT>,
) -> Result<Page, LocatedVal<PageDOMError>> {
    match o.val() {
        PDFObjT::Dict(d) => {
            let parent = match d.get_ref(b"Parent") {
                None => return Err(o.place(PageDOMError::PageNodeConversionNoParent)),
                Some(p) => p.id(),
            };
            let res = match (d.get_resolved_dict(ctxt, b"Resources"), r) {
                (Some(d), _) => match to_resources(ctxt, dom, d, o) {
                    Ok(r) => Rc::new(r),
                    Err(e) => return Err(e),
                },
                (_, Some(r)) => Rc::clone(r),
                (_, _) => Rc::new(Resources::default()),
            };
            let contents = match d.get(b"Contents") {
                None => return Err(o.place(PageDOMError::PageNodeConversionNoContents)),
                Some(o) => match to_page_contents(ctxt, o) {
                    None => return Err(o.place(PageDOMError::PageNodeConversionBadContents)),
                    Some(v) => v,
                },
            };
            Ok(Page::new(parent, res, contents))
        },
        _ => Err(o.place(PageDOMError::PageNodeConversionBadPage)),
    }
}

fn to_font_descriptor(d: &DictT) -> Result<FontDescriptor, PageDOMError> {
    let fontname = match d.get_name(b"FontName") {
        Some(n) => n.to_vec(),
        _ => return Err(PageDOMError::FontDescrConversionNoFontName),
    };
    let flags = match d.get_usize(b"Flags") {
        Some(i) => {
            let check_bit = |i: usize, bit: usize| -> bool { (i >> bit) & 0x01 == 0x01 };
            let mut flags = BTreeSet::new();
            if check_bit(i, 0) {
                flags.insert(FontFlag::FixedPitch);
            }
            if check_bit(i, 1) {
                flags.insert(FontFlag::Serif);
            }
            if check_bit(i, 2) {
                flags.insert(FontFlag::Symbolic);
            }
            if check_bit(i, 3) {
                flags.insert(FontFlag::Script);
            }
            if check_bit(i, 5) {
                flags.insert(FontFlag::Nonsymbolic);
            }
            if check_bit(i, 6) {
                flags.insert(FontFlag::Italic);
            }
            if check_bit(i, 16) {
                flags.insert(FontFlag::AllCap);
            }
            if check_bit(i, 17) {
                flags.insert(FontFlag::SmallCap);
            }
            if check_bit(i, 18) {
                flags.insert(FontFlag::ForceBold);
            }
            flags
        },
        None => return Err(PageDOMError::FontDescrConversionNoFlags),
    };
    let fontfile = d.get_ref(b"FontFile").map(|r| r.id());
    let fontfile2 = d.get_ref(b"FontFile2").map(|r| r.id());
    let fontfile3 = d.get_ref(b"FontFile3").map(|r| r.id());
    Ok(FontDescriptor {
        fontname,
        flags,
        fontfile,
        fontfile2,
        fontfile3,
    })
}

// Encoding: Section 7.6.5, page 322 (2020 edn)
fn to_encoding(
    ctxt: &PDFObjContext, o: &Rc<LocatedVal<PDFObjT>>,
) -> Result<FontEncoding, LocatedVal<PageDOMError>> {
    match o.val() {
        PDFObjT::Name(n) => match std::str::from_utf8(n.val()) {
            Ok("MacRomanEncoding") => Ok(FontEncoding::MacRoman),
            Ok("MacExpertEncoding") => Ok(FontEncoding::MacExpert),
            Ok("WinAnsiEncoding") => Ok(FontEncoding::WinAnsi),
            Ok(s) => Ok(FontEncoding::Unknown(s.to_string())),
            Err(_) => Err(o.place(PageDOMError::FontDictConversionUnknownEncoding)),
        },
        PDFObjT::Dict(_) => Ok(FontEncoding::Dict(Rc::clone(o))),
        PDFObjT::Reference(r) => match ctxt.lookup_obj(r.id()) {
            Some(o) => to_encoding(ctxt, o),
            None => Err(o.place(PageDOMError::FontDictConversionBadEncoding)),
        },
        _ => Err(o.place(PageDOMError::FontDictConversionBadEncoding)),
    }
}

// Type 1 fonts: Table 109, page 313 (2020 edn)
// Type 3 fonts: Table 110, page 317 (2020 edn)
fn obj_to_font_dict(
    ctxt: &PDFObjContext, dom: &mut DOMContext, o: &Rc<LocatedVal<PDFObjT>>,
) -> Result<FontDictionary, LocatedVal<PageDOMError>> {
    match o.val() {
        PDFObjT::Dict(d) => match to_font_dict(ctxt, dom, d) {
            Ok(fd) => Ok(fd),
            Err(e) => Err(o.place(e)),
        },
        _ => Err(o.place(PageDOMError::FontDictConversionBadFontDictionary)),
    }
}
fn to_font_dict(
    ctxt: &PDFObjContext, dom: &mut DOMContext, d: &DictT,
) -> Result<FontDictionary, PageDOMError> {
    let basefont = match d.get_name(b"BaseFont") {
        Some(n) => n.to_vec(),
        None => return Err(PageDOMError::FontDictConversionNoBaseFont),
    };
    let subtype = match d.get_name(b"Subtype") {
        Some(n) => match std::str::from_utf8(n) {
            Ok("Type0") => FontType::Type0,
            Ok("Type1") => FontType::Type1,
            Ok("MMType1") => FontType::MMType1,
            Ok("Type3") => FontType::Type3,
            Ok("TrueType") => FontType::TrueType,
            Ok("CIDFontType0") => FontType::CIDFontType0,
            Ok("CIDFontType1") => FontType::CIDFontType1,
            _ => FontType::Unknown(n.to_vec()),
        },
        _ => return Err(PageDOMError::FontDictConversionNoSubtype),
    };
    // TODO: FontDescriptor indirect resolution should be
    // version-dependent.
    let fontdescriptor = match d.get(b"FontDescriptor") {
        Some(o) => match o.val() {
            PDFObjT::Dict(d) => {
                let fd = to_font_descriptor(d)?;
                Some(Rc::new(fd))
            },
            PDFObjT::Reference(r) =>
            // Resolve this if we haven't already.
            {
                match dom.font_descrs().get(&r.id()) {
                    Some(fd) => Some(Rc::clone(fd)),
                    None => match ctxt.lookup_obj(r.id()) {
                        None => {
                            return Err(PageDOMError::FontDescrConversionUnknownObjectId(r.id()))
                        },
                        Some(o) => match o.val() {
                            PDFObjT::Dict(d) => {
                                let fd = to_font_descriptor(d)?;
                                let fd = Rc::new(fd);
                                dom.font_descrs.insert(r.id(), Rc::clone(&fd));
                                Some(fd)
                            },
                            _ => return Err(PageDOMError::FontDescrConversionBadFontDescr),
                        },
                    },
                }
            },
            _ => return Err(PageDOMError::FontDescrConversionBadFontDescr),
        },
        None => None,
    };
    // Encoding: Table 112, page 323 (2020 edn)
    let encoding = match d.get(b"Encoding") {
        Some(o) => match to_encoding(ctxt, o) {
            Err(e) => return Err(e.val().clone()),
            Ok(e) => Some(e),
        },
        None => None,
    };
    Ok(FontDictionary {
        subtype,
        basefont,
        fontdescriptor,
        encoding,
    })
}

/* Errors reported by the page DOM builder */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PageDOMError {
    CatalogConversionBadCatalog,
    CatalogConversionNoPages,
    CatalogConversionPagesIdNotFound,
    PageTreeNodeNotDict,
    PageTreeNodeUnexpectedType(Vec<u8>),
    PageTreeNodeConversionNoCount,
    PageTreeNodeConversionNoKids,
    PageTreeNodeConversionNoParent,
    PageTreeNodeConversionBadKids,
    PageTreeNodeConversionBadRoot,
    PageTreeNodeConversionBadNode,
    PageNodeConversionNoParent,
    PageNodeConversionNoContents,
    PageNodeConversionBadContents,
    PageNodeConversionBadPage,
    NoObjectType,
    ResourceFontValueUnknownObjectId(ObjectId),
    ResourceFontValueNotDict, // /Resources/Font is not dict
    FontResourceNotDict,      // /Resources/Font/<font-resource-name> is not dict
    FontDescrConversionUnknownObjectId(ObjectId),
    FontDescrConversionNoFontName,
    FontDescrConversionNoFlags,
    FontDescrConversionBadFontDescr,
    FontDictConversionNoBaseFont,
    FontDictConversionNoSubtype,
    FontDictConversionBadFontDescriptor,
    FontDictConversionNoEncoding,
    FontDictConversionUnknownEncoding,
    FontDictConversionBadEncoding,
    FontDictConversionBadFontDictionary,
    FontDictUnresolvedId(ObjectId),
}

// The DOM constructor builds the DOM starting from the '/Root'
// object.  The built page DOM consists of all the pages getting
// mapped in DOMContext.  The fonts structures are not mapped here,
// and need a separate pass.

pub fn to_page_dom(
    ctxt: &PDFObjContext, o: &LocatedVal<PDFObjT>,
) -> Result<(Catalog, DOMContext), LocatedVal<PageDOMError>> {
    let mut q = ConversionQ::new();
    let mut dom = DOMContext::new();

    let c = to_catalog(ctxt, &mut q, &mut dom, o)?;

    // TODO: "A page tree shall not contain multiple indirect
    // references to the same page tree node." (Section 7.7.3.2)
    while !q.is_empty() {
        let (id, r, o) = q.next().unwrap();
        match o.val() {
            PDFObjT::Dict(d) => {
                match d.get_name(b"Type") {
                    Some(b"Pages") => {
                        let ptn = to_page_tree_node(ctxt, &mut q, &mut dom, &r, &o);
                        match ptn {
                            Ok(n) => {
                                // ignore return value
                                dom.pages.insert(id, PageKid::Node(Rc::new(n)));
                            },
                            Err(e) => return Err(e),
                        }
                    },
                    Some(b"Page") => {
                        let pn = to_page(ctxt, &mut dom, &r, &o);
                        match pn {
                            Ok(p) => {
                                dom.pages.insert(id, PageKid::Leaf(Rc::new(p)));
                            },
                            Err(e) => return Err(e),
                        }
                    },
                    Some(t) => {
                        return Err(o.place(PageDOMError::PageTreeNodeUnexpectedType(t.to_vec())))
                    },
                    None => return Err(o.place(PageDOMError::NoObjectType)),
                }
            },
            _ => return Err(o.place(PageDOMError::PageTreeNodeNotDict)),
        }
    }
    Ok((c, dom))
}
