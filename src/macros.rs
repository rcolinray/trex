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

#[macro_export]
macro_rules! components {
    ( $( $T:ident ), + ) => {
        family!( $( $T ), +);
    }
}

#[macro_export]
macro_rules! events {
    ( $( $T:ident ),+ ) => {
        family!(@1, $( $T, )+ );
    }
}
