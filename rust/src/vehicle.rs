extern crate ndarray;
use ndarray::{Array, Array1}; 
extern crate pyo3;
use pyo3::prelude::*;
// use numpy::pyo3::Python;
// use numpy::ndarray::array;
// use numpy::{ToPyArray, PyArray};

// local 
use super::params::*;
//use super::utils::*;

const VEH_PT_TYPES:Vec<String> = vec![
    String::from("Conv"), 
    String::from("HEV"), 
    String::from("PHEV"), 
    String::from("BEV")];

const CONV:String = VEH_PT_TYPES[0];
const HEV:String = VEH_PT_TYPES[1];
const PHEV:String = VEH_PT_TYPES[2];
const BEV:String = VEH_PT_TYPES[3];

const FC_EFF_TYPES:Vec<String> = vec![
    String::from("SI"), 
    String::from("Atkinson"), 
    String::from("Diesel"), 
    String::from("H2FC"), 
    String::from("HD_Diesel")];

const SI:String = FC_EFF_TYPES[0];
const ATKINSON:String = FC_EFF_TYPES[1];
const DIESEL:String = FC_EFF_TYPES[2];
const H2FC:String = FC_EFF_TYPES[3];
const HD_DIESEL:String = FC_EFF_TYPES[4];


#[pyclass] 
#[derive(Debug, Clone)]
/// Struct containing time trace data 
pub struct RustVehicle{
    #[pyo3(get, set)]
    pub props: RustPhysicalProperties, 
    #[pyo3(get, set)]
    pub scenario_name: String,
    #[pyo3(get, set)]
    pub selection: u32,
    #[pyo3(get, set)]
    pub veh_year: u32,
    #[pyo3(get, set)]
    pub veh_pt_type: String,
    #[pyo3(get, set)]
    pub drag_coef: f64,
    #[pyo3(get, set)]
    pub frontal_area_m2: f64,
    #[pyo3(get, set)]
    pub glider_kg: f64,
    #[pyo3(get, set)]
    pub veh_cg_m: f64,
    #[pyo3(get, set)]
    pub drive_axle_weight_frac: f64,
    #[pyo3(get, set)]
    pub wheel_base_m: f64,
    #[pyo3(get, set)]
    pub cargo_kg: f64,
    #[pyo3(get, set)]
    pub veh_override_kg: f64,
    #[pyo3(get, set)]
    pub comp_mass_multiplier: f64,
    #[pyo3(get, set)]
    pub max_fuel_stor_kw: f64,
    #[pyo3(get, set)]
    pub fuel_stor_secs_to_peak_pwr: f64,
    #[pyo3(get, set)]
    pub fuel_stor_kwh: f64,
    #[pyo3(get, set)]
    pub fuel_stor_kwh_per_kg: f64,
    #[pyo3(get, set)]
    pub max_fuel_conv_kw: f64,
    pub fc_pwr_out_perc: Array1<f64>,
    pub fc_eff_map: Array1<f64>,
    #[pyo3(get, set)]
    pub fc_eff_type: String,
    #[pyo3(get, set)]
    pub fuel_conv_secs_to_peak_pwr: f64,
    #[pyo3(get, set)]
    pub fuel_conv_base_kg: f64,
    #[pyo3(get, set)]
    pub fuel_conv_kw_per_kg: f64,
    #[pyo3(get, set)]
    pub min_fc_time_on: f64,
    #[pyo3(get, set)]
    pub idle_fc_kw: f64,
    #[pyo3(get, set)]
    pub max_motor_kw: f64,
    pub mc_pwr_out_perc: Array1<f64>,
    pub mc_eff_map: Array1<f64>,
    #[pyo3(get, set)]
    pub motor_secs_to_peak_pwr: f64,
    #[pyo3(get, set)]
    pub mc_pe_kg_per_kw: f64,
    #[pyo3(get, set)]
    pub mc_pe_base_kg: f64,
    #[pyo3(get, set)]
    pub max_ess_kw: f64,
    #[pyo3(get, set)]
    pub max_ess_kwh: f64,
    #[pyo3(get, set)]
    pub ess_kg_per_kwh: f64,
    #[pyo3(get, set)]
    pub ess_base_kg: f64,
    #[pyo3(get, set)]
    pub ess_round_trip_eff: f64,
    #[pyo3(get, set)]
    pub ess_life_coef_a: f64,
    #[pyo3(get, set)]
    pub ess_life_coef_b: f64,
    #[pyo3(get, set)]
    pub min_soc: f64,
    #[pyo3(get, set)]
    pub max_soc: f64,
    #[pyo3(get, set)]
    pub ess_dischg_to_fc_max_eff_perc: f64,
    #[pyo3(get, set)]
    pub ess_chg_to_fc_max_eff_perc: f64,
    #[pyo3(get, set)]
    pub wheel_inertia_kg_m2: f64,
    #[pyo3(get, set)]
    pub num_wheels: f64,
    #[pyo3(get, set)]
    pub wheel_rr_coef: f64,
    #[pyo3(get, set)]
    pub wheel_radius_m: f64,
    #[pyo3(get, set)]
    pub wheel_coef_of_fric: f64,
    #[pyo3(get, set)]
    pub max_accel_buffer_mph: f64,
    #[pyo3(get, set)]
    pub max_accel_buffer_perc_of_useable_soc: f64,
    #[pyo3(get, set)]
    pub perc_high_acc_buf: f64,
    #[pyo3(get, set)]
    pub mph_fc_on: f64,
    #[pyo3(get, set)]
    pub kw_demand_fc_on: f64,
    #[pyo3(get, set)]
    pub max_regen: f64,
    #[pyo3(get, set)]
    pub stop_start: bool,
    #[pyo3(get, set)]
    pub force_aux_on_fc: f64,
    #[pyo3(get, set)]
    pub alt_eff: f64,
    #[pyo3(get, set)]
    pub chg_eff: f64,
    #[pyo3(get, set)]
    pub aux_kw: f64,
    #[pyo3(get, set)]
    pub trans_kg: f64,
    #[pyo3(get, set)]
    pub trans_eff: f64,
    #[pyo3(get, set)]
    pub ess_to_fuel_ok_error: f64,
    pub large_baseline_eff: Array1<f64>,
    pub small_baseline_eff: Array1<f64>,
    #[pyo3(get, set)]
    pub small_motor_power_kw: f64,
    #[pyo3(get, set)]
    pub large_motor_power_kw: f64,
    pub fc_perc_out_array: Array1<f64>,
    #[pyo3(get, set)]
    pub regen_a: f64,
    #[pyo3(get, set)]
    pub regen_b: f64,
    #[pyo3(get, set)]
    pub charging_on: bool,
    #[pyo3(get, set)]
    pub no_elec_sys: bool,
    #[pyo3(get, set)]
    pub no_elec_aux: bool,
    pub max_roadway_chg_kw: Array1<f64>,
    pub input_kw_out_array: Array1<f64>,
    pub fc_kw_out_array: Array1<f64>,
    pub fc_eff_array: Array1<f64>,
    #[pyo3(get, set)]
    pub modern_max: f64,
    pub mc_eff_array: Array1<f64>,
    pub mc_kw_in_array: Array1<f64>,
    pub mc_kw_out_array: Array1<f64>,
    #[pyo3(get, set)]
    pub mc_max_elec_in_kw: f64,
    pub mc_full_eff_array: Array1<f64>,
    #[pyo3(get, set)]
    pub veh_kg: f64,
    #[pyo3(get, set)]
    pub max_trac_mps2: f64,
    #[pyo3(get, set)]
    pub ess_mass_kg: f64,
    #[pyo3(get, set)]
    pub mc_mass_kg: f64,
    #[pyo3(get, set)]
    pub fc_mass_kg: f64,
    #[pyo3(get, set)]
    pub fs_mass_kg: f64,
    pub mc_perc_out_array: Array1<f64>,
    // these probably don't need to be in rust
    pub val_udds_mpgge: f64,
    pub val_hwy_mpgge: f64,
    pub val_comb_mpgge: f64,
    pub val_udds_kwh_per_mile: f64,
    pub val_hwy_kwh_per_mile: f64,
    pub val_comb_kwh_per_mile: f64,
    pub val_cd_range_mi: f64,
    pub val_const65_mph_kwh_per_mile: f64,
    pub val_const60_mph_kwh_per_mile: f64,
    pub val_const55_mph_kwh_per_mile: f64,
    pub val_const45_mph_kwh_per_mile: f64,
    pub val_unadj_udds_kwh_per_mile: f64,
    pub val_unadj_hwy_kwh_per_mile: f64,
    pub val0_to60_mph: f64,
    pub val_ess_life_miles: f64,
    pub val_range_miles: f64,
    pub val_veh_base_cost: f64,
    pub val_msrp: f64,
}

