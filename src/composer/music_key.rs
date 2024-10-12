/// The tonic of a music key
#[derive(Clone, Copy)]
pub enum KeyTonic {
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

    Dsharp,
    D,
    Dflat,

    Csharp,
    C,
}

/// The type of a music key
/// 
/// In music theory, this is usually inferred from the melody. In this crate
/// it does affect the notes, so it must be specified.
#[derive(Clone, Copy)]
pub enum KeyType {
    Major,
    Minor,
}

/// Represents a key in music theory.
/// 
/// It is used to convert `ScaledValue` into `ConcreteValue`. Currently this
/// only makes sense for the 12-TET note system, or partially for other similar
/// systems.
/// 
/// `MusicKey` also stores whether the key is Major or Minor.
#[derive(Clone, Copy)]
pub struct MusicKey {
    pub tonic: KeyTonic,
    pub key_type: KeyType,
}
