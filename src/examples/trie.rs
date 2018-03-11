#[test]
pub fn listing () { fgi_listing_test![
    decls {
        /// Optional natural numbers:
        type OpNat = (+ Unit + Nat );
        
        /// Levels (as numbers), for level trees.
        type Lev = ( Nat )
            
        /// Sequences (balanced binary level trees), whose leaves
        /// are optional natural numbers:
        type Seq = (
            rec seq. foralli (X,Y):NmSet.
                (+ (+ Unit + Nat)
                 + (exists (X1,X2,X3)   :NmSet | ((X1%X2%X3)=X:NmSet).
                    exists (Y1,Y2,Y3,Y4):NmSet | ((Y1%Y2%Y3%Y4)=Y:NmSet).
                    x Nm[X1] x Lev
                    x Ref[Y1](seq[X2][Y2])
                    x Ref[Y3](seq[X3][Y4]))
                )
        );                
            
        /// Sets (balanced binary hash tries), whose leaves are
        /// optional natural numbers:
        type Set = (
            rec set. foralli (X,Y):NmSet.
                (+ (+ Unit + Nat)
                 + (exists (X1,X2,X3)   :NmSet | ((X1%X2%X3)=X:NmSet).
                    exists (Y1,Y2,Y3,Y4):NmSet | ((Y1%Y2%Y3%Y4)=Y:NmSet).
                    x Nm[X1]
                    x Ref[Y1](set[X2][Y2])
                    x Ref[Y3](set[X3][Y4]))
                )
        );                
        
        idxtm Bin     = (#x:Nm.({x,@1})%({x,@2}));
        idxtm WS_Bin  = (#x:NmSet.{@!}(       (Bin)   x));
        idxtm WS_Bin1 = (#x:NmSet.{@!}((Bin) ((Bin)^* x)));
        idxtm WS_Join = (#x:NmSet.{@!}(       (Bin)^* x ));
        idxtm WS_Trie = (#x:NmSet.{@!}(  x * ((Bin)^* x )));
    }
    
    let join:(
        Thk[0] foralli (X1, X2, Y1, Y2):NmSet.
            0 Set[X1][Y1] ->
            0 Set[X2][Y2] ->
        { {WS_Join} (X1%X2); Y1%Y2 }
        F Set[(WS_Join)(X1 % X2)][{WS_Join}(X1%X2)]
    ) = {
        unimplemented
    }

    let trie:(
        Thk[0] foralli (X,Y):NmSet.
            0 Seq[X][Y] ->
        { ({WS_Trie} X) % {[@@][@666 * @1]}; Y }
        F Set[X][{WS_Trie} X]
    ) = {
        ret thunk fix trie. #seq.
        let foo = {
            ws (nmfn [#x:Nm. @666 * x]) {
                let (a,b) = { memo(name @1){ ret 0 } }
                ret 0
            }
        }
        unimplemented
    }

    let test_write_scope:(
        Thk[0] 
        { {@!}( ({@666} % {@777}) * ({@1} % {@2}) ); 0 }
        F Nat
    ) = {
        ret thunk
        let foo = {
            ws (nmfn [#x:Nm. @666 * x]) {
                let (a1,b1) = { memo(name @1){ ret 111 } }
                let (a2,b2) = { memo(name @2){ ret 222 } }
                ret 0
            }
        }
        let bar = {
            ws (nmfn [#x:Nm. @777 * x]) {
                let (a1,b1) = { memo(name @1){ ret 111 } }
                let (a2,b2) = { memo(name @2){ ret 222 } }
                ret 0
            }
        }
        ret 0
    }

    ret 0
]}
