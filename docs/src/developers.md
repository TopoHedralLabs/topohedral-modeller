# Developer Guide

## Style Guide

### Return Values

When the return type is of known size at compile time, for example a [Vector<D>], then return it. 
Examples: [`crate::geometry::common::Curve::eval`]