/// RustVehicle class for containing: 
#[pymethods]
impl RustVehicle{
    #[new]
    pub fn __new__(
        scenario_name: String,
        selection: u32,
        veh_year: u32,
        veh_pt_type: String,
        drag_coef: f64,
        frontal_area_m2: f64,
        glider_kg: f64,
        veh_cg_m: f64,
        drive_axle_weight_frac: f64,
        wheel_base_m: f64,
        cargo_kg: f64,
        veh_override_kg: f64,
        comp_mass_multiplier: f64,
        max_fuel_stor_kw: f64,
        fuel_stor_secs_to_peak_pwr: f64,
        fuel_stor_kwh: f64,
        fuel_stor_kwh_per_kg: f64,
        max_fuel_conv_kw: f64,
        fc_pwr_out_perc: Vec<f64>,
        fc_eff_map: Vec<f64>,
        fc_eff_type: String,
        fuel_conv_secs_to_peak_pwr: f64,
        fuel_conv_base_kg: f64,
        fuel_conv_kw_per_kg: f64,
        min_fc_time_on: f64,
        idle_fc_kw: f64,
        max_motor_kw: f64,
        mc_pwr_out_perc: Vec<f64>,
        mc_eff_map: Vec<f64>,
        motor_secs_to_peak_pwr: f64,
        mc_pe_kg_per_kw: f64,
        mc_pe_base_kg: f64,
        max_ess_kw: f64,
        max_ess_kwh: f64,
        ess_kg_per_kwh: f64,
        ess_base_kg: f64,
        ess_round_trip_eff: f64,
        ess_life_coef_a: f64,
        ess_life_coef_b: f64,
        min_soc: f64,
        max_soc: f64,
        ess_dischg_to_fc_max_eff_perc: f64,
        ess_chg_to_fc_max_eff_perc: f64,
        wheel_inertia_kg_m2: f64,
        num_wheels: f64,
        wheel_rr_coef: f64,
        wheel_radius_m: f64,
        wheel_coef_of_fric: f64,
        max_accel_buffer_mph: f64,
        max_accel_buffer_perc_of_useable_soc: f64,
        perc_high_acc_buf: f64,
        mph_fc_on: f64,
        kw_demand_fc_on: f64,
        max_regen: f64,
        stop_start: bool,
        force_aux_on_fc: f64,
        alt_eff: f64,
        chg_eff: f64,
        aux_kw: f64,
        trans_kg: f64,
        trans_eff: f64,
        ess_to_fuel_ok_error: f64,
        val_udds_mpgge: f64,
        val_hwy_mpgge: f64,
        val_comb_mpgge: f64,
        val_udds_kwh_per_mile: f64,
        val_hwy_kwh_per_mile: f64,
        val_comb_kwh_per_mile: f64,
        val_cd_range_mi: f64,
        val_const65_mph_kwh_per_mile: f64,
        val_const60_mph_kwh_per_mile: f64,
        val_const55_mph_kwh_per_mile: f64,
        val_const45_mph_kwh_per_mile: f64,
        val_unadj_udds_kwh_per_mile: f64,
        val_unadj_hwy_kwh_per_mile: f64,
        val0_to60_mph: f64,
        val_ess_life_miles: f64,
        val_range_miles: f64,
        val_veh_base_cost: f64,
        val_msrp: f64,
        props: RustPhysicalProperties,  
        large_baseline_eff: Vec<f64>,
        small_baseline_eff: Vec<f64>,
        small_motor_power_kw: f64,
        large_motor_power_kw: f64,
        fc_perc_out_array: Vec<f64>,
        // regen_a: f64,
        // regen_b: f64,
        charging_on: bool,
        no_elec_sys: bool,
        no_elec_aux: bool,
        max_roadway_chg_kw: Vec<f64>,
        // input_kw_out_array: Vec<f64>,
        // fc_kw_out_array: Vec<f64>,
        // fc_eff_array: Vec<f64>,
        // modern_max: f64,
        // mc_eff_array: Vec<f64>,
        // mc_kw_in_array: Vec<f64>,
        // mc_kw_out_array: Vec<f64>,
        // mc_max_elec_in_kw: f64,
        // mc_full_eff_array: Vec<f64>,
        // veh_kg: f64,
        // max_trac_mps2: f64,
        // ess_mass_kg: f64,
        // mc_mass_kg: f64,
        // fc_mass_kg: f64,
        // fs_mass_kg: f64,
        // mc_perc_out_array: Vec<f64>,
    ) -> Self{
        // Define Optionals
        let regen_a: f64 = 500.0;
        let regen_b: f64 = 0.99;

        let fc_pwr_out_perc = Array::from_vec(fc_pwr_out_perc);
        let fc_eff_map = Array::from_vec(fc_eff_map);
        let mc_pwr_out_perc = Array::from_vec(mc_pwr_out_perc);
        let mc_eff_map = Array::from_vec(mc_eff_map);
        let large_baseline_eff = Array::from_vec(large_baseline_eff);
        let small_baseline_eff = Array::from_vec(small_baseline_eff);
        let fc_perc_out_array = Array::from_vec(fc_perc_out_array);
        let max_roadway_chg_kw = Array::from_vec(max_roadway_chg_kw);
        // let input_kw_out_array = Array::from_vec(input_kw_out_array);
        // let fc_kw_out_array = Array::from_vec(fc_kw_out_array);
        // let fc_eff_array = Array::from_vec(fc_eff_array);
        // let mc_eff_array = Array::from_vec(mc_eff_array);
        // let mc_kw_in_array = Array::from_vec(mc_kw_in_array);
        // let mc_kw_out_array = Array::from_vec(mc_kw_out_array);
        // let mc_full_eff_array = Array::from_vec(mc_full_eff_array);
        // let mc_perc_out_array = Array::from_vec(mc_perc_out_array);

        // Derived Values
        let input_kw_out_array = fc_pwr_out_perc.clone() * max_fuel_conv_kw;
        let fc_kw_out_array = fc_perc_out_array.clone() * max_fuel_conv_kw;
        // TODO: need to do interpolation here... use `ndarray_stats`?
        let fc_eff_array = fc_eff_map.clone();
        let modern_max: f64 = 0.95;
        let mc_eff_array = mc_eff_map.clone();
        let mc_kw_out_array = Array::linspace(0.0, 1.0, 50) * max_motor_kw;
        // TODO: update to mc_full_eff_array = np.interp(x=self.mc_perc_out_array, xp=self.mc_pwr_out_perc, fp=self.mc_eff_array)
        let mc_full_eff_array = Array::ones(mc_eff_array.raw_dim());
        // TODO: need to do concatenate with [[0], mc_kw_out_array[1:] / mc_full_eff_array[1:]]
        let mc_kw_in_array: Array1<f64> = Array::ones(mc_kw_out_array.raw_dim());
        // TODO: edit below to be self.mc_max_elec_in_kw = max(mc_kw_in_array)
        let mc_max_elec_in_kw: f64 = 100.0;
        // TODO: implement proper derivation for ess_mass_kg; see Vehicle.set_veh_mass(...)
        let ess_mass_kg: f64 = 0.0;
        // TODO: implement proper derivation for ess_mass_kg; see Vehicle.set_veh_mass(...)
        let mc_mass_kg: f64 = 0.0;
        // TODO: implement proper derivation for ess_mass_kg; see Vehicle.set_veh_mass(...)
        let fc_mass_kg: f64 = 0.0;
        // TODO: implement proper derivation for ess_mass_kg; see Vehicle.set_veh_mass(...)
        let fs_mass_kg: f64 = 0.0;
        let veh_kg: f64 = cargo_kg + glider_kg + trans_kg * comp_mass_multiplier
            + ess_mass_kg + mc_mass_kg + fc_mass_kg + fs_mass_kg;
        let max_trac_mps2: f64 = (
            wheel_coef_of_fric * drive_axle_weight_frac * veh_kg * props.a_grav_mps2 /
            (1.0 + veh_cg_m * wheel_coef_of_fric / wheel_base_m)
        ) / (veh_kg * props.a_grav_mps2)  * props.a_grav_mps2;
        let mc_perc_out_array = Array::linspace(0.0, 1.0, 101);
        RustVehicle {
            scenario_name,
            selection,
            veh_year,
            veh_pt_type,
            drag_coef,
            frontal_area_m2,
            glider_kg,
            veh_cg_m,
            drive_axle_weight_frac,
            wheel_base_m,
            cargo_kg,
            veh_override_kg,
            comp_mass_multiplier,
            max_fuel_stor_kw,
            fuel_stor_secs_to_peak_pwr,
            fuel_stor_kwh,
            fuel_stor_kwh_per_kg,
            max_fuel_conv_kw,
            fc_pwr_out_perc,
            fc_eff_map,
            fc_eff_type,
            fuel_conv_secs_to_peak_pwr,
            fuel_conv_base_kg,
            fuel_conv_kw_per_kg,
            min_fc_time_on,
            idle_fc_kw,
            max_motor_kw,
            mc_pwr_out_perc,
            mc_eff_map,
            motor_secs_to_peak_pwr,
            mc_pe_kg_per_kw,
            mc_pe_base_kg,
            max_ess_kw,
            max_ess_kwh,
            ess_kg_per_kwh,
            ess_base_kg,
            ess_round_trip_eff,
            ess_life_coef_a,
            ess_life_coef_b,
            min_soc,
            max_soc,
            ess_dischg_to_fc_max_eff_perc,
            ess_chg_to_fc_max_eff_perc,
            wheel_inertia_kg_m2,
            num_wheels,
            wheel_rr_coef,
            wheel_radius_m,
            wheel_coef_of_fric,
            max_accel_buffer_mph,
            max_accel_buffer_perc_of_useable_soc,
            perc_high_acc_buf,
            mph_fc_on,
            kw_demand_fc_on,
            max_regen,
            stop_start,
            force_aux_on_fc,
            alt_eff,
            chg_eff,
            aux_kw,
            trans_kg,
            trans_eff,
            ess_to_fuel_ok_error,
            val_udds_mpgge,
            val_hwy_mpgge,
            val_comb_mpgge,
            val_udds_kwh_per_mile,
            val_hwy_kwh_per_mile,
            val_comb_kwh_per_mile,
            val_cd_range_mi,
            val_const65_mph_kwh_per_mile,
            val_const60_mph_kwh_per_mile,
            val_const55_mph_kwh_per_mile,
            val_const45_mph_kwh_per_mile,
            val_unadj_udds_kwh_per_mile,
            val_unadj_hwy_kwh_per_mile,
            val0_to60_mph,
            val_ess_life_miles,
            val_range_miles,
            val_veh_base_cost,
            val_msrp,
            props,
            large_baseline_eff,
            small_baseline_eff,
            small_motor_power_kw,
            large_motor_power_kw,
            fc_perc_out_array,
            regen_a,
            regen_b,
            charging_on,
            no_elec_sys,
            no_elec_aux,
            max_roadway_chg_kw,
            input_kw_out_array,
            fc_kw_out_array,
            fc_eff_array,
            modern_max,
            mc_eff_array,
            mc_kw_in_array,
            mc_kw_out_array,
            mc_max_elec_in_kw,
            mc_full_eff_array,
            veh_kg,
            max_trac_mps2,
            ess_mass_kg,
            mc_mass_kg,
            fc_mass_kg,
            fs_mass_kg,
            mc_perc_out_array,
        }
    }

