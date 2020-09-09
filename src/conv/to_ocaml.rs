// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use crate::memory::{
    alloc_bytes, alloc_cons, alloc_double, alloc_int32, alloc_int64, alloc_some, alloc_string,
    alloc_tuple, alloc_tuple_3, alloc_tuple_4, OCamlAllocResult, OCamlAllocToken,
};
use crate::mlvalues::{
    Intnat, OCamlBytes, OCamlInt32, OCamlInt64, OCamlList, RawOCaml, FALSE, NONE, TRUE,
};
use crate::value::OCaml;
use crate::{ocaml_alloc, ocaml_frame};

/// Implements conversion from Rust values into OCaml values.
pub unsafe trait ToOCaml<T> {
    /// Convert to OCaml value.
    fn to_ocaml(&self, gc: OCamlAllocToken) -> OCamlAllocResult<T>;
}

unsafe impl ToOCaml<Intnat> for i64 {
    fn to_ocaml(&self, _token: OCamlAllocToken) -> OCamlAllocResult<Intnat> {
        OCamlAllocResult::of(((self << 1) | 1) as RawOCaml)
    }
}

unsafe impl ToOCaml<Intnat> for i32 {
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<Intnat> {
        (*self as i64).to_ocaml(token)
    }
}

unsafe impl ToOCaml<OCamlInt32> for i32 {
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<OCamlInt32> {
        alloc_int32(token, *self)
    }
}

unsafe impl ToOCaml<OCamlInt64> for i64 {
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<OCamlInt64> {
        alloc_int64(token, *self)
    }
}

unsafe impl ToOCaml<f64> for f64 {
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<f64> {
        alloc_double(token, *self)
    }
}

unsafe impl ToOCaml<bool> for bool {
    fn to_ocaml(&self, _token: OCamlAllocToken) -> OCamlAllocResult<bool> {
        OCamlAllocResult::of(if *self { TRUE } else { FALSE })
    }
}

unsafe impl<T: AsRef<str>> ToOCaml<String> for T {
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<String> {
        alloc_string(token, self.as_ref())
    }
}

unsafe impl<T: AsRef<[u8]>> ToOCaml<OCamlBytes> for T {
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<OCamlBytes> {
        alloc_bytes(token, self.as_ref())
    }
}

unsafe impl<A, ToA> ToOCaml<Option<ToA>> for Option<A>
where
    A: ToOCaml<ToA>,
{
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<Option<ToA>> {
        if let Some(value) = self {
            ocaml_frame!(gc, {
                let ocaml_value = ocaml_alloc!(value.to_ocaml(gc));
                alloc_some(token, ocaml_value)
            })
        } else {
            OCamlAllocResult::of(NONE)
        }
    }
}

unsafe impl<A, B, ToA, ToB> ToOCaml<(ToA, ToB)> for (A, B)
where
    A: ToOCaml<ToA>,
    B: ToOCaml<ToB>,
{
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<(ToA, ToB)> {
        ocaml_frame!(gc, {
            let fst = ocaml_alloc!((self.0).to_ocaml(gc));
            let ref fst_ref = gc.keep(fst);
            let snd = ocaml_alloc!((self.1).to_ocaml(gc));
            alloc_tuple(token, gc.get(fst_ref), snd)
        })
    }
}

unsafe impl<A, B, C, ToA, ToB, ToC> ToOCaml<(ToA, ToB, ToC)> for (A, B, C)
where
    A: ToOCaml<ToA>,
    B: ToOCaml<ToB>,
    C: ToOCaml<ToC>,
{
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<(ToA, ToB, ToC)> {
        ocaml_frame!(gc, {
            let fst = ocaml_alloc!((self.0).to_ocaml(gc));
            let ref fst_ref = gc.keep(fst);
            let snd = ocaml_alloc!((self.1).to_ocaml(gc));
            let ref snd_ref = gc.keep(snd);
            let elt3 = ocaml_alloc!((self.2).to_ocaml(gc));
            alloc_tuple_3(token, gc.get(fst_ref), gc.get(snd_ref), elt3)
        })
    }
}

unsafe impl<A, B, C, D, ToA, ToB, ToC, ToD> ToOCaml<(ToA, ToB, ToC, ToD)> for (A, B, C, D)
where
    A: ToOCaml<ToA>,
    B: ToOCaml<ToB>,
    C: ToOCaml<ToC>,
    D: ToOCaml<ToD>,
{
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<(ToA, ToB, ToC, ToD)> {
        ocaml_frame!(gc, {
            let fst = ocaml_alloc!((self.0).to_ocaml(gc));
            let ref fst_ref = gc.keep(fst);
            let snd = ocaml_alloc!((self.1).to_ocaml(gc));
            let ref snd_ref = gc.keep(snd);
            let elt3 = ocaml_alloc!((self.2).to_ocaml(gc));
            let ref elt3_ref = gc.keep(elt3);
            let elt4 = ocaml_alloc!((self.3).to_ocaml(gc));
            alloc_tuple_4(
                token,
                gc.get(fst_ref),
                gc.get(snd_ref),
                gc.get(elt3_ref),
                elt4,
            )
        })
    }
}

unsafe impl<A, ToA> ToOCaml<OCamlList<ToA>> for Vec<A>
where
    A: ToOCaml<ToA>,
{
    fn to_ocaml(&self, token: OCamlAllocToken) -> OCamlAllocResult<OCamlList<ToA>> {
        (&self).to_ocaml(token)
    }
}

unsafe impl<A, ToA> ToOCaml<OCamlList<ToA>> for &Vec<A>
where
    A: ToOCaml<ToA>,
{
    fn to_ocaml(&self, _token: OCamlAllocToken) -> OCamlAllocResult<OCamlList<ToA>> {
        ocaml_frame!(gc, {
            let ref mut result_ref = gc.keep(OCaml::nil());
            for elt in self.iter().rev() {
                let ov = ocaml_alloc!(elt.to_ocaml(gc));
                let cons = ocaml_alloc!(alloc_cons(gc, ov, gc.get(result_ref)));
                result_ref.set(cons);
            }
            OCamlAllocResult::of_ocaml(gc.get(result_ref))
        })
    }
}
