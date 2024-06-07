use nix::errno::Errno;
use nix::libc::{RB_DISABLE_CAD, RB_ENABLE_CAD};
use nix::sys;
use nix::sys::reboot::RebootMode;
use pyo3::prelude::PyModule;
use pyo3::{create_exception, pyfunction, pymodule, wrap_pyfunction, Bound, PyResult};

create_exception!(rebo, PermissionError, pyo3::exceptions::PyException);
create_exception!(rebo, CommandError, pyo3::exceptions::PyException);

#[pyfunction]
fn reboot(mode: i64) -> PyResult<()> {
    if mode as i32 == RB_DISABLE_CAD {
        return handle_reboot_execution(sys::reboot::set_cad_enabled(false));
    } else if mode as u32 as i32 == RB_ENABLE_CAD {
        return handle_reboot_execution(sys::reboot::set_cad_enabled(true));
    } else {
        let reboot_mode = match mode as u32 {
            0xcdef0123u32 => RebootMode::RB_HALT_SYSTEM,
            0x45584543u32 => RebootMode::RB_KEXEC,
            0x4321fedcu32 => RebootMode::RB_POWER_OFF,
            0xd000fce2u32 => RebootMode::RB_SW_SUSPEND,
            0x01234567u32 => RebootMode::RB_AUTOBOOT,
            _ => {
                return Err(CommandError::new_err(
                    "Argument is unknown or not supported.",
                ))
            }
        };
        sys::reboot::reboot(reboot_mode).unwrap();
    }
    Ok(())
}

fn handle_reboot_execution<T>(execution: Result<(), T>) -> PyResult<()> {
    return match execution {
        Ok(_) => Ok(()),
        Err(err) => match err {
            Errno::EPERM => Err(PermissionError::new_err(
                "Process needs CAP_SYS_BOOT capability set.",
            )),
            _ => Err(CommandError::new_err("Unknown argument.")),
        },
    };
}

#[pymodule]
fn rebo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reboot, m)?)?;
    Ok(())
}
