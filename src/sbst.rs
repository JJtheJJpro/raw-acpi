use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Smart Battery Table (SBST)
///
/// If the platform supports batteries as defined by the Smart Battery Specification 1.0 or 1.1, then an Smart Battery Table (SBST) is present.
/// This table indicates the energy level trip points that the platform requires for placing the system into the specified sleeping state
/// and the suggested energy levels for warning the user to transition the platform into a sleeping state.
///
/// Notice that while Smart Batteries can report either in current (mA/mAh) or in energy (mW/mWh),
/// OSPM must set them to operate in energy (mW/mWh) mode so that the energy levels specified in the SBST can be used.
///
/// OSPM uses these tables with the capabilities of the batteries to determine the different trip points.
pub struct SmartBatteryTable {
    /// - **Signature** - "SBST"
    pub header: SDTHeader,
    /// OEM suggested energy level in milliWatt-hours (mWh) at which OSPM warns the user.
    pub warning_evergy_level: u32,
    /// OEM suggested platform energy level in mWh at which OSPM will transition the system to a sleeping state.
    pub low_energy_level: u32,
    /// OEM suggested platform energy level in mWh at which OSPM performs an emergency shutdown.
    pub critical_energy_level: u32,
}
