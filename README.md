# orx-fixed-vec
`FixedVec` implements `PinnedVec`: it preserves the memory locations of its elements and can be used to create an ImpVec allowing to push to the vector with an immutable reference. `FixedVec` allows operations with same complexity and speed of `std::vec::Vec` with the drawback that it cannot exceed its preset capacity.
