/*! Static (typing-time) substitutions. */

//use std::fmt;
use std::rc::Rc;
use crate::{normal, ast::*, bitype::{Term}};

// TODO-Someday: Use Rc-based lists instead vectors to represent the
// bound variable list, for cheaper O(1) clones.


/// Simplistic policy to create a "primed" version of `x`
pub fn alpha_vary(x:&String) -> String {
    format!("{}~",x)
}    

/// Test if a variable term appears in a vector of variable terms
pub fn fv_contains(fv:&Vec<Term>, x:&Term) -> bool {
    for y in fv.iter() {
        if y == x { return true }
    };
    return false
}

/// Test if an index term variable appears in a vector of variable terms
pub fn fv_contains_idxtm(fv:&Vec<Term>, x:&String) -> bool {
    let x = &Term::IdxTm(IdxTm::Var(x.clone()));
    fv_contains(fv, x)
}


/// Compute a list of variables (as terms) that appear free in the given term
pub fn fv_of_term(t:&Term) -> Vec<Term> {
    let mut out : Vec<Term> = vec![];
    match t {
        &Term::NmTm (ref n) => fv_of_nmtm (n, vec![], &mut out),
        &Term::IdxTm(ref i) => fv_of_idxtm(i, vec![], &mut out),
        &Term::Type (ref t) => fv_of_type (t, vec![], &mut out),        
    }
    return out
}

/// Compute the free variables of a name term
pub fn fv_of_nmtm(n:&NameTm, bound:Vec<Term>, out:&mut Vec<Term>) {
    use crate::ast::NameTm::*;
    match n {
        &Var(_) |
        &ValVar(_) => {
            let x = Term::NmTm(n.clone());
            if let Some(_) = bound.iter().position(|y| &x == y) { /* x is bound. */ }
            else { /* x is free */ out.push(x) }
        },
        &Name(_) => {},
        &Bin(ref n, ref m) |
        &App(ref n, ref m) => {
            fv_of_nmtm(n, bound.clone(), out);
            fv_of_nmtm(m, bound,         out);
        }
        &Lam(ref x, _, ref m) => {
            let mut bound = bound;
            bound.push(Term::NmTm(NameTm::Var(x.clone())));
            fv_of_nmtm(m, bound, out);
        }
        &WriteScope => { out.push(Term::NmTm(
            NameTm::WriteScope
        )) },
        &NoParse(_) => { },
        &Ident(_) => { },
    }
}

/// Compute the free variables of an index term
pub fn fv_of_idxtm(i:&IdxTm, bound:Vec<Term>, out:&mut Vec<Term>) {
    use crate::ast::IdxTm::*;
    match i {
        &Unknown => { },          
        &Var(ref _x) => {
            let x = Term::IdxTm(i.clone());
            if let Some(_) = bound.iter().position(|y| &x == y) { /* x is bound. */ }
            else { /* x is free */ out.push(x) }
        }
        &Lam(ref x, ref _g, ref i) => {
            let mut bound = bound;
            bound.push(Term::IdxTm(IdxTm::Var(x.clone())));
            fv_of_idxtm(i, bound, out);
        }
        &Ident(_)   |
        &NoParse(_) |
        &Unit       |
        &Empty      => {}
        &WriteScope => { out.push(Term::IdxTm(IdxTm::WriteScope)) }
        &Sing(ref n) => {
            fv_of_nmtm(n, bound, out)
        }
        &Map(ref n, ref i)     |
        &MapStar(ref n, ref i) => {
            fv_of_nmtm(n, bound.clone(), out);
            fv_of_idxtm(i, bound,        out);
        }
        &Proj1(ref i) |
        &Proj2(ref i) => {
            fv_of_idxtm(i, bound, out);
        }
        &NmSet(ref nms) => {
            use crate::normal::NmSetTm::*;
            for t in nms.terms.iter() {
                match t {
                    &Single(ref n) => { fv_of_nmtm(n, bound.clone(), out)  }
                    &Subset(ref i) => { fv_of_idxtm(i, bound.clone(), out) }
                };
            }
        }
        &NmTm(ref n) => { fv_of_nmtm(n, bound, out) }
        &Apart(ref i, ref j)   |
        &Union(ref i, ref j)   |
        &Bin(ref i, ref j)     |
        &Pair(ref i, ref j)    |
        &App(ref i, ref j)     |
        &FlatMap(ref i, ref j) |
        &FlatMapStar(ref i, ref j)    => {
            fv_of_idxtm(i, bound.clone(), out);
            fv_of_idxtm(j, bound        , out);
        }
    }
}
pub fn fv_of_type(_t:&Type, _bound:Vec<Term>, _out:&mut Vec<Term>) {
    // XXX
    //unimplemented!()
    // TODO!
}
pub fn fv_of_ctype(_ct:&CType, _bound:Vec<Term>, _out:&mut Vec<Term>) {
    unimplemented!()
}
pub fn fv_of_ceffect(_ct:&CEffect, _bound:Vec<Term>, _out:&mut Vec<Term>) {
    unimplemented!()
}
pub fn fv_of_effect(_ct:&Effect, _bound:Vec<Term>, _out:&mut Vec<Term>) {
    unimplemented!()
}




