#[derive(Debug)]
#[repr(u16)]
pub enum CSRRegister{

    /////////////////////////////////////
    // Unprivileged Floating-Point CSRs
    /////////////////////////////////////

    FFlags = 0x001,
    FFRM   = 0x002,
    FCSR   = 0x003,
    
    /////////////////////////////////////
    // Unprivileged Counter / Timers
    /////////////////////////////////////
    Cycle   = 0xC00,
    Time    = 0xC01,
    InstRet = 0xC02,

    HPMCounter3 = 0xC03,
    HPMCounter4 = 0xC04,
    HPMCounter5 = 0xC05,
    HPMCounter6 = 0xC06,
    HPMCounter7 = 0xC07,
    HPMCounter8 = 0xC08,
    HPMCounter9 = 0xC09,
    HPMCounter10 = 0xC0A,
    HPMCounter11 = 0xC0B,
    HPMCounter12 = 0xC0C,
    HPMCounter13 = 0xC0D,
    HPMCounter14 = 0xC0E,
    HPMCounter15 = 0xC0F,
    HPMCounter16 = 0xC10,
    HPMCounter17 = 0xC11,
    HPMCounter18 = 0xC12,
    HPMCounter19 = 0xC13,
    HPMCounter20 = 0xC14,
    HPMCounter21 = 0xC15,
    HPMCounter22 = 0xC16,
    HPMCounter23 = 0xC17,
    HPMCounter24 = 0xC18,
    HPMCounter25 = 0xC19,
    HPMCounter26 = 0xC1A,
    HPMCounter27 = 0xC1B,
    HPMCounter28 = 0xC1C,
    HPMCounter29 = 0xC1D,
    HPMCounter30 = 0xC1E,
    HPMCounter31 = 0xC1F,

    CycleH = 0xC80,
    TimeH  = 0xC81,
    InstRetH = 0xC82,


    HPMCounter3H  = 0xC83,
    HPMCounter4H  = 0xC84,
    HPMCounter5H  = 0xC85,
    HPMCounter6H  = 0xC86,
    HPMCounter7H  = 0xC87,
    HPMCounter8H  = 0xC88,
    HPMCounter9H  = 0xC89,
    HPMCounter10H = 0xC8A,
    HPMCounter11H = 0xC8B,
    HPMCounter12H = 0xC8C,
    HPMCounter13H = 0xC8D,
    HPMCounter14H = 0xC8E,
    HPMCounter15H = 0xC8F,
    HPMCounter16H = 0xC90,
    HPMCounter17H = 0xC91,
    HPMCounter18H = 0xC92,
    HPMCounter19H = 0xC93,
    HPMCounter20H = 0xC94,
    HPMCounter21H = 0xC95,
    HPMCounter22H = 0xC96,
    HPMCounter23H = 0xC97,
    HPMCounter24H = 0xC98,
    HPMCounter25H = 0xC99,
    HPMCounter26H = 0xC9A,
    HPMCounter27H = 0xC9B,
    HPMCounter28H = 0xC9C,
    HPMCounter29H = 0xC9D,
    HPMCounter30H = 0xC9E,
    HPMCounter31H = 0xC9F,


    /////////////////////////////////////
    // Supervisor Trap Setup
    /////////////////////////////////////
    
    SStatus    = 0x100,
    SIE        = 0x104,
    STVec      = 0x105,
    SCounterEn = 0x106,

    /////////////////////////////////////
    // Supervisor Configuration
    /////////////////////////////////////

    SEnvCfg = 0x10A,

    /////////////////////////////////////
    // Supervisor Trap Handling
    /////////////////////////////////////
    
    SScratch = 0x140,
    SEPC     = 0x141,
    SCause   = 0x142,
    STVal    = 0x143,
    SIP      = 0x144,

    /////////////////////////////////////
    // Supervisor Protection and Translation
    /////////////////////////////////////

    SATP = 0x180,

    /////////////////////////////////////
    // Supervisor Debug / Trace Registers
    /////////////////////////////////////

    SContext = 0x5A8,


    ///////////////////////////////////
    // Hypervisor Trap Setup
    ///////////////////////////////////

    HStatus    = 0x600,
    HEDeleg    = 0x602,
    HIDeleg    = 0x603,
    HIE        = 0x604,
    HCounterEn = 0x606,
    HGEIE      = 0x607,


    ///////////////////////////////////
    // Hypervisor Configuration 
    ///////////////////////////////////

    HEnvCfg = 0x60A,
    HEnvCfgH = 0x61A,


    ///////////////////////////////////
    // Hypervisor Protection and Translation
    ///////////////////////////////////

    HGATP  = 0x680,


    ///////////////////////////////////
    // Hypervisor Debug / Trace Registers 
    ///////////////////////////////////

    HContext = 0x6A8,


    ///////////////////////////////////
    // Hypervisor Counter/Timer Virtualization Registers 
    ///////////////////////////////////

    HTimeDelta = 0x605,
    HTimeDeltaH = 0x615,


    ///////////////////////////////////
    // Virtual Supervisor Registers
    ///////////////////////////////////
    
