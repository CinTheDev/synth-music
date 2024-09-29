#[derive(Clone, Copy)]
pub enum MusicKeyBase {
    C,
    Csharp,
    D,
    Dflat,
    E,
    Eflat,
    F,
    Fsharp,
    G,
    Gflat,
    A,
    Aflat,
    B,
    Bflat,
}
#[derive(Clone, Copy)]
pub enum MusicKeyType {
    Major,
    Minor,
}
#[derive(Clone, Copy)]
pub struct MusicKey {
    pub base: MusicKeyBase,
    pub key_type: MusicKeyType,
}