/// Substitute a type for a type variable in another type
pub fn subst_type_type(a:Type, x:&String, b:Type) -> Type {
    subst_term_type(Term::Type(a), x, b)
}

/// Substitute an index for an index variable in another type
pub fn subst_idxtm_type(i:IdxTm, x:&String, b:Type) -> Type {
    subst_term_type(Term::IdxTm(i), x, b)
}


/// Predicate for type terms
pub fn term_is_type(t:&Term) -> bool {
    match t { &Term::Type(_) => true, _ => false }
}

/// Predicate for index terms
pub fn term_is_idxtm(t:&Term) -> bool {
    match t { &Term::IdxTm(_) => true, _ => false }
}

/// Predicate for name terms
pub fn term_is_nmtm(t:&Term) -> bool {
    match t { &Term::NmTm(_) => true, _ => false }
}

/// Substitute terms into types
pub fn subst_term_type_rec(t:Term, x:&String, a:Rc<Type>) -> Rc<Type> {
    Rc::new(subst_term_type(t, x, (*a).clone()))
}

/// Substitute terms into computation types
pub fn subst_term_ctype(t:Term, x:&String, ct:CType) -> CType {
    match ct {
        CType::Lift(a) => {
            CType::Lift(subst_term_type(t,x,a))
        }
        CType::Arrow(a, ce) => {
            CType::Arrow(subst_term_type(t.clone(),x,a),
                         subst_term_ceffect_rec(t,x,ce))
        }
        CType::NoParse(s) => CType::NoParse(s)
    }
}

/// Substitute terms into effects
pub fn subst_term_effect_rec(t:Term, x:&String, eff:Rc<Effect>) -> Rc<Effect> {
    Rc::new(subst_term_effect(t, x, (*eff).clone()))
}

/// Substitute terms into effects
pub fn subst_term_effect(t:Term, x:&String, eff:Effect) -> Effect {
    match eff {
        Effect::WR(i, j) => {
            Effect::WR(subst_term_idxtm(t.clone(), x, i),
                       subst_term_idxtm(t, x, j))
        },
        // Effect::Then(e1, e2) => {
        //     Effect::Then(subst_term_effect_rec(t.clone(), x, e1),
        //                  subst_term_effect_rec(t, x, e2))
        // }
        Effect::NoParse(s) => Effect::NoParse(s)
    }
}

/// Substitute terms into computation effects 
pub fn subst_term_ceffect_rec(t:Term, x:&String, ce:Rc<CEffect>) -> Rc<CEffect> {
    Rc::new(subst_term_ceffect(t, x, (*ce).clone()))
}

/// Substitute terms into computation effects 
pub fn subst_term_ceffect(t:Term, x:&String, ce:CEffect) -> CEffect {
    match ce {
        CEffect::Cons(ct, eff) => {
            CEffect::Cons(subst_term_ctype(t.clone(), x, ct),
                          subst_term_effect(t, x, eff))
        }
        CEffect::ForallType(y, k, ce) => {
            if term_is_type(&t) && x == &y {
                CEffect::ForallType(y, k, ce)
            } else {
                CEffect::ForallType(y, k, subst_term_ceffect_rec(t, x, ce))
            }
        }
        CEffect::ForallIdx(y, g, p, ce) => {
            if term_is_idxtm(&t) && x == &y {
                CEffect::ForallIdx(y, g, p, ce)
            } else {
                CEffect::ForallIdx(y, g,
                                   subst_term_prop(t.clone(), x, p),
                                   subst_term_ceffect_rec(t, x, ce))
            }
        }
        CEffect::NoParse(s) => CEffect::NoParse(s)
    }
}

