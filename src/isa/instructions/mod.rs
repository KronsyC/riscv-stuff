///
/// Each Instruction in the RISC-V spec follows one of these formats
///
/// Each immediate subfield is labeled with the bit
/// position (imm(x)) in the immediate value being produced, rather than the bit position within the
/// instruction’s immediate field as is usually done.
///
pub enum InstructionFormat{
    ///
    /// R-Type Instructions
    ///
    /// Register - Register
    ///
    ///   31       24    19    14       11    6
    ///   | funct7 | rs2 | rs1 | funct3 | rd | opcode |
    ///
    RType,

    ///
    /// I-Type Instructions 
    ///
    /// Short Immediates and Loads
    ///
    ///   31             19    14       11    6
    ///   |   imm(11:0)  | rs1 | funct3 | rd | opcode |
    ///
    IType,

    ///
    /// S-Type Instructions
    ///
    /// Stores
    ///
    ///   31       24    19    14       11          6
    ///   | imm11:5| rs2 | rs1 | funct3 | imm(4:0) | opcode |
    ///
    SType,

    ///
    /// B-Type Instructions 
    ///
    /// Conditional Branches (S-Type variation)
    ///
    ///   31         30         24    19    14       11          7         6
    ///   | imm(12) | imm(10:5) | rs2 | rs1 | funct3 | imm(4:1) | imm(11) | opcode |
    ///   
    BType,

    ///
    /// U-Type Instructions 
    ///
    /// Long Immediates
    ///
    ///   31                            11    6
    ///   |           imm(31:12)        | rd | opcode |
    ///
    UType,

    ///
    /// J-Type Instructions
    ///
    /// Unconditional Jumps (B-Type variation)
    ///
    ///   31         30         20         19          14        11    6
    ///   | imm(20) | imm(10:1) | imm(11) | imm(19:12) | funct3 | rd | opcode |
    ///   
    JType,
}

pub enum MachinePrivilegedInstructionType{

}

pub enum SupervisorPrivilegedInstructionType{

}

pub enum UserPrivilegedInstructionType{

}


pub enum PrivilegedInstructionType{
    Machine(MachinePrivilegedInstructionType),
    Supervisor(SupervisorPrivilegedInstructionType),
    User(UserPrivilegedInstructionType)
}

pub enum NonPrivilegedInstructionType{

}


///
/// We use the InstructionType to encode all instruction types, alongside their characteristics
///
/// Characteristics Include:
/// - Required Environment 
/// - Side-Effects
///
#[allow(non_camel_case_types)]
pub enum InstructionType{
    /////////////////////////////////////////
    // Machine-Mode Privileged Instructions
    /////////////////////////////////////////


    ///
    /// Environment Call (I)
    ///
    /// The ECALL instruction is used to make a request to the supporting execution environment. When
    /// executed in U-mode, S-mode, or M-mode, it generates an environment-call-from-U-mode exception,
    /// environment-call-from-S-mode exception, or environment-call-from-M-mode exception, respectively,
    /// and performs no other operation.
    ///
    ECall,

    ///
    /// Environment Breakpoint (I)
    ///
    /// The EBREAK instruction is used by debuggers to cause control to be transferred back to a debugging
    /// environment. It generates a breakpoint exception and performs no other operation.
    ///
    EBreak,

    ///
    /// Machine Return From Trap (I)
    ///
    MRet,

    ///
    /// Wait for Interrupt (I)
    ///
    /// The Wait for Interrupt instruction (WFI) informs the implementation that the current hart can be
    /// stalled until an interrupt might need servicing. Execution of the WFI instruction can also be used to
    /// inform the hardware platform that suitable interrupts should preferentially be routed to this hart. WFI
    /// is available in all privileged modes, and optionally available to U-mode. This instruction may raise an
    /// illegal-instruction exception when TW=1 in mstatus, as described in Section 3.1.6.5.
    ///
    WFI,


    /////////////////////////////////////////
    // Supervisor-Mode Privileged Instructions
    /////////////////////////////////////////


    ///
    /// Supervisor Return From Trap (I)
    ///
    SRet,


    ///
    /// Supervisor Memory-Management Fence (R)
    ///
    /// The supervisor memory-management fence instruction SFENCE.VMA is used to synchronize updates
    /// to in-memory memory-management data structures with current execution. Instruction execution
    /// causes implicit reads and writes to these data structures; however, these implicit references are
    /// ordinarily not ordered with respect to explicit loads and stores. Executing an SFENCE.VMA
    /// instruction guarantees that any previous stores already visible to the current RISC-V hart are ordered
    /// before certain implicit references by subsequent instructions in that hart to the memory-management
    /// data structures. The specific set of operations ordered by SFENCE.VMA is determined by rs1 and rs2,
    /// as described below. SFENCE.VMA is also used to invalidate entries in the address-translation cache
    /// associated with a hart.
    ///
    SFence_VMA,

