#[derive(Debug)]
#[repr(u8)]
pub enum ExecutionMode{
    UserApplication,
    Supervisor,
    Machine,

    ////////
    // Hypervisor Extension
    ////////

    VirtualizedSupervisor,
    VirtualizedUserApplication
}



impl ExecutionMode{
    pub fn as_str(&self) -> &'static str{
        match self{
            Self::UserApplication            => "User/Application",
            Self::Supervisor                 => "Supervisor",
            Self::Machine                    => "Machine",
            Self::VirtualizedSupervisor      => "Virtualized Supervisor",
            Self::VirtualizedUserApplication => "Virtualized User/Application"
        }
    }


    pub fn abbrev(&self) -> &'static str{
        match self{
            Self::UserApplication            => "U",
            Self::Supervisor                 => "S",
            Self::Machine                    => "M",
            Self::VirtualizedSupervisor      => "HS",
            Self::VirtualizedUserApplication => "HU"
        }
    }
}
