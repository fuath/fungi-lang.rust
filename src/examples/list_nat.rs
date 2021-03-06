//
// Fungi module: linked lists with names, holding natural numbers
//
fgi_mod!{
    open crate::examples::op_nat;

    /// Lists of natural numbers
    type List  = (
        rec list. foralli (X,Y):NmSet.
            (+ Unit +
             (exists (X1,X2):NmSet | ((X1%X2)=X:NmSet).
              exists (Y1,Y2):NmSet | ((Y1%Y2)=Y:NmSet).
              x Nm[X1] x Nat x Ref[Y1](list[X2][Y2])
             )
            )
    );

    /// Allocate a Ref cell, holding a Cons cell, pointing at a list:
    ///
    /// ref         cons       ref     list
    /// |{@!}X1|--->|X1|_|*|-->|Y1|--> |...[X2][Y2]...|
    ///
    /// : Ref[{@!}X1](List[X1%X2][Y1%Y2])
    ///
    fn ref_cons:(
        Thk[0]
            foralli (X,X1,X2):NmSet | ((X1%X2)=X:NmSet).
            foralli (Y,Y1,Y2):NmSet | ((Y1%Y2)=Y:NmSet).
            0 Nm[X1] ->
            0 Nat ->
            0 Ref[Y1](List[X2][Y2]) ->
        {{@!}X1;0} F Ref[{@!}X1](List[X1%X2][Y1%Y2])
    ) = {            
        #n.#h.#t. ref n
        roll inj2 pack (X1,X2,Y1,Y2) (n, h, t)
    }
    
    /// Create a cons cell with a new ref cell tail, holding the given list.
    fn cons_ref:(
        Thk[0]
            foralli (X,X1,X2):NmSet | ((X1%X2)=X:NmSet).
            foralli (Y2):NmSet.
            0 Nm[X1] ->
            0 Nat ->
            0 List[X2][Y2] ->
        {{@!}X1;0} F (List[X1%X2][({@!}X1)%Y2])
    ) = {            
        #n.#h.#t.
        let rt = {ref n t}
        ret roll inj2 pack (X1,X2,({@!}X1),Y2) (n, h, rt)
    }

    /// Compute the length of the list; does not use memoization.
    fn len:(
        Thk[0]
            foralli (X,Y):NmSet.
            0 List[X][Y] -> {0;Y} F Nat
    ) = {
        #l. unroll match l {
            _u => {ret 0}
            c => {
                unpack (X1,X2,Y1,Y2) c = c
                let (x,y,ys) = {ret c}
                let lenys = {{force len}[X2][Y2] {!ys}}
                lenys + 1
            }
        }
    }

    /// Map a list of natural numbers, using a given function from
    /// naturals to naturals.
    fn map:(
        Thk[0]
            foralli (X,Y):NmSet.
            0 (Thk[0] 0 Nat -> 0 F Nat) ->
            0 List[X][Y] ->
        {{@!}X; Y%({@!}X)} F List[X][{@!}X]
    ) = {
        #f.#l. unroll match l {
            _u => { ret roll inj1 () }
            c => {
                unpack (X1,X2,Y1,Y2) c = c
                let (n, h, t) = { ret c }
                let h2 = {{force f} h}
                let (rt2,_t2) = {
                    memo(n){{force map} [X2][Y2] f {!t}}
                }
                ret roll inj2
                    pack (X1,X2,{@!}X1,{@!}X2) (n, h2, rt2)
            }
        }
    }

    /// Filter a list of naturals by a given predicate
    fn filter:(
        Thk[0]
            foralli (X,Y):NmSet.
            0 (Thk[0] 0 Nat -> 0 F Bool) ->
            0 List[X][Y] ->
        {{@!}X; Y%({@!}X)} F List[X][{@!}X]
    ) = {
        #f.#l. unroll match l {
            _u => { ret roll inj1 () }
            c => {
                unpack (X1,X2,Y1,Y2) c = c
                let (n, h, t) = { ret c }
                let (rt2, t2) = {
                    memo(n){{force filter} [X2][Y2] f {!t}}
                }
                if {{force f} h} {
                    ret roll inj2
                        pack (X1,X2,{@!}X1,{@!}X2) (n, h, rt2)
                }
                else {
                    ret t2
                }
            }
        }
    }

    /// Map a list of naturals by a given partial mapping function   
    fn map_filter:(
        Thk[0]
            foralli (X,Y):NmSet.
            0 (Thk[0] 0 Nat -> 0 F OpNat) ->
            0 List[X][Y] ->
        {{@!}X; Y%({@!}X)} F List[X][{@!}X]
    ) = {
        #f.#l. unroll match l {
            _u => { ret roll inj1 () }
            c => {
                unpack (X1,X2,Y1,Y2) c = c
                let (n, h, t) = { ret c }
                let (rt2, t2) = {
                    memo(n){{force map_filter} [X2][Y2] f {!t}}
                }
                let oh2 = {{force f} h}
                match oh2 {
                    _u => { ret t2 }
                    h2 => {
                        ret roll inj2
                            pack (X1,X2,{@!}X1,{@!}X2) (n, h, rt2)
                    }
                }
            }
        }
    }
}

