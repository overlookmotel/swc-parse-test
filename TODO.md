# TODO

* Try to create `vec<u8>` from struct pointer
* Modify vector content and see if value in struct changes (i.e. is vec using same memory as struct?)
* Wrap vector in Buffer and figure out if JS can write to the buffer and change vector in Rust (i.e. is JS Buffer using same memory as Rust vector?)
* Wrap vector in Buffer, then modify vector content and see if Buffer contains changed data (i.e. is JS Buffer using same memory as Rust vector?)
* Can you put all of Rust's heap in a Buffer to pass to JS?
