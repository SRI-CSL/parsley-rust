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

use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

use super::super::pcore::parsebuffer::LocatedVal;
use super::pdf_obj::{DictKey, DictT, ObjectId, PDFObjContext, PDFObjT};

// DOM context, a more specialized version of PDFObjContext
pub struct DOMContext {
    // pages does not contain the root page, since that is contained
    // within the Catalog.
    pages: BTreeMap<ObjectId, PageKid>,
}
impl DOMContext {
    pub fn new() -> Self {
        Self {
            pages: BTreeMap::new(),
        }
    }
    pub fn pages(&self) -> &BTreeMap<ObjectId, PageKid> { &self.pages }
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Resources {
    map: BTreeMap<DictKey, Rc<LocatedVal<PDFObjT>>>,
}
impl Resources {
    pub fn new(d: &DictT) -> Self {
        let mut map = BTreeMap::new();
        for (k, v) in d.map().iter() {
            map.insert(k.clone(), Rc::clone(v));
        }
        Self { map }
    }
    pub fn new_default() -> Self {
        Self {
            map: BTreeMap::new(),
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

pub struct ConversionQ {
    page_nodes: VecDeque<(ObjectId, Option<Rc<Resources>>, Rc<LocatedVal<PDFObjT>>)>,
}
impl ConversionQ {
    pub fn new() -> Self {
        Self {
            page_nodes: VecDeque::new(),
        }
    }
}

pub fn to_catalog(
    ctxt: &PDFObjContext, q: &mut ConversionQ, o: &LocatedVal<PDFObjT>,
) -> Option<Catalog> {
    match o.val() {
        PDFObjT::Dict(d) => d.get_ref(b"Pages").and_then(|r| {
            ctxt.lookup_obj(r.id()).and_then(|o| {
                to_root_page_tree_node(ctxt, q, o).and_then(|p| Some(Catalog::new(Rc::new(p))))
            })
        }),
        _ => None,
    }
}

pub fn to_page_kids(
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
                        match ctxt.lookup_obj(rf.id()) {
                            Some(o) => match r {
                                None => q.page_nodes.push_back((rf.id(), None, Rc::clone(o))),
                                Some(r) => q.page_nodes.push_back((
                                    rf.id(),
                                    Some(Rc::clone(r)),
                                    Rc::clone(o),
                                )),
                            },
                            None => (),
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

pub fn to_root_page_tree_node(
    ctxt: &PDFObjContext, q: &mut ConversionQ, o: &LocatedVal<PDFObjT>,
) -> Option<RootPageTreeNode> {
    match o.val() {
        PDFObjT::Dict(d) => {
            let res = match d.get_resolved_dict(ctxt, b"Resources") {
                None => None,
                Some(d) => Some(Rc::new(Resources::new(d))),
            };
            let count = match d.get_usize(b"Count") {
                None => return None,
                Some(i) => i,
            };
            let kids = match d.get(b"Kids") {
                None => return None,
                Some(o) => to_page_kids(ctxt, q, &res, o),
            };
            match kids {
                None => None,
                Some(v) => Some(RootPageTreeNode::new(res, count, v)),
            }
        },
        _ => None,
    }
}

pub fn to_page_tree_node(
    ctxt: &PDFObjContext, q: &mut ConversionQ, r: &Option<Rc<Resources>>, o: &LocatedVal<PDFObjT>,
) -> Option<PageTreeNode> {
    match o.val() {
        PDFObjT::Dict(d) => {
            let parent = match d.get_ref(b"Parent") {
                None => return None,
                Some(p) => p.id(),
            };
            let res = match (d.get_resolved_dict(ctxt, b"Resources"), r) {
                (Some(d), _) => Some(Rc::new(Resources::new(d))),
                (_, Some(r)) => Some(Rc::clone(r)),
                (_, _) => None,
            };
            let count = match d.get_usize(b"Count") {
                None => return None,
                Some(i) => i,
            };
            let kids = match d.get(b"Kids") {
                None => return None,
                Some(o) => to_page_kids(ctxt, q, &res, o),
            };
            match kids {
                None => None,
                Some(v) => Some(PageTreeNode::new(parent, res, count, v)),
            }
        },
        _ => None,
    }
}

pub fn to_page_content(
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

pub fn to_page_contents(
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

pub fn to_page(
    ctxt: &PDFObjContext, r: &Option<Rc<Resources>>, o: &LocatedVal<PDFObjT>,
) -> Option<Page> {
    match o.val() {
        PDFObjT::Dict(d) => {
            let parent = match d.get_ref(b"Parent") {
                None => return None,
                Some(p) => p.id(),
            };
            let res = match (d.get_resolved_dict(ctxt, b"Resources"), r) {
                (Some(d), _) => Rc::new(Resources::new(d)),
                (_, Some(r)) => Rc::clone(r),
                (_, _) => Rc::new(Resources::new_default()),
            };
            let contents = match d.get(b"Contents") {
                None => return None,
                Some(o) => match to_page_contents(ctxt, o) {
                    None => return None,
                    Some(v) => v,
                },
            };
            Some(Page::new(parent, res, contents))
        },
        _ => None,
    }
}

/* Errors reported by the page DOM builder */
#[derive(Debug, PartialEq)]
pub enum PageDomError {
    CatalogConversion,
    PageTreeNodeConversion,
    PageNodeConversion,
    NoObjectType,
    UnexpectedType(Vec<u8>),
    NonDictPageNode,
}

// The DOM constructor, which starts from the '/Root' object.

pub fn to_page_dom(
    ctxt: &PDFObjContext, o: &LocatedVal<PDFObjT>,
) -> Result<(Catalog, DOMContext), LocatedVal<PageDomError>> {
    let mut q = ConversionQ::new();
    let c = to_catalog(ctxt, &mut q, o);
    if c.is_none() {
        return Err(o.place(PageDomError::CatalogConversion))
    }
    let c = c.unwrap();

    // TODO: "A page tree shall not contain multiple indirect
    // references to the same page tree node." (Section 7.7.3.2)
    let mut dom_ctxt = DOMContext::new();
    while !q.page_nodes.is_empty() {
        let (id, r, o) = q.page_nodes.pop_front().unwrap();
        match o.val() {
            PDFObjT::Dict(d) => {
                match d.get_name(b"Type") {
                    Some(b"Pages") => {
                        let ptn = to_page_tree_node(ctxt, &mut q, &r, &o);
                        match ptn {
                            Some(n) => {
                                // ignore return value
                                dom_ctxt.pages.insert(id, PageKid::Node(Rc::new(n)));
                            },
                            None => return Err(o.place(PageDomError::PageTreeNodeConversion)),
                        }
                    },
                    Some(b"Page") => {
                        let pn = to_page(ctxt, &r, &o);
                        match pn {
                            Some(p) => {
                                dom_ctxt.pages.insert(id, PageKid::Leaf(Rc::new(p)));
                            },
                            None => return Err(o.place(PageDomError::PageNodeConversion)),
                        }
                    },
                    Some(t) => return Err(o.place(PageDomError::UnexpectedType(t.to_vec()))),
                    None => return Err(o.place(PageDomError::NoObjectType)),
                }
            },
            _ =>
                return Err(o.place(PageDomError::NonDictPageNode))
        }
    }
    Ok((c, dom_ctxt))
}
