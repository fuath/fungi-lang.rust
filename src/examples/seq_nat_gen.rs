fgi_mod!{
    open crate::examples::seq_nat;
    open crate::examples::name;
    open crate::examples::nat;

    /// Generate a sequence of natural numbers
    //
    // XXX -- This type is wrong.  TODO -- figure out how to
    // ecode this type correctly, with existentials.
    fn seq_gen:(
        Thk[0]
            foralli (Y1,X1,Y2):NmSet.
            0 Nat -> 0 F Ref[Y1](Seq[X1][Y2])
    ) = {
        #n. if {{force nat_is_zero} n} {
            ref (@0) roll inj1 ()
        } else {
            let nm = {{force name_of_nat} n}
            let pred = {{force nat_sub} n 1}
            let seq_ref = {{force seq_gen} pred}
            let leaf_ref = {ref nm roll inj2 inj1 (nm, n)}
            let nmb = {nm,(@@bin)}
            ref nmb
                roll inj2 inj2 pack (?,?,?)
                (nmb, n, leaf_ref, seq_ref)
        }
    }
}

pub mod static_tests {
    #[test]
    pub fn typing() { fgi_listing_test!{
        open crate::examples::seq_nat_gen;
        ret 0
    }}
}
