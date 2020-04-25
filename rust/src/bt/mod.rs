/// Version BT is sa compromise between what I wanted and a pure CDAWG by the paper.
///
/// The idea was to "unthaw" a CDAWG as it is built.
///
/// Next idea is to stick closer to the original algorithm, but try to split the sink nodes in the same way that as DAWG is built where a sink can be a primary
/// or secondary. A sink is considered primary if it contains one word and that word is equal to the suffix. It is considered
/// secondary if the word it represents is less than or greater than the suffix that is going to be added.
///
/// cocoa$ has a length of 6 while coa$ has a length of 4. Sinks representing [c[o[a$]]] would contain cocoa$.
/// If you added cocoa$ and you followed the word [[co]a$] to the sink then you can see the sink is a secondary since
/// it is a currently a sink that does not represent what was traversed. If you added coa$, then the sink pointed to in that traversal
/// would be replaced with a sink that contains coa$ and the original cocoa$ and considered to be the primary for coa$,
/// so no more sink reassignments can occur.
///
/// I think this would allow for the CDAWG to be correctly built while maintaining all suffixes for prefix, suffix and
/// contains queries. I think it is more than likely that only one terminator can be used to build a CDAWG to prevent
/// word terminator exhaustion.

pub mod core;
pub mod traversal;