    VSStatus = 0x200,
    VSIE     = 0x204,
    VSTVec   = 0x205,
    VSScratch = 0x240,
    VSEPC     = 0x241,
    VSCause = 0x242,
    VSTVal  = 0x243,
    VSIP    = 0x244,
    VSATP  = 0x280,


    ///////////////////////////////////
    // Machine Information Registers 
    ///////////////////////////////////

    MVendorID = 0xF11,
    MArchID   = 0xF12,
    MImpID    = 0xF13,
    MHarTID   = 0xF14,
    MConfigPtr = 0xF15,


    ///////////////////////////////////
    // Machine Trap Setup
    ///////////////////////////////////

    MStatus    = 0x300,
    MISA       = 0x301,
    MEDeleg    = 0x302,
    MIDeleg    = 0x303,
    MIE        = 0x304,
    MTVec      = 0x305,
    MCounterEn = 0x306,
    MStatusH   = 0x310,


    ///////////////////////////////////
    // Machine Trap Handling
    ///////////////////////////////////

    MScratch = 0x340,
    MEPC     = 0x341,
    MCause   = 0x342,
    MTVal    = 0x343,
    MIP      = 0x344,
    MTInst   = 0x34A,
    MTVal2   = 0x34B,


    ///////////////////////////////////
    // Machine Configuration
    ///////////////////////////////////

    MEnvCfg = 0x30A,
    MEnvCfgH = 0x31A,
    MSecCfg  = 0x747,
    MSecCfgH = 0x757,


    ///////////////////////////////////
    // Machine Memory Protection 
    ///////////////////////////////////

    PMPCfg0 = 0x3A0,
    PMPCfg1 = 0x3A1,
    PMPCfg2 = 0x3A2,
    PMPCfg3 = 0x3A3,
    PMPCfg4 = 0x3A4,
    PMPCfg5 = 0x3A5,
    PMPCfg6 = 0x3A6,
    PMPCfg7 = 0x3A7,
    PMPCfg8 = 0x3A8,
    PMPCfg9 = 0x3A9,
    PMPCfg10 = 0x3AA,
    PMPCfg11 = 0x3AB,
    PMPCfg12 = 0x3AC,
    PMPCfg13 = 0x3AD,
    PMPCfg14 = 0x3AE,
    PMPCfg15 = 0x3AF,

    PMPAddr0 = 0x3B0,
    PMPAddr1 = 0x3B1,
    PMPAddr2 = 0x3B2,
    PMPAddr3 = 0x3B3,
    PMPAddr4 = 0x3B4,
    PMPAddr5 = 0x3B5,
    PMPAddr6 = 0x3B6,
    PMPAddr7 = 0x3B7,
    PMPAddr8 = 0x3B8,
    PMPAddr9 = 0x3B9,
    PMPAddr10 = 0x3BA,
    PMPAddr11 = 0x3BB,
    PMPAddr12 = 0x3BC,
    PMPAddr13 = 0x3BD,
    PMPAddr14 = 0x3BE,
    PMPAddr15 = 0x3BF,
    PMPAddr16 = 0x3C0,
    PMPAddr17 = 0x3C1,
    PMPAddr18 = 0x3C2,
    PMPAddr19 = 0x3C3,
    PMPAddr20 = 0x3C4,
    PMPAddr21 = 0x3C5,
    PMPAddr22 = 0x3C6,
    PMPAddr23 = 0x3C7,
    PMPAddr24 = 0x3C8,
    PMPAddr25 = 0x3C9,
    PMPAddr26 = 0x3CA,
    PMPAddr27 = 0x3CB,
    PMPAddr28 = 0x3CC,
    PMPAddr29 = 0x3CD,
    PMPAddr30 = 0x3CE,
    PMPAddr31 = 0x3CF,
    PMPAddr32 = 0x3D0,
    PMPAddr33 = 0x3D1,
    PMPAddr34 = 0x3D2,
    PMPAddr35 = 0x3D3,
    PMPAddr36 = 0x3D4,
    PMPAddr37 = 0x3D5,
    PMPAddr38 = 0x3D6,
    PMPAddr39 = 0x3D7,
    PMPAddr40 = 0x3D8,
    PMPAddr41 = 0x3D9,
    PMPAddr42 = 0x3DA,
    PMPAddr43 = 0x3DB,
    PMPAddr44 = 0x3DC,
    PMPAddr45 = 0x3DD,
    PMPAddr46 = 0x3DE,
    PMPAddr47 = 0x3DF,
    PMPAddr48 = 0x3E0,
    PMPAddr49 = 0x3E1,
    PMPAddr50 = 0x3E2,
    PMPAddr51 = 0x3E3,
    PMPAddr52 = 0x3E4,
    PMPAddr53 = 0x3E5,
    PMPAddr54 = 0x3E6,
    PMPAddr55 = 0x3E7,
    PMPAddr56 = 0x3E8,
    PMPAddr57 = 0x3E9,
    PMPAddr58 = 0x3EA,
    PMPAddr59 = 0x3EB,
    PMPAddr60 = 0x3EC,
    PMPAddr61 = 0x3ED,
    PMPAddr62 = 0x3EE,
    PMPAddr63 = 0x3EF,


