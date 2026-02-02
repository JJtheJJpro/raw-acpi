#[derive(Copy, Clone)]
/// Polarity of the APIC I/O input signals.
pub enum InterruptSourceOverridePolarity {
    /// Conforms to the specifications of the bus (for example, EISA is active-low for level-triggered interrupts).
    Conform,
    /// Active high.
    ActiveHigh,
    /// Active low.
    ActiveLow = 3,
}
#[derive(Copy, Clone)]
/// Trigger mode of the APIC I/O Input signals.
pub enum InterruptSourceOverrideTriggerMode {
    /// Conforms to specifications of the bus (For example, ISA is edge-triggered).
    Conform,
    /// Edge-triggered.
    EdgeTriggered,
    /// Level-triggered.
    LevelTriggered = 3,
}

#[derive(Copy, Clone)]
/// The MPS INTI Flags are identical to the flags used in the MPS version 1.4 specification (Table 4-10).
/// 
/// The Polarity flags are the PO bits and the Trigger Mode flags are the EL bits.
pub struct MPSINTIFlags(u16);
impl MPSINTIFlags {
    /// Polarity of the APIC I/O input signals.
    pub const fn polarity(&self) -> InterruptSourceOverridePolarity {
        match self.0 & 0b0011 {
            0b00 => InterruptSourceOverridePolarity::Conform,
            0b01 => InterruptSourceOverridePolarity::ActiveHigh,
            0b10 => panic!(
                "InterruptSourceOverride's Polarity 2-bit flag set to 0b10, which is a reserved value."
            ),
            0b11 => InterruptSourceOverridePolarity::ActiveLow,
            _ => unreachable!(),
        }
    }
    /// Trigger mode of the APIC I/O Input signals.
    pub const fn trigger_mode(&self) -> InterruptSourceOverrideTriggerMode {
        match self.0 & 0b1100 {
            0b00 => InterruptSourceOverrideTriggerMode::Conform,
            0b01 => InterruptSourceOverrideTriggerMode::EdgeTriggered,
            0b10 => panic!(
                "InterruptSourceOverride's Polarity 2-bit flag set to 0b10, which is a reserved value."
            ),
            0b11 => InterruptSourceOverrideTriggerMode::LevelTriggered,
            _ => unreachable!(),
        }
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Interrupt Source Override Structure
///
/// Interrupt Source Overrides are necessary to describe variances between the IA-PC standard dual 8259 interrupt definition and the platform's implementation.
///
/// It is assumed that the ISA interrupts will be identity-mapped into the first I/O APIC sources. Most existing APIC designs, however, will contain at least one exception to this assumption.
/// The Interrupt Source Override Structure is provided in order to describe these exceptions. It is not necessary to provide an Interrupt Source Override for every ISA interrupt.
/// Only those that are not identity-mapped onto the APIC interrupt inputs need be described.
///
/// This specification only supports overriding ISA interrupt sources.
///
/// For example, if your machine has the ISA Programmable Interrupt Timer (PIT) connected to ISA IRQ 0, but in APIC mode, it is connected to I/O APIC interrupt input 2,
/// then you would need an Interrupt Source Override where the source entry is '0' and the Global System Interrupt is '2.'
pub struct InterruptSourceOverride {
    /// 2 - Interrupt Source Override
    pub r#type: u8,
    /// 10
    pub length: u8,
    /// **JJ's Note: The docs say "0 Constant, meaning ISA"...I have no idea what that means.**
    pub bus: u8,
    /// Bus-relative interrupt source (IRQ)
    pub source: u8,
    /// The Global System Interrupt that this bus-relative interrupt source will signal.
    pub global_system_interrupt: u32,
    /// MPS INTI flags. See the corresponding tabel below for a description of this field.
    pub flags: MPSINTIFlags,
}