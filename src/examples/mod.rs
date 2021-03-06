/*!

Examples of data structures and algorithms in Fungi.

### Basics

These (very small) examples demonstrate basic concepts from Fungi's type and effect system:

- [`basic_read_effects`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/basic_read_effects.rs.html)
--- _read effects_ track the reference cells and thunks that a Fungi program observes and forces.
- [`basic_write_effects`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/basic_write_effects.rs.html)
--- _write effects_ track the reference cells and thunks that a Fungi program allocates.
- [`basic_write_scope`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/basic_write_scope.rs.html)
--- _write scopes_ distinctly qualify written names for different dynamic calling contexts.
- [`basic_subtyping`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/basic_subtyping.rs.html)
--- _subtyping_ permits structures with _fewer_ names to be used in contexts that expect _more_ names.
- [`basic_existentials`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/basic_existentials.rs.html)
--- _existential types_ permit packing names and named structures into types that approximate them.

### FP Basics in Fungi

Basic patterns from functional programming (FP), in the _"pure"_ fragment of Fungi.  The pure effect (written `0`, for short) means that a computation lacks read and write effects.

- [`op_nat`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/op_nat.rs.html) --- Simple primitives for optional natural numbers
- [`pure_list_nat`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/pure_list_nat.rs.html) --- Simple primitives for lists of natural numbers

### Lists

Linked lists whose cons cells contain names, and whose tail pointers are (named) reference cells.

- [`list_nat`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/list_nat.rs.html) --- Primitives for lists of natural numbers
- [`list_nat_dedup`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/list_nat_dedup.rs.html) --- Deduplicate input list elements; uses a hash trie

### Tries

Hash tries that represent functional sets, with named elements.

- [`trie_nat`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/trie_nat.rs.html) --- Primitives for tries of natural numbers

### Sequences

Sequences of natural numbers, represented as probabilistically-balanced binary trees (level trees), with names and reference cells:
- [`seq_max`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/seq_max.rs.html)
--- finds the maximum element in a sequence.
 - [`seq_filter`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/seq_filter.rs.html)
--- filters a sequence of elements, producing a new (smaller) sequence.

*/

// ### Sets

// _In progress_

// Sets of natural numbers, represented as probabilistically-balanced binary hash tries:
// - [`trie_join`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/trie.rs.html)
// --- joins two sets (as tries) into a single set (as a trie).
//  - [`trie_of_seq`](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/trie.rs.html)
// --- builds a set of elements (as a hash trie) from a sequence of elements (as a level tree).

// ### Quickhull

// Computes the convex hull, in sorted order, of an unordered sequence of points in 2D space.

// **TODO**


/// Optional natural numbers
///
/// [Fungi listing](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/op_nat.rs.html)
pub mod op_nat;

/// Primitive utilities:
pub mod nat;
pub mod name;

/// Lists of natural numbers, without names, and with pure operations.
///
/// [Fungi listing](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/pure_list_nat.rs.html)
pub mod pure_list_nat;

pub mod ref_edit;

pub mod list_nat;
pub mod list_nat_edit;
pub mod list_nat_convert;
pub mod list_nat_reverse;

// Wait unti dedup is finished, then seed trie_nat module with that implementation.
pub mod trie_nat;

// Kleene closure problem here; also, divergence.
pub mod list_nat_dedup;

pub mod seq_nat;
pub mod seq_nat_gen;
pub mod seq_nat_dfs;
//pub mod seq_nat_bfs;

//pub mod seq_nat_dfs_lazy;
//pub mod stream_nat;

/// Find the maximum element in a sequence
///
/// [Fungi listing](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/seq_max.rs.html)
pub mod seq_max;

/// Filter a sequence of elements, producing a new (smaller) sequence
///
/// [Fungi listing](https://docs.rs/fungi-lang/0/src/fungi_lang/examples/seq_filter.rs.html)
pub mod seq_filter;

// --- In progress:
//pub mod set_join;
//pub mod trie;

//pub mod pure_stream_nat;
//pub mod pure_lazylist;
//pub mod pure_rtq;
//pub mod fifo;


// --- Regression tests
pub mod basic_hostobj;
pub mod basic_read_effects;
pub mod basic_write_effects;
pub mod basic_write_scope;
pub mod basic_subtyping;
pub mod basic_existentials;
