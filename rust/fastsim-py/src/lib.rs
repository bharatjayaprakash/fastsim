//! # Crate features
//! * **full** - When enabled (which is default), include additional capabilities that
//!   require additional dependencies
//! * **resources** - When enabled (which is triggered by enabling full (thus default)
//!   or enabling this feature directly), compiles commonly used resources (e.g.
//!   standard drive cycles) for faster access.

#[cfg(feature = "simdrivelabel")]
use fastsim_core::simdrivelabel::*;
use fastsim_core::*;
use pyo3imports::*;

/// Function for adding Rust structs as Python Classes
#[pymodule]
fn fastsimrust(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    #[cfg(feature = "logging")]
    pyo3_log::init();
    m.add_class::<cycle::RustCycle>()?;
    m.add_class::<vehicle::RustVehicle>()?;
    m.add_class::<params::RustPhysicalProperties>()?;
    m.add_class::<params::AdjCoef>()?;
    m.add_class::<params::RustLongParams>()?;
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

    // Features
    #[cfg(feature = "default")]
    m.add_function(wrap_pyfunction!(vehicle_utils::abc_to_drag_coeffs, m)?)?;
    #[cfg(feature = "simdrivelabel")]
    {
        m.add_class::<simdrivelabel::LabelFe>()?;
        m.add_class::<simdrivelabel::LabelFePHEV>()?;
        m.add_class::<simdrivelabel::PHEVCycleCalc>()?;
        m.add_class::<simdrive::simdrive_iter::SimDriveVec>()?;
        m.add_function(wrap_pyfunction!(make_accel_trace_py, m)?)?;
        m.add_function(wrap_pyfunction!(get_net_accel_py, m)?)?;
        m.add_function(wrap_pyfunction!(get_label_fe_py, m)?)?;
        m.add_function(wrap_pyfunction!(get_label_fe_phev_py, m)?)?;
    }
    #[cfg(feature = "vehicle-import")]
    {
        m.add_class::<vehicle_import::OtherVehicleInputs>()?;
        m.add_function(wrap_pyfunction!(
            vehicle_import::get_options_for_year_make_model,
            m
        )?)?;
        m.add_function(wrap_pyfunction!(
            vehicle_import::get_vehicle_data_for_id,
            m
        )?)?;
        m.add_function(wrap_pyfunction!(
            vehicle_import::vehicle_import_by_id_and_year,
            m
        )?)?;
        m.add_function(wrap_pyfunction!(vehicle_import::import_all_vehicles, m)?)?;
    }
    // Function to check what features are enabled from Python
    m.add_function(wrap_pyfunction!(enabled_features, m)?)?;

    Ok(())
}
