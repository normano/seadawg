# SeaDAWG Js

Implementation of SeaDAWG in Typescript. Probably the first working open sourced implementation of online construction of CDAWGs.

Under development, so anything can change at any point.

# Tools

**Bad State Runner** - runs words that put the Graph into a bad state while I was developing. Mostly to ensure that those bad states do not happen again.

**Thrasher** - runs series of random 128 character words to make sure the graph is constructed properly. If bad state that causes fatal error is encountered, then words that were added will be dumped into a file, so it can be reproduced.

# Versions

You will see different versions in the `src/` directory.

**Original version** is the one implemented in the paper with an error. Should not use this.

**Version 1** is derived from the original version with support for same terminator. The observation is that the terminator does not need to exist as a suffix, but only as a terminal node. Terminal node and entry will never move or be detected by the update algorithm.
- With 300,000 128 Character Random Alphanumeric Characters, the memory it requires is ~3.26GB, with build duration of 1m 26s

**Version 2** is derived from version 1 without any terminator therefore a little lower memory requirement. Observation is that the sink node can store an entry to its original node while each node could store pointers to other sinks of which it is a suffix.
- With 300,000 128 Character Random Alphanumeric Characters, the memory it requires is ~2.11GB, with build duration of 1m 6s
