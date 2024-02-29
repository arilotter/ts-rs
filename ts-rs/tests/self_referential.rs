#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "tests-out/self_referential/")]
struct HasT {
    t: &'static T<'static>,
}

#[derive(TS)]
#[ts(export, export_to = "tests-out/self_referential/")]
struct T<'a> {
    t_box: Box<T<'a>>,
    self_box: Box<Self>,

    t_ref: &'a T<'a>,
    self_ref: &'a Self,

    t_arc: Arc<T<'a>>,
    self_arc: Arc<Self>,

    #[ts(inline)]
    has_t: HasT,
}

#[test]
fn named() {
    assert_eq!(
        T::decl(),
        "type T = { \
            t_box: T, \
            self_box: T, \
            t_ref: T, \
            self_ref: T, \
            t_arc: T, \
            self_arc: T, \
            has_t: { t: T, }, \
         };"
    );
}

#[derive(TS)]
#[ts(export, export_to = "tests-out/self_referential/")]
enum ExternallyTagged {
    A(Box<ExternallyTagged>),
    B(&'static ExternallyTagged),
    C(Box<Self>),
    D(&'static Self),
    E(
        Box<ExternallyTagged>,
        Box<Self>,
        &'static ExternallyTagged,
        &'static Self,
    ),
    F {
        a: Box<Self>,
        b: &'static ExternallyTagged,
        c: HashMap<String, ExternallyTagged>,
        d: Option<Arc<ExternallyTagged>>,
        #[ts(optional = nullable)]
        e: Option<Arc<ExternallyTagged>>,
        #[ts(optional)]
        f: Option<Arc<ExternallyTagged>>,
    },

    G(
        Vec<ExternallyTagged>,
        [&'static ExternallyTagged; 1024],
        HashMap<String, ExternallyTagged>,
    ),
}

#[test]
fn enum_externally_tagged() {
    assert_eq!(
        ExternallyTagged::decl(),
       "type ExternallyTagged = { \"A\": T } | \
                                { \"B\": T } | \
                                { \"C\": T } | \
                                { \"D\": T } | \
                                { \"E\": [T, T, T, T] } | \
                                { \"F\": { a: T, b: T, c: Record<string, T>, d: T | null, e?: T | null, f?: T, } } | \
                                { \"G\": [Array<T>, Array<T>, Record<string, T>] };"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[ts(export, export_to = "tests-out/self_referential/")]
#[cfg_attr(feature = "serde-compat", serde(tag = "tag"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "tag"))]
enum InternallyTagged {
    A(Box<InternallyTagged>),
    B(&'static InternallyTagged),
    C(Box<Self>),
    D(&'static Self),
    E(Vec<Self>),
    F {
        a: Box<Self>,
        b: &'static InternallyTagged,
        c: HashMap<InternallyTagged, InternallyTagged>,
        d: Option<&'static InternallyTagged>,
        #[ts(optional = nullable)]
        e: Option<&'static InternallyTagged>,
        #[ts(optional)]
        f: Option<&'static InternallyTagged>,
    },
}

// NOTE: The generated type is actually not valid TS here, since the indirections rust enforces for recursive types
//       gets lost during the translation to TypeScript (e.g "Box<T>" => "T").
#[test]
fn enum_internally_tagged() {
    assert_eq!(
        InternallyTagged::decl(),
        "type InternallyTagged = { \"tag\": \"A\" } & T | \
                                 { \"tag\": \"B\" } & T | \
                                 { \"tag\": \"C\" } & T | \
                                 { \"tag\": \"D\" } & T | \
                                 { \"tag\": \"E\" } & Array<T> | \
                                 { \"tag\": \"F\", a: T, b: T, c: Record<T, T>, d: T | null, e?: T | null, f?: T, };"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[ts(export, export_to = "tests-out/self_referential/")]
#[cfg_attr(feature = "serde-compat", serde(tag = "tag", content = "content"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "tag", content = "content"))]
enum AdjacentlyTagged {
    A(Box<AdjacentlyTagged>),
    B(&'static AdjacentlyTagged),
    C(Box<Self>),
    D(&'static Self),
    E(Vec<Self>),
    F {
        a: Box<Self>,
        b: &'static AdjacentlyTagged,
        c: HashMap<String, AdjacentlyTagged>,
        d: Option<&'static AdjacentlyTagged>,
        #[ts(optional = nullable)]
        e: Option<&'static AdjacentlyTagged>,
        #[ts(optional)]
        f: Option<&'static AdjacentlyTagged>,
    },
    G(
        Vec<Self>,
        [&'static AdjacentlyTagged; 4],
        HashMap<String, AdjacentlyTagged>,
    ),
}

// NOTE: The generated type is actually not valid TS here, since the indirections rust enforces for recursive types
//       gets lost during the translation to TypeScript (e.g "Box<T>" => "T").
#[test]
fn enum_adjacently_tagged() {
    assert_eq!(
        AdjacentlyTagged::decl(),
        "type AdjacentlyTagged = { \"tag\": \"A\", \"content\": T } | \
                                 { \"tag\": \"B\", \"content\": T } | \
                                 { \"tag\": \"C\", \"content\": T } | \
                                 { \"tag\": \"D\", \"content\": T } | \
                                 { \"tag\": \"E\", \"content\": Array<T> } | \
                                 { \
                                    \"tag\": \"F\", \
                                    \"content\": { \
                                        a: T, \
                                        b: T, \
                                        c: Record<string, T>, \
                                        d: T | null, \
                                        e?: T | null, \
                                        f?: T, \
                                    } \
                                 } | \
                                 { \
                                    \"tag\": \"G\", \
                                    \"content\": [\
                                        Array<T>, \
                                        [T, T, T, T], \
                                        Record<string, T>\
                                    ] \
                                 };"
    );
}
