//
// CallKind
//

/// Call kind.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CallKind {
    /// Normal.
    ///
    /// All of the call's arguments will be evaluated before its dispatch.
    #[default]
    Normal,

    /// Eager.
    ///
    /// The call's arguments will *not* be evaluated before its dispatch. They will be sent as is.
    ///
    /// This gives the function control and full responsibility over if and in what order to
    /// evaluate any of its arguments.
    Eager,

    /// Lazy.
    ///
    /// The call should not be dispatched during an evaluation. It should remain as is.
    ///
    /// This allows callers to pass functions "by value" so that they can be dispatched if and when
    /// necessary.
    ///
    /// Setting a call to lazy merely specifies desired behavior. Functions may choose to ignore
    /// it.
    Lazy,
}
