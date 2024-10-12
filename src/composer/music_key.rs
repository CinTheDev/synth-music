// Major keys
pub const C_MAJOR: MusicKey = MusicKey {
    tonic: KeyTonic::C,
    key_type: KeyType::Major,
};
pub const D_MAJOR: MusicKey = MusicKey {
    tonic: KeyTonic::D,
    key_type: KeyType::Major,
};
pub const E_MAJOR: MusicKey = MusicKey {
    tonic: KeyTonic::E,
    key_type: KeyType::Major,
};
pub const F_MAJOR: MusicKey = MusicKey {
    tonic: KeyTonic::F,
    key_type: KeyType::Major,
};
pub const G_MAJOR: MusicKey = MusicKey {
    tonic: KeyTonic::G,
    key_type: KeyType::Major,
};
pub const A_MAJOR: MusicKey = MusicKey {
    tonic: KeyTonic::A,
    key_type: KeyType::Major,
};
pub const B_MAJOR: MusicKey = MusicKey {
    tonic: KeyTonic::B,
    key_type: KeyType::Major,
};

// Minor keys
pub const C_MINOR: MusicKey = MusicKey {
    tonic: KeyTonic::C,
    key_type: KeyType::Minor,
};
pub const D_MINOR: MusicKey = MusicKey {
    tonic: KeyTonic::D,
    key_type: KeyType::Minor,
};
pub const E_MINOR: MusicKey = MusicKey {
    tonic: KeyTonic::E,
    key_type: KeyType::Minor,
};
pub const F_MINOR: MusicKey = MusicKey {
    tonic: KeyTonic::F,
    key_type: KeyType::Minor,
};
pub const G_MINOR: MusicKey = MusicKey {
    tonic: KeyTonic::G,
    key_type: KeyType::Minor,
};
pub const A_MINOR: MusicKey = MusicKey {
    tonic: KeyTonic::A,
    key_type: KeyType::Minor,
};
pub const B_MINOR: MusicKey = MusicKey {
    tonic: KeyTonic::B,
    key_type: KeyType::Minor,
};

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