/* Run as (shortened version):
   cargo test examples::list_nat::static 2>&1 | less -R
*/
pub mod static_tests {
    #[test]
    pub fn typing() { fgi_listing_test!{
        open crate::examples::list_nat;
        ret 0
    }}
}

pub mod dynamic_tests {
    /* 
     * Try the following at the command line:
     * 
     *  $ export FUNGI_VERBOSE_REDUCE=1
     *
     *  $ cargo test examples::list_nat::dynamic_tests::short -- --nocapture | less -R
     *
     */
    #[test]
    pub fn short() { fgi_dynamic_trace!{
        [Expect::SuccessXXX]
        open crate::examples::list_nat;
        open crate::examples::list_nat_edit;
        open crate::examples::list_nat_reverse;
        
        /// Generate input
        let list1  = {ws (@@gen) {{force gen} 10}}

        /// Allocate nil ref cell
        let refnil = {ref (@@nil) roll inj1 ()}

        /// Allocate archivist thunk: when forced, it reverses the input list
        let t = {ws (@@archivst) thk (@@comp) {
            let list2 = {{force reverse} {!list1} refnil (@@revres)}
            ret (list1, list2)
        }}

        /// Initial run
        let outs_1 = {force t}

        /// First change: Insert name 666, element 666 after name 5
        let b1 = {
            {force insert_after}[?] (@5) (@666) 666 {!list1}
        }

        /// Re-force archivist; Precipitates change propagation
        let outs_2 = {force t}

        /// Second change: Remove inserted name 666, and element 666
        let b2 = {
            {force remove_after}[?] (@5) {!list1}
        }

        /// Re-force archivist; Precipitates change propagation
        let outs_3 = {force t}
        ret (b1, b2)
    }}
    
    #[test]    
    pub fn long() { fgi_dynamic_trace!{
        [Expect::SuccessXXX]
        open crate::examples::list_nat;
        open crate::examples::list_nat_edit;
        open crate::examples::list_nat_reverse;
        open crate::examples::nat;

        /// Generate input
        let list1  = {{force gen} 10}

        /// Generate nil
        let refnil = {ref (@@nil) roll inj1 ()}        

        /// Allocate archivist thunk: Compute two mappings, reverse, filter and map_filter over the input list
        let t = { thk (@@archivist) {
            let list2 = {ws (@@map1)   {memo (@@map1)   {{force map} (thunk #x. x + 1) {!list1}}}}
            let list3 = {ws (@@map2)   {memo (@@map2)   {{force map} (thunk #x. x + 2) {!list1}}}}
            let list4 = {ws (@@rev)    {memo (@@rev)    {{{force reverse} {!list1} refnil (@@revres)}}}}
            let list5 = {ws (@@filter) {memo (@@filter) {{force filter} nat_is_odd {!list1}}}}
            let list6 = {ws (@@mapftr) {memo (@@mapftr) {{force map_filter} nat_succ_even {!list1}}}}
            ret (list1, list2, list3, list4, list5, list6)
        }}

        /// Initial run of archivist thunk
        let outs_1 = {force t}

        /// First change: insert name 666, element 666 after name 5
        let b1 = {
            {force insert_after}[?] (@5) (@666) 666 {!list1}
        }

        /// Change propagation
        let outs_2 = {force t}

        /// Second change: remove name @666 and element 666 (after name @5)
        let b2 = {
            {force remove_after}[?] (@5) {!list1}
        }

        /// Change propagation
        let outs_3 = {force t}
        ret (b1, b2)
    }}
}

///////////////////////////////////////////////////////////////////////
#[test]
pub fn listing1 () { fgi_listing_expect![
    [Expect::FailureXXX]
        
        decls {
            /// Lists of natural numbers
            type List  = (
                rec list. foralli (X,Y):NmSet.
                    (+ Unit +
                     (exists (X1,X2):NmSet | ((X1%X2)=X:NmSet).
                      exists (Y1,Y2):NmSet | ((Y1%Y2)=Y:NmSet).
                      x Nm[X1] x Nat x Ref[Y1](list[X2][Y2])
                     ))
            );
        }
    
    let cons:(
        Thk[0]
            foralli (X1,X2):NmSet. // <-- forgot to insist that X1%X2
            foralli (Y1,Y2):NmSet. // <-- forgot to insist that Y1%Y2
            0 Nm[X1] ->
            0 Nat ->
            0 Ref[Y1](List[X2][Y2]) ->
            0 F List[X1%X2][Y1%Y2]
    ) = {            
        ret thunk #n.#h.#t. ret roll inj2
            pack (X1,X2,Y1,Y2) (n, h, t)
    }
    
    ret 0
]}
