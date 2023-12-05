/// A marker used to indicate which stack we're using for an operation.
///
/// This is because our bytecode has two stacks, one for the "in-circuit" computation,
/// and one for the "out-of-circuit" computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WhichStack {
    /// The stack inside the circuit.
    Inside,
    /// The stack outside the circuit.
    Outside,
}

/// An operation over field elements on a given stack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldOp {
    /// [..] -> [.., 0]
    Zero,
    /// [..] -> [.., 1]
    One,
    /// [.., a, b] -> [.., a + b]
    Add,
    /// [.., a] -> [.., -a]
    Neg,
    /// [.., a, b] -> [.., a * b]
    Mul,
    /// [.., a] -> [.., a⁻¹]
    Inv,
}

/// An operation manipulating a single stack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackOp {
    /// Apply some field operation to the stack.
    FieldOp(FieldOp),
    /// Copy the nth (starting from 0, and going down the stack) value onto the stack.
    Copy(u32),
    /// Drop n values from the stack.
    Drop(u32),
}

/// An operation in our bytecode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    /// Manipulate one of the stacks.
    StackOp(WhichStack, StackOp),
    /// Move the top n values from one stack to the other.
    Move(u32, WhichStack, WhichStack),
}