/// Substitute terms into types
pub fn subst_term_type(t:Term, x:&String, a:Type) -> Type {
    match a {
        Type::Unit => Type::Unit,
        Type::Abstract(y) => Type::Abstract(y),
        Type::Ident(y) => Type::Ident(y),
        Type::IdentUndef(y) => Type::IdentUndef(y),
        Type::Prim(pt) => Type::Prim(pt),
        Type::IdentDef(y, ydef) => {
            // If the substitution of [t/x] changes the body
            // of the definition, then we "forget" the
            // identifier x.  Otherwise, we keep it around.
            let ydef2 = subst_term_type(t, x, (*ydef).clone());
            if *ydef != ydef2 { ydef2 } else {
                Type::IdentDef(y, ydef)
            }
        }
        Type::Var(y) => {
            if term_is_type(&t) && x == &y {
                match t {
                    Term::Type(b) => b.clone(),
                    _ => unreachable!(),
                }
            }
            else {
                Type::Var(y)
            }
        }
        Type::Sum(a1, a2) => {
            Type::Sum(
                subst_term_type_rec(t.clone(), x, a1),
                subst_term_type_rec(t,         x, a2),
            )
        }
        Type::Prod(a1, a2) => {
            Type::Prod(
                subst_term_type_rec(t.clone(), x, a1),
                subst_term_type_rec(t,         x, a2),
            )
        }
        Type::Ref(i, a0) => {
            Type::Ref(
                subst_term_idxtm(t.clone(), x, i),
                subst_term_type_rec(t, x, a0),
            )
        }
        Type::Thk(i, ce) => {
            //
            // The scope of the write scope variable `WriteScope`:
            // ---------------------------------------------------
            //
            // Conceptually, there is a hidden binder to bind the `@@`
            // and `@!` variables for each `Thk` type (determined by
            // the allocation context's dynamic write scope); these
            // variables may appear free in each `ce`.
            //
            let ce = {
                if x == idxtm_writescope_var_str() ||
                    x == nmtm_writescope_var_str()
                {
                    ce
                } else {
                    subst_term_ceffect_rec(t.clone(), x, ce)
                }
            };
            Type::Thk(
                subst_term_idxtm(t, x, i),
                ce,
            )
        }        
        Type::IdxApp(a0, i) => {
            Type::IdxApp(
                subst_term_type_rec(t.clone(), x, a0),
                subst_term_idxtm(t, x, i)
            )
        }
        Type::TypeApp(a1, a2) => {
            Type::TypeApp(
                subst_term_type_rec(t.clone(), x, a1),
                subst_term_type_rec(t, x, a2)
            )
        }
        Type::Nm(i) => {
            Type::Nm(subst_term_idxtm(t, x, i))
        }
        Type::NmFn(n) => {
            Type::NmFn(subst_term_nmtm(t, x, n))
        }
        Type::Rec(y, a1) => {
            if term_is_type(&t) && x == &y {
                Type::Rec(y, a1)
            } else {
                Type::Rec(y, subst_term_type_rec(t, x, a1))
            }            
        }
        Type::TypeFn(y, k, a1) => {
            if term_is_type(&t) && x == &y {
                Type::TypeFn(y, k, a1)
            } else {
                Type::TypeFn(y, k, subst_term_type_rec(t, x, a1))
            }
        }
        Type::IdxFn(y, g, a1) => {
            if term_is_idxtm(&t) && x == &y {
                Type::IdxFn(y, g, a1)
            } else {
                let fv = fv_of_term(&t);
                if fv_contains_idxtm(&fv, &y) {
                    // When free variables of `t` contains var `y`, we
                    // need to alpha-vary `y` before substituting into `a1`.
                    let y_ = alpha_vary(&y);
                    assert!(!fv_contains_idxtm(&fv, &y_));
                    let t_y_ = Term::IdxTm(IdxTm::Var(y_.clone()));
                    let a1 = subst_term_type_rec(t_y_, &y, a1);
                    Type::IdxFn(y_, g, subst_term_type_rec(t, x, a1))
                        
                } else {
                    Type::IdxFn(y, g, subst_term_type_rec(t, x, a1))
                }
            }
        }
        //  [i/b] (exists a:g |      p.      A)
        //     == (exists a:g | [i/b]p. [i/b]A)
        //
        Type::Exists(y, g, p, a1) => {
            if term_is_idxtm(&t) && x == &y {
                Type::Exists(y, g, p, a1)
            } else {
                let fv = fv_of_term(&t);
                if fv_contains_idxtm(&fv, &y) {
                    let y_ = alpha_vary(&y);
                    assert!(!fv_contains_idxtm(&fv, &y_));
                    let t_y_ = Term::IdxTm(IdxTm::Var(y_.clone()));
                    let a1 = subst_term_type_rec(t_y_.clone(), &y, a1);
                    let p  = subst_term_prop(t_y_, &y, p);
                    Type::Exists(
                        y_, g,
                        subst_term_prop(t.clone(), x, p),
                        subst_term_type_rec(t, x, a1),
                    )
                } else {
                    Type::Exists(
                        y, g,
                        subst_term_prop(t.clone(), x, p),
                        subst_term_type_rec(t, x, a1),
                    )
                }
            }
        }
        Type::NoParse(s) => Type::NoParse(s)
    }
}

