# SeaDAWG Rust

Open high performance implementation of an Online DAWG and Online CDAWG for string indexing. 

Support for Prefix, Contains, Suffix and Exact match queries.
 
**WARNING**: I encourage looking at Finite State Transducers for larger corpus of data.

# Remark about Rust

Rust is stupidly weird about mutable and immutable. If I have a MUT lock on an object, then I must obviously have exclusive access to the object's internal data. WTF is this annoying error around not being able to take a non exclusive READ (immutable) lock where I already have an exclusive WRITE (mutable) lock. I resort to unsafe in order to grab mut inner data.