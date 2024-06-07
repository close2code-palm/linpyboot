use nix::libc::{RB_DISABLE_CAD, RB_ENABLE_CAD};
use nix::sys;
use nix::sys::reboot::RebootMode;
use pyo3::{Bound, pyfunction, pymodule, PyResult, wrap_pyfunction};
use pyo3::prelude::PyModule;

#[pyfunction]
// #[cfg(target_os="linux")]
fn reboot(mode: i64) -> PyResult<()> {
    if mode as i32 == RB_DISABLE_CAD {
        // sys::reboot::set_cad_enabled(false)?;
        println!("CAD dis")
    } else if mode as u32 as i32 == RB_ENABLE_CAD {
        // sys::reboot::set_cad_enabled(true)?;
        println!("CAD enabled")
    } else {
        let reboot_mode = match mode as u32 {
            0xcdef0123u32 => RebootMode::RB_HALT_SYSTEM,
            0x45584543u32 => RebootMode::RB_KEXEC,
            0x4321fedcu32 => RebootMode::RB_POWER_OFF,
            0xd000fce2u32 => RebootMode::RB_SW_SUSPEND,
            _ => RebootMode::RB_AUTOBOOT,
        };
        println!("{reboot_mode}");
    }
    // sys::reboot::reboot(reboot_mode).unwrap();
    Ok(())
}


#[pymodule]
fn rebo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reboot, m)?)?;
    Ok(())
}
