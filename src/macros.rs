#[doc(hidden)]
#[macro_export]
macro_rules! family {
    ( @$family:expr, ) => {};

    ( @$family:expr, $head:ident, $( $tail:ident, )* ) => {
        impl $crate::FamilyMember for $head {
            fn family() -> $crate::Family {
                $family
            }
        }

        family!(@$family + 1, $( $tail, )*);
    };

    ( $( $T:ident ),+ ) => {
        family!(@0, $( $T, )+ );
    };
}

/// Defines the component family.
#[macro_export]
macro_rules! components {
    ( $( $T:ident ), + ) => {
        family!( $( $T ), +);
    }
}

/// Defines the event family.
#[macro_export]
macro_rules! events {
    ( $( $T:ident ),+ ) => {
        // Families start at 1 since the Halt event is automatically registered
        // at 0.
        family!(@1, $( $T, )+ );
    }
}
