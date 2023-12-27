#[repr(u8)]
pub enum RiscVRegister{
    /// Hard-wired Zero
    X0,

    /// Return Address
    X1,
    
    /// Stack Pointer
    X2,

    /// Global Pointer
    X3,

    /// Thread Pointer 
    X4,

    /// Temp 0
    X5,

    /// Temp 1
    X6,

    /// Temp 2
    X7,

    /// Saved 0 / Frame Pointer
    X8,

    /// Saved 1
    X9,

    /// Function Argument 0 / Return Value 0
    X10,

    /// Function Argument 1 / Return Value 1 
    X11,

    /// Function Argument 2 
    X12,

    /// Function Argument 3 
    X13,

    /// Function Argument 4 
    X14,

    /// Function Argument 5 
    X15,

    /// Function Argument 6 
    X16,

    /// Function Argument 7
    X17,

    /// Saved 2 
    X18,

    /// Saved 3 
    X19,

    /// Saved 4
    X20,

    /// Saved 5
    X21,

    /// Saved 6
    X22,

    /// Saved 7
    X23,

    /// Saved 8
    X24,

    /// Saved 9
    X25,

    /// Saved 10
    X26,

    /// Saved 11
    X27,

    /// Temp 3
    X28,

    /// Temp 4
    X29,

    /// Temp 5
    X30,

    /// Temp 6
    X31
}

pub enum TempRegister{
    Temp0,
    Temp1,
    Temp2,
    Temp3,
    Temp4,
    Temp5,
    Temp6
}

pub enum SavedRegister{
    Saved0,
    Saved1,
    Saved2,
    Saved3,
    Saved4,
    Saved5,
    Saved6,
    Saved7,
    Saved8,
    Saved9,
    Saved10,
    Saved11
}

pub enum ReturnValueRegister{
    ReturnValue0,
    ReturnValue1
}


pub enum FunctionArgumentRegister{
    FunctionArgument0,
    FunctionArgument1,
    FunctionArgument2,
    FunctionArgument3,
    FunctionArgument4,
    FunctionArgument5,
    FunctionArgument6,
    FunctionArgument7,
}



impl From<TempRegister> for RiscVRegister{
    fn from(temp : TempRegister) -> RiscVRegister {
        match temp{
            TempRegister::Temp0 => RiscVRegister::X5,
            TempRegister::Temp1 => RiscVRegister::X6,
            TempRegister::Temp2 => RiscVRegister::X7,
            TempRegister::Temp3 => RiscVRegister::X28,
            TempRegister::Temp4 => RiscVRegister::X29,
            TempRegister::Temp5 => RiscVRegister::X30,
            TempRegister::Temp6 => RiscVRegister::X31,
        }
    }
}

impl From<SavedRegister> for RiscVRegister{
    fn from(value: SavedRegister) -> Self {
        match value{
            SavedRegister::Saved0 => RiscVRegister::X8,
            SavedRegister::Saved1 => RiscVRegister::X9,
            SavedRegister::Saved2 => RiscVRegister::X18,
            SavedRegister::Saved3 => RiscVRegister::X19,
            SavedRegister::Saved4 => RiscVRegister::X20,
            SavedRegister::Saved5 => RiscVRegister::X21,
            SavedRegister::Saved6 => RiscVRegister::X22,
            SavedRegister::Saved7 => RiscVRegister::X23,
            SavedRegister::Saved8 => RiscVRegister::X24,
            SavedRegister::Saved9 => RiscVRegister::X25,
            SavedRegister::Saved10 => RiscVRegister::X26,
            SavedRegister::Saved11 => RiscVRegister::X27,
        }
    }
}

impl From<ReturnValueRegister> for RiscVRegister{
    fn from(value: ReturnValueRegister) -> Self {
        match value{
            ReturnValueRegister::ReturnValue0 => RiscVRegister::X10,
            ReturnValueRegister::ReturnValue1 => RiscVRegister::X11,
        }
    }
}

impl From<FunctionArgumentRegister> for RiscVRegister{
    fn from(value: FunctionArgumentRegister) -> Self {
        match value{
            FunctionArgumentRegister::FunctionArgument0 => RiscVRegister::X10,
            FunctionArgumentRegister::FunctionArgument1 => RiscVRegister::X11,
            FunctionArgumentRegister::FunctionArgument2 => RiscVRegister::X12,
            FunctionArgumentRegister::FunctionArgument3 => RiscVRegister::X13,
            FunctionArgumentRegister::FunctionArgument4 => RiscVRegister::X14,
            FunctionArgumentRegister::FunctionArgument5 => RiscVRegister::X15,
            FunctionArgumentRegister::FunctionArgument6 => RiscVRegister::X16,
            FunctionArgumentRegister::FunctionArgument7 => RiscVRegister::X17,
        }
    }
}

pub enum RegisterSaver{
    NotApplicable,
    Caller,
    Callee
}



impl RiscVRegister{
    pub fn saved_by(&self) -> RegisterSaver{
        use RiscVRegister::*;
        use RegisterSaver::*;
        match self{
            X0 => NotApplicable,
            X1 => Caller,
            X2 => Callee,
            X3 => NotApplicable,
            X4 => NotApplicable,
            X5 => Caller,
            X6 => Caller,
            X7 => Caller,
            X8 => Callee,
            X9 => Callee,
            X10 => Caller,
            X11 => Caller,
            X12 => Caller,
            X13 => Caller,
            X14 => Caller,
            X15 => Caller,
            X16 => Caller,
            X17 => Caller,
            X18 => Callee,
            X19 => Callee,
            X20 => Callee,
            X21 => Callee,
            X22 => Callee,
            X23 => Callee,
            X24 => Callee,
            X25 => Callee,
            X26 => Callee,
            X27 => Callee,
            X28 => Caller,
            X29 => Caller,
            X30 => Caller,
            X31 => Caller,
        }
    }
}
