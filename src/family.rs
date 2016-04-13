/// A unique identifier for a type that is a member of group of types.
pub type Family = usize;

/// Used to identify types that are members of a group of types.
pub trait FamilyMember {
    fn family() -> Family;
}

pub trait FamilyStore {
    fn family(&self) -> Family;
}