    /////////////////////////////////////////
    // Hypervisor-Mode Privileged Instructions
    /////////////////////////////////////////

    ///
    /// TODO: Document
    ///
    HLV_Width,


    ///
    /// TODO: Document
    ///
    HLVX_HU,


    ///
    /// TODO: Document
    ///
    HLVX_WU,


    ///
    /// TODO: Document
    ///
    HSV_Width,


    ///
    /// TODO: Document
    ///
    HFence_VVMA,

    ///
    /// TODO: Document
    ///
    HFence_GVMA,


    ///////////////////////////////////////////////
    ///
    // Unprivileged Instructions 
    //
    //
    // These instructions are defined in the 
    // unprivileged RISC-V spec located at 
    // https://github.com/riscv/riscv-isa-manual/releases/latest
    //
    //
    ///////////////////////////////////////////////


    ///////////////////////////////////////////
    // Integer Register-Immediate Instructions 
    ///////////////////////////////////////////

    ///
    /// Add Immediate (I)
    ///
    /// ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and the
    /// result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to implement the MV rd, rs1
    /// assembler pseudoinstruction.
    ///
    ADDI,

    ///
    /// Set less than Immediate (I)
    ///
    /// SLTI places the value 1 in register rd if register rs1 is less than the sign-
    /// extended immediate when both are treated as signed numbers, else 0 is written to rd.
    ///
    SLTI,

    ///
    /// Set less than Immediate Unsigned (I)
    ///
    /// SLTIU places the value 1 in register rd if register rs1 is less than the sign-
    /// extended immediate when both are treated as unsigned numbers, else 0 is written to rd.
    ///
    SLTIU,

    ///
    /// Logical AND Immediate (I)
    ///
    ANDI,

    ///
    /// Logical OR Immediate (I)
    ///
    ORI,

    ///
    /// Logical XOR Immediate (I)
    ///
    XORI,

    ///
    /// Logical Left-Shift Immediate (I)
    ///
    SLLI,

    ///
    /// Logical Right-Shift Immediate (I)
    ///
    SRLI,

    ///
    /// Arithmetic Right-Shift Immediate (I)
    ///
    SRAI,
    

    ///
    /// Load Upper Immediate (U)
    ///
    /// LUI is used to build 32-bit constants and uses the U-type format. LUI places the 32-bit
    /// U-immediate value into the destination register rd, filling in the lowest 12 bits with zeros.
    ///
    LUI,


    ///
    /// Add Upper Immediate to Program Counter (U)
    ///
    /// AUIPC is used to build pc-relative addresses and uses the U-type format.
    /// AUIPC forms a 32-bit offset from the U-immediate, filling in the lowest 12 bits with zeros, adds this
    /// offset to the address of the AUIPC instruction, then places the result in register rd.
    ///
    AUIPC,


    ///////////////////////////////////////////
    // Integer Register-Register Instructions 
    ///////////////////////////////////////////
   
    ///
    /// Add (R)
    ///
    ADD,

    ///
    /// Set less than (R)
    ///
    /// Writes 1 to rd if rs1 < rs2, else 0
    ///
    SLT,

    ///
    /// Set less than Unsigned (R)
    ///
    /// Writes 1 to rd if rs1 < rs2, else 0
    ///
    SLTU,

    ///
    /// Logical AND (R)
    ///
    AND,

    ///
    /// Logical OR (R)
    ///
    OR,

    ///
    /// Logical XOR (R)
    ///
    XOR,

    ///
    /// Logical Shift Left (R)
    ///
    /// The value in rs1 in shifted by the lower 5 bits in rs2
    ///
    SLL,

    ///
    /// Logical Shift Right (R)
    ///
    /// The value in rs1 in shifted by the lower 5 bits in rs2
    ///
    SRL,

    ///
    /// Subtract (R)
    ///
    /// rs2 is subtracted from rs1 
    ///
    /// (rd = rs1 - rs2)
    ///
    SUB,

    ///
    /// Arithmetic Shift Right (R)
    ///
    /// The value in rs1 in shifted by the lower 5 bits in rs2
    ///
    SRA,


    ///
    /// No Operation (I)
    ///
    /// The NOP instruction does not change any architecturally visible state, except for advancing the pc and
    /// incrementing any applicable performance counters. NOP is encoded as ADDI x0, x0, 0.
    ///
    NOP,

}

pub trait Instruction{
    const CODE : InstructionType;
}
