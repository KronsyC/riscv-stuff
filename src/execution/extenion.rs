#[derive(Debug, enumset::EnumSetType)]
#[repr(u8)]
pub enum Extension{
    Atomic,
    Compressed,
    DoublePrecisionFP,
    RV32E,
    SinglePrecisionFP,
    Hypervisor,
    Integer,
    IntegerMultiplyDivide,
    QuadPrecisionFP,
    SupervisorMode,
    UserMode,
    Vector,
    NonStandard
}


impl Extension{
    pub fn character(&self) -> char{
        match self{
            Self::Atomic                => 'A',
            Self::Compressed            => 'C',
            Self::DoublePrecisionFP     => 'D',
            Self::RV32E                 => 'E',
            Self::SinglePrecisionFP     => 'F',
            Self::Hypervisor            => 'H',
            Self::Integer               => 'I',
            Self::IntegerMultiplyDivide => 'M',
            Self::QuadPrecisionFP       => 'Q',
            Self::SupervisorMode        => 'S',
            Self::UserMode              => 'U',
            Self::Vector                => 'V',
            Self::NonStandard           => 'X'
        }
    }

    pub fn from_char(c : char) -> Option<Self>{
        use Extension::*;
        match c{
            'A' => Some(Atomic),
            'C' => Some(Compressed),
            'D' => Some(DoublePrecisionFP),
            'E' => Some(RV32E),
            'F' => Some(SinglePrecisionFP),
            'H' => Some(Hypervisor),
            'I' => Some(Integer),
            'M' => Some(IntegerMultiplyDivide),
            'Q' => Some(QuadPrecisionFP),
            'S' => Some(SupervisorMode),
            'U' => Some(UserMode),
            'V' => Some(Vector),
            'X' => Some(NonStandard),
            _ => None
        }
    }
}