    #[getter]
    pub fn get_alt_eff(&self) -> PyResult<f64>{
      Ok(self.alt_eff)
    }
    #[setter]
    pub fn set_alt_eff(&mut self, new_value:f64) -> PyResult<()>{
      self.alt_eff = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_aux_kw(&self) -> PyResult<f64>{
      Ok(self.aux_kw)
    }
    #[setter]
    pub fn set_aux_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.aux_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_cargo_kg(&self) -> PyResult<f64>{
      Ok(self.cargo_kg)
    }
    #[setter]
    pub fn set_cargo_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.cargo_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_charging_on(&self) -> PyResult<bool>{
      Ok(self.charging_on)
    }
    #[setter]
    pub fn set_charging_on(&mut self, new_value:bool) -> PyResult<()>{
      self.charging_on = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_chg_eff(&self) -> PyResult<f64>{
      Ok(self.chg_eff)
    }
    #[setter]
    pub fn set_chg_eff(&mut self, new_value:f64) -> PyResult<()>{
      self.chg_eff = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_comp_mass_multiplier(&self) -> PyResult<f64>{
      Ok(self.comp_mass_multiplier)
    }
    #[setter]
    pub fn set_comp_mass_multiplier(&mut self, new_value:f64) -> PyResult<()>{
      self.comp_mass_multiplier = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_drag_coef(&self) -> PyResult<f64>{
      Ok(self.drag_coef)
    }
    #[setter]
    pub fn set_drag_coef(&mut self, new_value:f64) -> PyResult<()>{
      self.drag_coef = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_drive_axle_weight_frac(&self) -> PyResult<f64>{
      Ok(self.drive_axle_weight_frac)
    }
    #[setter]
    pub fn set_drive_axle_weight_frac(&mut self, new_value:f64) -> PyResult<()>{
      self.drive_axle_weight_frac = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_base_kg(&self) -> PyResult<f64>{
      Ok(self.ess_base_kg)
    }
    #[setter]
    pub fn set_ess_base_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_base_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_chg_to_fc_max_eff_perc(&self) -> PyResult<f64>{
      Ok(self.ess_chg_to_fc_max_eff_perc)
    }
    #[setter]
    pub fn set_ess_chg_to_fc_max_eff_perc(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_chg_to_fc_max_eff_perc = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_dischg_to_fc_max_eff_perc(&self) -> PyResult<f64>{
      Ok(self.ess_dischg_to_fc_max_eff_perc)
    }
    #[setter]
    pub fn set_ess_dischg_to_fc_max_eff_perc(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_dischg_to_fc_max_eff_perc = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_kg_per_kwh(&self) -> PyResult<f64>{
      Ok(self.ess_kg_per_kwh)
    }
    #[setter]
    pub fn set_ess_kg_per_kwh(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_kg_per_kwh = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_life_coef_a(&self) -> PyResult<f64>{
      Ok(self.ess_life_coef_a)
    }
    #[setter]
    pub fn set_ess_life_coef_a(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_life_coef_a = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_life_coef_b(&self) -> PyResult<f64>{
      Ok(self.ess_life_coef_b)
    }
    #[setter]
    pub fn set_ess_life_coef_b(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_life_coef_b = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_mass_kg(&self) -> PyResult<f64>{
      Ok(self.ess_mass_kg)
    }
    #[setter]
    pub fn set_ess_mass_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_mass_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_round_trip_eff(&self) -> PyResult<f64>{
      Ok(self.ess_round_trip_eff)
    }
    #[setter]
    pub fn set_ess_round_trip_eff(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_round_trip_eff = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_ess_to_fuel_ok_error(&self) -> PyResult<f64>{
      Ok(self.ess_to_fuel_ok_error)
    }
    #[setter]
    pub fn set_ess_to_fuel_ok_error(&mut self, new_value:f64) -> PyResult<()>{
      self.ess_to_fuel_ok_error = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fc_eff_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_eff_array.to_vec())
    }
    #[setter]
    pub fn set_fc_eff_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_eff_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_eff_map(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_eff_map.to_vec())
    }
    #[setter]
    pub fn set_fc_eff_map(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_eff_map = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_eff_type(&self) -> PyResult<String>{
      Ok(self.fc_eff_type.clone())
    }
    #[setter]
    pub fn set_fc_eff_type(&mut self, new_value:String) -> PyResult<()>{
      self.fc_eff_type = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fc_kw_out_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_kw_out_array.to_vec())
    }
    #[setter]
    pub fn set_fc_kw_out_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_kw_out_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_mass_kg(&self) -> PyResult<f64>{
      Ok(self.fc_mass_kg)
    }
    #[setter]
    pub fn set_fc_mass_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.fc_mass_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fc_perc_out_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_perc_out_array.to_vec())
    }
    #[setter]
    pub fn set_fc_perc_out_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_perc_out_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_pwr_out_perc(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_pwr_out_perc.to_vec())
    }
    #[setter]
    pub fn set_fc_pwr_out_perc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_pwr_out_perc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_force_aux_on_fc(&self) -> PyResult<f64>{
      Ok(self.force_aux_on_fc)
    }
    #[setter]
    pub fn set_force_aux_on_fc(&mut self, new_value:f64) -> PyResult<()>{
      self.force_aux_on_fc = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_frontal_area_m2(&self) -> PyResult<f64>{
      Ok(self.frontal_area_m2)
    }
    #[setter]
    pub fn set_frontal_area_m2(&mut self, new_value:f64) -> PyResult<()>{
      self.frontal_area_m2 = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fs_mass_kg(&self) -> PyResult<f64>{
      Ok(self.fs_mass_kg)
    }
    #[setter]
    pub fn set_fs_mass_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.fs_mass_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fuel_conv_base_kg(&self) -> PyResult<f64>{
      Ok(self.fuel_conv_base_kg)
    }
    #[setter]
    pub fn set_fuel_conv_base_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.fuel_conv_base_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fuel_conv_kw_per_kg(&self) -> PyResult<f64>{
      Ok(self.fuel_conv_kw_per_kg)
    }
    #[setter]
    pub fn set_fuel_conv_kw_per_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.fuel_conv_kw_per_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fuel_conv_secs_to_peak_pwr(&self) -> PyResult<f64>{
      Ok(self.fuel_conv_secs_to_peak_pwr)
    }
    #[setter]
    pub fn set_fuel_conv_secs_to_peak_pwr(&mut self, new_value:f64) -> PyResult<()>{
      self.fuel_conv_secs_to_peak_pwr = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fuel_stor_kwh(&self) -> PyResult<f64>{
      Ok(self.fuel_stor_kwh)
    }
    #[setter]
    pub fn set_fuel_stor_kwh(&mut self, new_value:f64) -> PyResult<()>{
      self.fuel_stor_kwh = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fuel_stor_kwh_per_kg(&self) -> PyResult<f64>{
      Ok(self.fuel_stor_kwh_per_kg)
    }
    #[setter]
    pub fn set_fuel_stor_kwh_per_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.fuel_stor_kwh_per_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_fuel_stor_secs_to_peak_pwr(&self) -> PyResult<f64>{
      Ok(self.fuel_stor_secs_to_peak_pwr)
    }
    #[setter]
    pub fn set_fuel_stor_secs_to_peak_pwr(&mut self, new_value:f64) -> PyResult<()>{
      self.fuel_stor_secs_to_peak_pwr = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_glider_kg(&self) -> PyResult<f64>{
      Ok(self.glider_kg)
    }
    #[setter]
    pub fn set_glider_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.glider_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_idle_fc_kw(&self) -> PyResult<f64>{
      Ok(self.idle_fc_kw)
    }
    #[setter]
    pub fn set_idle_fc_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.idle_fc_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_input_kw_out_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.input_kw_out_array.to_vec())
    }
    #[setter]
    pub fn set_input_kw_out_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.input_kw_out_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_kw_demand_fc_on(&self) -> PyResult<f64>{
      Ok(self.kw_demand_fc_on)
    }
    #[setter]
    pub fn set_kw_demand_fc_on(&mut self, new_value:f64) -> PyResult<()>{
      self.kw_demand_fc_on = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_large_baseline_eff(&self) -> PyResult<Vec<f64>>{
      Ok(self.large_baseline_eff.to_vec())
    }
    #[setter]
    pub fn set_large_baseline_eff(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.large_baseline_eff = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_large_motor_power_kw(&self) -> PyResult<f64>{
      Ok(self.large_motor_power_kw)
    }
    #[setter]
    pub fn set_large_motor_power_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.large_motor_power_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_accel_buffer_mph(&self) -> PyResult<f64>{
      Ok(self.max_accel_buffer_mph)
    }
    #[setter]
    pub fn set_max_accel_buffer_mph(&mut self, new_value:f64) -> PyResult<()>{
      self.max_accel_buffer_mph = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_accel_buffer_perc_of_useable_soc(&self) -> PyResult<f64>{
      Ok(self.max_accel_buffer_perc_of_useable_soc)
    }
    #[setter]
    pub fn set_max_accel_buffer_perc_of_useable_soc(&mut self, new_value:f64) -> PyResult<()>{
      self.max_accel_buffer_perc_of_useable_soc = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_ess_kw(&self) -> PyResult<f64>{
      Ok(self.max_ess_kw)
    }
    #[setter]
    pub fn set_max_ess_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.max_ess_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_ess_kwh(&self) -> PyResult<f64>{
      Ok(self.max_ess_kwh)
    }
    #[setter]
    pub fn set_max_ess_kwh(&mut self, new_value:f64) -> PyResult<()>{
      self.max_ess_kwh = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_fuel_conv_kw(&self) -> PyResult<f64>{
      Ok(self.max_fuel_conv_kw)
    }
    #[setter]
    pub fn set_max_fuel_conv_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.max_fuel_conv_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_fuel_stor_kw(&self) -> PyResult<f64>{
      Ok(self.max_fuel_stor_kw)
    }
    #[setter]
    pub fn set_max_fuel_stor_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.max_fuel_stor_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_motor_kw(&self) -> PyResult<f64>{
      Ok(self.max_motor_kw)
    }
    #[setter]
    pub fn set_max_motor_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.max_motor_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_regen(&self) -> PyResult<f64>{
      Ok(self.max_regen)
    }
    #[setter]
    pub fn set_max_regen(&mut self, new_value:f64) -> PyResult<()>{
      self.max_regen = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_roadway_chg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.max_roadway_chg_kw.to_vec())
    }
    #[setter]
    pub fn set_max_roadway_chg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.max_roadway_chg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_max_soc(&self) -> PyResult<f64>{
      Ok(self.max_soc)
    }
    #[setter]
    pub fn set_max_soc(&mut self, new_value:f64) -> PyResult<()>{
      self.max_soc = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_trac_mps2(&self) -> PyResult<f64>{
      Ok(self.max_trac_mps2)
    }
    #[setter]
    pub fn set_max_trac_mps2(&mut self, new_value:f64) -> PyResult<()>{
      self.max_trac_mps2 = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_mc_eff_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_eff_array.to_vec())
    }
    #[setter]
    pub fn set_mc_eff_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_eff_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_eff_map(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_eff_map.to_vec())
    }
    #[setter]
    pub fn set_mc_eff_map(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_eff_map = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_full_eff_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_full_eff_array.to_vec())
    }
    #[setter]
    pub fn set_mc_full_eff_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_full_eff_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_kw_in_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_kw_in_array.to_vec())
    }
    #[setter]
    pub fn set_mc_kw_in_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_kw_in_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_kw_out_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_kw_out_array.to_vec())
    }
    #[setter]
    pub fn set_mc_kw_out_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_kw_out_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_mass_kg(&self) -> PyResult<f64>{
      Ok(self.mc_mass_kg)
    }
    #[setter]
    pub fn set_mc_mass_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.mc_mass_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_mc_max_elec_in_kw(&self) -> PyResult<f64>{
      Ok(self.mc_max_elec_in_kw)
    }
    #[setter]
    pub fn set_mc_max_elec_in_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.mc_max_elec_in_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_mc_pe_base_kg(&self) -> PyResult<f64>{
      Ok(self.mc_pe_base_kg)
    }
    #[setter]
    pub fn set_mc_pe_base_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.mc_pe_base_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_mc_pe_kg_per_kw(&self) -> PyResult<f64>{
      Ok(self.mc_pe_kg_per_kw)
    }
    #[setter]
    pub fn set_mc_pe_kg_per_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.mc_pe_kg_per_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_mc_perc_out_array(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_perc_out_array.to_vec())
    }
    #[setter]
    pub fn set_mc_perc_out_array(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_perc_out_array = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_pwr_out_perc(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_pwr_out_perc.to_vec())
    }
    #[setter]
    pub fn set_mc_pwr_out_perc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_pwr_out_perc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_min_fc_time_on(&self) -> PyResult<f64>{
      Ok(self.min_fc_time_on)
    }
    #[setter]
    pub fn set_min_fc_time_on(&mut self, new_value:f64) -> PyResult<()>{
      self.min_fc_time_on = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_min_soc(&self) -> PyResult<f64>{
      Ok(self.min_soc)
    }
    #[setter]
    pub fn set_min_soc(&mut self, new_value:f64) -> PyResult<()>{
      self.min_soc = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_modern_max(&self) -> PyResult<f64>{
      Ok(self.modern_max)
    }
    #[setter]
    pub fn set_modern_max(&mut self, new_value:f64) -> PyResult<()>{
      self.modern_max = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_motor_secs_to_peak_pwr(&self) -> PyResult<f64>{
      Ok(self.motor_secs_to_peak_pwr)
    }
    #[setter]
    pub fn set_motor_secs_to_peak_pwr(&mut self, new_value:f64) -> PyResult<()>{
      self.motor_secs_to_peak_pwr = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_mph_fc_on(&self) -> PyResult<f64>{
      Ok(self.mph_fc_on)
    }
    #[setter]
    pub fn set_mph_fc_on(&mut self, new_value:f64) -> PyResult<()>{
      self.mph_fc_on = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_no_elec_aux(&self) -> PyResult<bool>{
      Ok(self.no_elec_aux)
    }
    #[setter]
    pub fn set_no_elec_aux(&mut self, new_value:bool) -> PyResult<()>{
      self.no_elec_aux = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_no_elec_sys(&self) -> PyResult<bool>{
      Ok(self.no_elec_sys)
    }
    #[setter]
    pub fn set_no_elec_sys(&mut self, new_value:bool) -> PyResult<()>{
      self.no_elec_sys = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_num_wheels(&self) -> PyResult<f64>{
      Ok(self.num_wheels)
    }
    #[setter]
    pub fn set_num_wheels(&mut self, new_value:f64) -> PyResult<()>{
      self.num_wheels = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_perc_high_acc_buf(&self) -> PyResult<f64>{
      Ok(self.perc_high_acc_buf)
    }
    #[setter]
    pub fn set_perc_high_acc_buf(&mut self, new_value:f64) -> PyResult<()>{
      self.perc_high_acc_buf = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_props(&self) -> PyResult<RustPhysicalProperties>{
      let new_props = RustPhysicalProperties{
        air_density_kg_per_m3: self.props.air_density_kg_per_m3,
        a_grav_mps2: self.props.a_grav_mps2,
        kwh_per_gge: self.props.kwh_per_gge,
        fuel_rho_kg__L: self.props.fuel_rho_kg__L,
        fuel_afr_stoich: self.props.fuel_afr_stoich,
      };
      Ok(new_props)
    }
    #[setter]
    pub fn set_props(&mut self, new_value:RustPhysicalProperties) -> PyResult<()>{
      self.props = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_regen_a(&self) -> PyResult<f64>{
      Ok(self.regen_a)
    }
    #[setter]
    pub fn set_regen_a(&mut self, new_value:f64) -> PyResult<()>{
      self.regen_a = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_regen_b(&self) -> PyResult<f64>{
      Ok(self.regen_b)
    }
    #[setter]
    pub fn set_regen_b(&mut self, new_value:f64) -> PyResult<()>{
      self.regen_b = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_scenario_name(&self) -> PyResult<String>{
      Ok(self.scenario_name.clone())
    }
    #[setter]
    pub fn set_scenario_name(&mut self, new_value:String) -> PyResult<()>{
      self.scenario_name = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_selection(&self) -> PyResult<u32>{
      Ok(self.selection)
    }
    #[setter]
    pub fn set_selection(&mut self, new_value:u32) -> PyResult<()>{
      self.selection = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_small_baseline_eff(&self) -> PyResult<Vec<f64>>{
      Ok(self.small_baseline_eff.to_vec())
    }
    #[setter]
    pub fn set_small_baseline_eff(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.small_baseline_eff = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_small_motor_power_kw(&self) -> PyResult<f64>{
      Ok(self.small_motor_power_kw)
    }
    #[setter]
    pub fn set_small_motor_power_kw(&mut self, new_value:f64) -> PyResult<()>{
      self.small_motor_power_kw = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_stop_start(&self) -> PyResult<bool>{
      Ok(self.stop_start)
    }
    #[setter]
    pub fn set_stop_start(&mut self, new_value:bool) -> PyResult<()>{
      self.stop_start = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_trans_eff(&self) -> PyResult<f64>{
      Ok(self.trans_eff)
    }
    #[setter]
    pub fn set_trans_eff(&mut self, new_value:f64) -> PyResult<()>{
      self.trans_eff = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_trans_kg(&self) -> PyResult<f64>{
      Ok(self.trans_kg)
    }
    #[setter]
    pub fn set_trans_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.trans_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val0_to60_mph(&self) -> PyResult<f64>{
      Ok(self.val0_to60_mph)
    }
    #[setter]
    pub fn set_val0_to60_mph(&mut self, new_value:f64) -> PyResult<()>{
      self.val0_to60_mph = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_cd_range_mi(&self) -> PyResult<f64>{
      Ok(self.val_cd_range_mi)
    }
    #[setter]
    pub fn set_val_cd_range_mi(&mut self, new_value:f64) -> PyResult<()>{
      self.val_cd_range_mi = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_comb_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_comb_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_comb_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_comb_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_comb_mpgge(&self) -> PyResult<f64>{
      Ok(self.val_comb_mpgge)
    }
    #[setter]
    pub fn set_val_comb_mpgge(&mut self, new_value:f64) -> PyResult<()>{
      self.val_comb_mpgge = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_const45_mph_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_const45_mph_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_const45_mph_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_const45_mph_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_const55_mph_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_const55_mph_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_const55_mph_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_const55_mph_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_const60_mph_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_const60_mph_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_const60_mph_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_const60_mph_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_const65_mph_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_const65_mph_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_const65_mph_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_const65_mph_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_ess_life_miles(&self) -> PyResult<f64>{
      Ok(self.val_ess_life_miles)
    }
    #[setter]
    pub fn set_val_ess_life_miles(&mut self, new_value:f64) -> PyResult<()>{
      self.val_ess_life_miles = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_hwy_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_hwy_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_hwy_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_hwy_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_hwy_mpgge(&self) -> PyResult<f64>{
      Ok(self.val_hwy_mpgge)
    }
    #[setter]
    pub fn set_val_hwy_mpgge(&mut self, new_value:f64) -> PyResult<()>{
      self.val_hwy_mpgge = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_msrp(&self) -> PyResult<f64>{
      Ok(self.val_msrp)
    }
    #[setter]
    pub fn set_val_msrp(&mut self, new_value:f64) -> PyResult<()>{
      self.val_msrp = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_range_miles(&self) -> PyResult<f64>{
      Ok(self.val_range_miles)
    }
    #[setter]
    pub fn set_val_range_miles(&mut self, new_value:f64) -> PyResult<()>{
      self.val_range_miles = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_udds_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_udds_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_udds_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_udds_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_udds_mpgge(&self) -> PyResult<f64>{
      Ok(self.val_udds_mpgge)
    }
    #[setter]
    pub fn set_val_udds_mpgge(&mut self, new_value:f64) -> PyResult<()>{
      self.val_udds_mpgge = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_unadj_hwy_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_unadj_hwy_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_unadj_hwy_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_unadj_hwy_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_unadj_udds_kwh_per_mile(&self) -> PyResult<f64>{
      Ok(self.val_unadj_udds_kwh_per_mile)
    }
    #[setter]
    pub fn set_val_unadj_udds_kwh_per_mile(&mut self, new_value:f64) -> PyResult<()>{
      self.val_unadj_udds_kwh_per_mile = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_val_veh_base_cost(&self) -> PyResult<f64>{
      Ok(self.val_veh_base_cost)
    }
    #[setter]
    pub fn set_val_veh_base_cost(&mut self, new_value:f64) -> PyResult<()>{
      self.val_veh_base_cost = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_veh_cg_m(&self) -> PyResult<f64>{
      Ok(self.veh_cg_m)
    }
    #[setter]
    pub fn set_veh_cg_m(&mut self, new_value:f64) -> PyResult<()>{
      self.veh_cg_m = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_veh_kg(&self) -> PyResult<f64>{
      Ok(self.veh_kg)
    }
    #[setter]
    pub fn set_veh_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.veh_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_veh_override_kg(&self) -> PyResult<f64>{
      Ok(self.veh_override_kg)
    }
    #[setter]
    pub fn set_veh_override_kg(&mut self, new_value:f64) -> PyResult<()>{
      self.veh_override_kg = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_veh_pt_type(&self) -> PyResult<String>{
      Ok(self.veh_pt_type.clone())
    }
    #[setter]
    pub fn set_veh_pt_type(&mut self, new_value:String) -> PyResult<()>{
      self.veh_pt_type = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_veh_year(&self) -> PyResult<u32>{
      Ok(self.veh_year)
    }
    #[setter]
    pub fn set_veh_year(&mut self, new_value:u32) -> PyResult<()>{
      self.veh_year = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_wheel_base_m(&self) -> PyResult<f64>{
      Ok(self.wheel_base_m)
    }
    #[setter]
    pub fn set_wheel_base_m(&mut self, new_value:f64) -> PyResult<()>{
      self.wheel_base_m = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_wheel_coef_of_fric(&self) -> PyResult<f64>{
      Ok(self.wheel_coef_of_fric)
    }
    #[setter]
    pub fn set_wheel_coef_of_fric(&mut self, new_value:f64) -> PyResult<()>{
      self.wheel_coef_of_fric = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_wheel_inertia_kg_m2(&self) -> PyResult<f64>{
      Ok(self.wheel_inertia_kg_m2)
    }
    #[setter]
    pub fn set_wheel_inertia_kg_m2(&mut self, new_value:f64) -> PyResult<()>{
      self.wheel_inertia_kg_m2 = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_wheel_radius_m(&self) -> PyResult<f64>{
      Ok(self.wheel_radius_m)
    }
    #[setter]
    pub fn set_wheel_radius_m(&mut self, new_value:f64) -> PyResult<()>{
      self.wheel_radius_m = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_wheel_rr_coef(&self) -> PyResult<f64>{
      Ok(self.wheel_rr_coef)
    }
    #[setter]
    pub fn set_wheel_rr_coef(&mut self, new_value:f64) -> PyResult<()>{
      self.wheel_rr_coef = new_value;
      Ok(())
    }
}
