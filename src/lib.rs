use nix::sys;
use nix::sys::reboot::RebootMode;
use pyo3::{Bound, pyfunction, pymodule, PyResult, wrap_pyfunction};
use pyo3::prelude::PyModule;

#[pyfunction]
fn reboot(mode: i32) -> PyResult<()> {
    let reboot_mode = match mode {
        0xcdef0123 => RebootMode::RB_HALT_SYSTEM,
        0x45584543 => RebootMode::RB_KEXEC,
        0x4321fedc => RebootMode::RB_POWER_OFF,
        0xd000fce2 => RebootMode::RB_SW_SUSPEND,
        _ => RebootMode::RB_AUTOBOOT,
    };
    sys::reboot::reboot(reboot_mode).unwrap();
    Ok(())
}


#[pymodule]
fn rebo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reboot, m)?)?;
    Ok(())
}
