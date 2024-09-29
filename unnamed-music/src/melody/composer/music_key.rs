#[derive(Clone, Copy)]
pub enum KeyTonic {
    Dsharp,
    D,
    Dflat,

    Csharp,
    C,

    B,
    Bflat,

    Asharp,
    A,
    Aflat,

    Gsharp,
    G,
    Gflat,

    Fsharp,
    F,

    E,
    Eflat,
}
#[derive(Clone, Copy)]
pub enum KeyType {
    Major,
    Minor,
}
#[derive(Clone, Copy)]
pub struct MusicKey {
    pub tonic: KeyTonic,
    pub key_type: KeyType,
}