/// Substitute terms into propositions
pub fn subst_term_prop(t:Term, x:&String, p:Prop) -> Prop {
    match p {
        Prop::Tt => Prop::Tt,
        Prop::Equiv(i, j, g) => {
            Prop::Equiv(
                subst_term_idxtm(t.clone(), x, i),
                subst_term_idxtm(t        , x, j),
                g)
        },
        Prop::Apart(i, j, g) => {
            Prop::Apart(
                subst_term_idxtm(t.clone(), x, i),
                subst_term_idxtm(t        , x, j),
                g)
        },
        Prop::Conj(p1, p2) => {
            Prop::Conj(
                subst_term_prop_rec(t.clone(), x, p1),
                subst_term_prop_rec(t        , x, p2),
            )
        }
        Prop::NoParse(s) => Prop::NoParse(s)
    }
}

/// Substitute terms into propositions
pub fn subst_term_prop_rec(t:Term, x:&String, p:Rc<Prop>) -> Rc<Prop> {
    Rc::new(subst_term_prop(t,x,(*p).clone()))
}

/// Substitute terms into index terms
pub fn subst_term_idxtm_rec(t:Term, x:&String, i:Rc<IdxTm>) -> Rc<IdxTm> {
    Rc::new(subst_term_idxtm(t, x, (*i).clone()))
}

/// magic variable name for the write scope function over name sets (`"@!"`)
pub fn idxtm_writescope_var_str() -> &'static str { "@!" }

/// magic variable name for the write scope function over names (`"@@"`)
pub fn nmtm_writescope_var_str() -> &'static str { "@@" }

/// Substitute terms into index terms
pub fn subst_term_idxtm(t:Term, x:&String, i:IdxTm) -> IdxTm {
    // Types never appear in index terms
    if term_is_type(&t) { i.clone() } else { match i {
        // Variables and identifiers are lexically distinct
        IdxTm::Unknown => IdxTm::Unknown,
        IdxTm::Ident(y) => IdxTm::Ident(y),
        IdxTm::WriteScope => {
            if x == idxtm_writescope_var_str() { match t {
                Term::IdxTm(i) => i,
                _ => unreachable!(),
            }} else {
                IdxTm::WriteScope
            }
        },
        IdxTm::Var(y) => {
            if term_is_idxtm(&t) && x == &y {
                match t {
                    Term::IdxTm(i) => i.clone(),
                    _ => unreachable!(),
                }
            }
            else {
                IdxTm::Var(y)
            }
        }
        IdxTm::Sing(n) => {
            IdxTm::Sing(subst_term_nmtm(t,x,n))
        },
        IdxTm::NmTm(n) => {
            IdxTm::NmTm(subst_term_nmtm(t,x,n))
        },
        IdxTm::Unit  => IdxTm::Unit,
        IdxTm::Empty => IdxTm::Empty,
        IdxTm::NmSet(s) => {
            let mut terms = vec![];
            for tm in s.terms {
                let tm = match tm {
                    normal::NmSetTm::Single(n) => {
                        normal::NmSetTm::Single(subst_term_nmtm(t.clone(),x,n))
                    },
                    normal::NmSetTm::Subset(i) => {
                        normal::NmSetTm::Subset(subst_term_idxtm(t.clone(),x,i))
                    }
                };
                terms.push(tm)
            }
            IdxTm::NmSet(normal::NmSet{cons:s.cons, terms:terms})
        }
        IdxTm::Apart(i, j) => {
            IdxTm::Apart(subst_term_idxtm_rec(t.clone(),x,i),
                        subst_term_idxtm_rec(t,x,j))
        }
        IdxTm::Union(i, j) => {
            IdxTm::Union(subst_term_idxtm_rec(t.clone(),x,i),
                         subst_term_idxtm_rec(t,x,j))
        }
        IdxTm::Bin(i, j) => {
            IdxTm::Bin(subst_term_idxtm_rec(t.clone(),x,i),
                       subst_term_idxtm_rec(t,x,j))
        }
        IdxTm::Pair(i, j) => {
            IdxTm::Pair(subst_term_idxtm_rec(t.clone(),x,i),
                        subst_term_idxtm_rec(t,x,j))
        }
        IdxTm::Proj1(i) => {
            IdxTm::Proj1(subst_term_idxtm_rec(t,x,i))
        }
        IdxTm::Proj2(i) => {
            IdxTm::Proj2(subst_term_idxtm_rec(t,x,i))
        }
        IdxTm::Lam(y,g,i) => {
            if term_is_idxtm(&t) && x == &y {
                IdxTm::Lam(y,g,i)
            } else {
                IdxTm::Lam(y,g,subst_term_idxtm_rec(t,x,i))
            }
        }
        IdxTm::App(i, j) => {
            IdxTm::App(subst_term_idxtm_rec(t.clone(), x, i),
                       subst_term_idxtm_rec(t, x, j))
        }
        IdxTm::Map(n, j) => {
            IdxTm::Map(subst_term_nmtm_rec(t.clone(), x, n),
                       subst_term_idxtm_rec(t, x, j))
        }
        IdxTm::MapStar(n, j) => {
            IdxTm::MapStar(subst_term_nmtm_rec(t.clone(), x, n),
                           subst_term_idxtm_rec(t, x, j))
        }
        IdxTm::FlatMap(i, j) => {
            IdxTm::FlatMap(subst_term_idxtm_rec(t.clone(), x, i),
                           subst_term_idxtm_rec(t, x, j))
        }
        IdxTm::FlatMapStar(i, j) => {
            IdxTm::FlatMapStar(subst_term_idxtm_rec(t.clone(), x, i),
                        subst_term_idxtm_rec(t, x, j))
        }
        IdxTm::NoParse(s) =>
            IdxTm::NoParse(s)
    }}
}