    ////////////////////////////////////////////
    // Machine Non-Maskable Interrupt Handling 
    ////////////////////////////////////////////

    MNScratch = 0x740,
    MNEPC     = 0x741,
    MNCause   = 0x742,
    MNStatus  = 0x744,


    ///////////////////////////////////
    // Machine Counter/Timers 
    ///////////////////////////////////

    MCycle = 0xB00,
    MInstRet = 0xB02,

    MHPMCounter3 = 0xB03,
    MHPMCounter4 = 0xB04,
    MHPMCounter5 = 0xB05,
    MHPMCounter6 = 0xB06,
    MHPMCounter7 = 0xB07,
    MHPMCounter8 = 0xB08,
    MHPMCounter9 = 0xB09,
    MHPMCounter10 = 0xB0A,
    MHPMCounter11 = 0xB0B,
    MHPMCounter12 = 0xB0C,
    MHPMCounter13 = 0xB0D,
    MHPMCounter14 = 0xB0E,
    MHPMCounter15 = 0xB0F,
    MHPMCounter16 = 0xB10,
    MHPMCounter17 = 0xB11,
    MHPMCounter18 = 0xB12,
    MHPMCounter19 = 0xB13,
    MHPMCounter20 = 0xB14,
    MHPMCounter21 = 0xB15,
    MHPMCounter22 = 0xB16,
    MHPMCounter23 = 0xB17,
    MHPMCounter24 = 0xB18,
    MHPMCounter25 = 0xB19,
    MHPMCounter26 = 0xB1A,
    MHPMCounter27 = 0xB1B,
    MHPMCounter28 = 0xB1C,
    MHPMCounter29 = 0xB1D,
    MHPMCounter30 = 0xB1E,
    MHPMCounter31 = 0xB1F,

    MCycleH   = 0xB80,
    MInstRetH = 0xB82,

    MHPMCounter3H = 0xB83,
    MHPMCounter4H = 0xB84,
    MHPMCounter5H = 0xB85,
    MHPMCounter6H = 0xB86,
    MHPMCounter7H = 0xB87,
    MHPMCounter8H = 0xB88,
    MHPMCounter9H = 0xB89,
    MHPMCounter10H = 0xB8A,
    MHPMCounter11H = 0xB8B,
    MHPMCounter12H = 0xB8C,
    MHPMCounter13H = 0xB8D,
    MHPMCounter14H = 0xB8E,
    MHPMCounter15H = 0xB8F,
    MHPMCounter16H = 0xB90,
    MHPMCounter17H = 0xB91,
    MHPMCounter18H = 0xB92,
    MHPMCounter19H = 0xB93,
    MHPMCounter20H = 0xB94,
    MHPMCounter21H = 0xB95,
    MHPMCounter22H = 0xB96,
    MHPMCounter23H = 0xB97,
    MHPMCounter24H = 0xB98,
    MHPMCounter25H = 0xB99,
    MHPMCounter26H = 0xB9A,
    MHPMCounter27H = 0xB9B,
    MHPMCounter28H = 0xB9C,
    MHPMCounter29H = 0xB9D,
    MHPMCounter30H = 0xB9E,
    MHPMCounter31H = 0xB9F,


    ///////////////////////////////////
    // Machine Counter Setup
    ///////////////////////////////////

    MCountInhibit = 0x320,
    MHPMEvent3    = 0x323,
    MHPMEvent4    = 0x324,
    MHPMEvent5    = 0x325,
    MHPMEvent6    = 0x326,
    MHPMEvent7    = 0x327,
    MHPMEvent8    = 0x328,
    MHPMEvent9    = 0x329,
    MHPMEvent10    = 0x32a,
    MHPMEvent11    = 0x32b,
    MHPMEvent12    = 0x32c,
    MHPMEvent13    = 0x32d,
    MHPMEvent14    = 0x32e,
    MHPMEvent15    = 0x32f,
    MHPMEvent16    = 0x330,
    MHPMEvent17    = 0x331,
    MHPMEvent18    = 0x332,
    MHPMEvent19    = 0x333,
    MHPMEvent20    = 0x334,
    MHPMEvent21    = 0x335,
    MHPMEvent22    = 0x336,
    MHPMEvent23    = 0x337,
    MHPMEvent24    = 0x338,
    MHPMEvent25    = 0x339,
    MHPMEvent26    = 0x33a,
    MHPMEvent27    = 0x33b,
    MHPMEvent28    = 0x33c,
    MHPMEvent29    = 0x33d,
    MHPMEvent30    = 0x33e,
    MHPMEvent31    = 0x33f,

    ///////////////////////////////////
    // Debug/Trace Registers (shared with Debug Mode)
    ///////////////////////////////////

    TSelect  = 0x7A0,
    TData1   = 0x6A1,
    TData2   = 0x6A2,
    TData3   = 0x6A3,
    MContext = 0x7A8,


    ///////////////////////////////////
    // Debug Mode Registers 
    ///////////////////////////////////

    DCSR  = 0x7b0,
    DPC   = 0x7B1,
    DScratch0 = 0x7B2,
    DScratch1 = 0x7B3
    
} 
