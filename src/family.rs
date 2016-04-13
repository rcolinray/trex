pub type Family = usize;

pub trait FamilyMember {
    fn family() -> Family;
}

pub trait FamilyStore {
    fn family(&self) -> Family;
}