/// Substitute terms into name terms
pub fn subst_term_nmtm_rec(t:Term, x:&String, m:Rc<NameTm>) -> Rc<NameTm> {
    Rc::new(subst_term_nmtm(t,x,(*m).clone()))
}


/// Substitute name terms into name terms
pub fn subst_term_nmtm(t:Term, x:&String, m:NameTm) -> NameTm {
    if ! term_is_nmtm(&t) { m.clone() } else { match m {
        NameTm::ValVar(y) => {
            NameTm::ValVar(y)
        }
        NameTm::Ident(y) => {
            NameTm::Ident(y)
        }
        NameTm::Var(y) => {
            if term_is_nmtm(&t) && x == &y {
                match t {
                    Term::NmTm(n) => n,
                    _ => unreachable!(),
                }
            } else {
                NameTm::Var(y)
            }
        }
        NameTm::WriteScope => {
            if x == nmtm_writescope_var_str() { match t {
                Term::NmTm(n) => n,
                _ => unreachable!(),
            }} else {
                NameTm::WriteScope
            }
        },
        NameTm::NoParse(s) => NameTm::NoParse(s),
        NameTm::Name(n) => NameTm::Name(n),
        NameTm::App(n1, n2) => {
            NameTm::App(
                subst_term_nmtm_rec(t.clone(), x, n1),
                subst_term_nmtm_rec(t        , x, n2),
            )                
        }
        NameTm::Bin(n1, n2) => {
            NameTm::Bin(
                subst_term_nmtm_rec(t.clone(), x, n1),
                subst_term_nmtm_rec(t        , x, n2),
            )                
        }
        NameTm::Lam(y,yg,m1) => {
            if term_is_nmtm(&t) && x == &y {
                NameTm::Lam(y,yg,m1)
            }
            else {
                NameTm::Lam(y,yg,
                            subst_term_nmtm_rec(t,x,m1))
            }
        }
    }}
}


/// Substitute name terms into name terms
pub fn subst_nmtm(t:NameTm, x:&String, m:NameTm) -> NameTm {
    subst_term_nmtm(Term::NmTm(t), x, m)
}

/// Substitute name terms into name terms
pub fn subst_nmtm_rec(t:NameTm, x:&String, m:Rc<NameTm>) -> Rc<NameTm> {
    Rc::new(subst_term_nmtm(Term::NmTm(t), x, (*m).clone()))
}

