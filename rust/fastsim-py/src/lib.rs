use fastsim_core::*;

use pyo3imports::*;
use fastsim_core::simdrivelabel::*;


/// Function for adding Rust structs as Python Classes
#[pymodule]
fn fastsimrust(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<cycle::RustCycle>()?;
    m.add_class::<vehicle::RustVehicle>()?;
    m.add_class::<params::RustPhysicalProperties>()?;
    m.add_class::<utils::Pyo3ArrayU32>()?;
    m.add_class::<utils::Pyo3ArrayF64>()?;
    m.add_class::<utils::Pyo3ArrayBool>()?;
    m.add_class::<utils::Pyo3VecF64>()?;
    m.add_class::<simdrive::RustSimDriveParams>()?;
    m.add_class::<simdrive::RustSimDrive>()?;
    m.add_class::<thermal::SimDriveHot>()?;
    m.add_class::<vehicle_thermal::VehicleThermal>()?;
    m.add_class::<thermal::ThermalState>()?;
    m.add_class::<vehicle_thermal::HVACModel>()?;

    cycle::register(py, m)?;
    m.add_function(wrap_pyfunction!(vehicle_utils::abc_to_drag_coeffs, m)?)?;
    m.add_function(wrap_pyfunction!(make_accel_trace_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_net_accel_py, m)?)?;
    #[cfg(feature = "pyo3")]
    {
        m.add_class::<LabelFe>()?;
        m.add_function(wrap_pyfunction!(get_label_fe_py, m)?)?;
        m.add_function(wrap_pyfunction!(get_label_fe_phev_py, m)?)?;
        m.add_function(wrap_pyfunction!(get_label_fe_conv_py, m)?)?;

    }
   
 Ok(())
}
