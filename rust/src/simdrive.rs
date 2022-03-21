extern crate ndarray;
use ndarray::{Array, Array1, array, s};
extern crate pyo3;
use pyo3::prelude::*;
use std::cmp;

use super::utils::*;
use super::params::RustPhysicalProperties;
use super::params;
use super::vehicle::*;
use super::cycle::RustCycle;


#[pyclass]
#[derive(Debug, Clone)]
/// Struct containing time trace data
pub struct RustSimDriveParams{
    pub missed_trace_correction: bool, // if true, missed trace correction is active, default = false
    pub max_time_dilation: f64,
    pub min_time_dilation: f64,
    pub time_dilation_tol: f64,
    pub max_trace_miss_iters: u32,
    pub trace_miss_speed_mps_tol: f64,
    pub trace_miss_time_tol: f64,
    pub trace_miss_dist_tol: f64,
    pub sim_count_max: u32,
    pub verbose: bool,
    pub newton_gain: f64,
    pub newton_max_iter: u32,
    pub newton_xtol: f64,
    pub energy_audit_error_tol: f64,
    pub max_epa_adj: f64,
}

#[pymethods]
impl RustSimDriveParams{
    #[new]
    pub fn __new__() -> Self{
        // if true, missed trace correction is active, default = false
        let missed_trace_correction = false;
        // maximum time dilation factor to "catch up" with trace -- e.g. 1.0 means 100% increase in step size
        let max_time_dilation: f64 = 1.0;
        // minimum time dilation margin to let trace "catch up" -- e.g. -0.5 means 50% reduction in step size
        let min_time_dilation: f64 = -0.5;
        let time_dilation_tol: f64 = 5e-4; // convergence criteria for time dilation
        let max_trace_miss_iters: u32 = 5; // number of iterations to achieve time dilation correction
        let trace_miss_speed_mps_tol: f64 = 1.0; // # threshold of error in speed [m/s] that triggers warning
        let trace_miss_time_tol: f64 = 1e-3; // threshold for printing warning when time dilation is active
        let trace_miss_dist_tol: f64 = 1e-3; // threshold of fractional eror in distance that triggers warning
        let sim_count_max: u32 = 30; // max allowable number of HEV SOC iterations
        let verbose = true; // show warning and other messages
        let newton_gain: f64 = 0.9; // newton solver gain
        let newton_max_iter: u32 = 100; // newton solver max iterations
        let newton_xtol: f64 = 1e-9; // newton solver tolerance
        let energy_audit_error_tol: f64 = 0.002; // tolerance for energy audit error warning, i.e. 0.1%
        // EPA fuel economy adjustment parameters
        let max_epa_adj: f64 = 0.3; // maximum EPA adjustment factor
        RustSimDriveParams{
            missed_trace_correction,
            max_time_dilation,
            min_time_dilation,
            time_dilation_tol,
            max_trace_miss_iters,
            trace_miss_speed_mps_tol,
            trace_miss_time_tol,
            trace_miss_dist_tol,
            sim_count_max,
            verbose,
            newton_gain,
            newton_max_iter,
            newton_xtol,
            energy_audit_error_tol,
            max_epa_adj,
        }
    }

    #[getter]
    pub fn get_energy_audit_error_tol(&self) -> PyResult<f64>{
      Ok(self.energy_audit_error_tol)
    }
    #[setter]
    pub fn set_energy_audit_error_tol(&mut self, new_value:f64) -> PyResult<()>{
      self.energy_audit_error_tol = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_epa_adj(&self) -> PyResult<f64>{
      Ok(self.max_epa_adj)
    }
    #[setter]
    pub fn set_max_epa_adj(&mut self, new_value:f64) -> PyResult<()>{
      self.max_epa_adj = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_time_dilation(&self) -> PyResult<f64>{
      Ok(self.max_time_dilation)
    }
    #[setter]
    pub fn set_max_time_dilation(&mut self, new_value:f64) -> PyResult<()>{
      self.max_time_dilation = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_trace_miss_iters(&self) -> PyResult<u32>{
      Ok(self.max_trace_miss_iters)
    }
    #[setter]
    pub fn set_max_trace_miss_iters(&mut self, new_value:u32) -> PyResult<()>{
      self.max_trace_miss_iters = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_min_time_dilation(&self) -> PyResult<f64>{
      Ok(self.min_time_dilation)
    }
    #[setter]
    pub fn set_min_time_dilation(&mut self, new_value:f64) -> PyResult<()>{
      self.min_time_dilation = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_missed_trace_correction(&self) -> PyResult<bool>{
      Ok(self.missed_trace_correction)
    }
    #[setter]
    pub fn set_missed_trace_correction(&mut self, new_value:bool) -> PyResult<()>{
      self.missed_trace_correction = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_newton_gain(&self) -> PyResult<f64>{
      Ok(self.newton_gain)
    }
    #[setter]
    pub fn set_newton_gain(&mut self, new_value:f64) -> PyResult<()>{
      self.newton_gain = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_newton_max_iter(&self) -> PyResult<u32>{
      Ok(self.newton_max_iter)
    }
    #[setter]
    pub fn set_newton_max_iter(&mut self, new_value:u32) -> PyResult<()>{
      self.newton_max_iter = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_newton_xtol(&self) -> PyResult<f64>{
      Ok(self.newton_xtol)
    }
    #[setter]
    pub fn set_newton_xtol(&mut self, new_value:f64) -> PyResult<()>{
      self.newton_xtol = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_sim_count_max(&self) -> PyResult<u32>{
      Ok(self.sim_count_max)
    }
    #[setter]
    pub fn set_sim_count_max(&mut self, new_value:u32) -> PyResult<()>{
      self.sim_count_max = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_time_dilation_tol(&self) -> PyResult<f64>{
      Ok(self.time_dilation_tol)
    }
    #[setter]
    pub fn set_time_dilation_tol(&mut self, new_value:f64) -> PyResult<()>{
      self.time_dilation_tol = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_trace_miss_dist_tol(&self) -> PyResult<f64>{
      Ok(self.trace_miss_dist_tol)
    }
    #[setter]
    pub fn set_trace_miss_dist_tol(&mut self, new_value:f64) -> PyResult<()>{
      self.trace_miss_dist_tol = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_trace_miss_speed_mps_tol(&self) -> PyResult<f64>{
      Ok(self.trace_miss_speed_mps_tol)
    }
    #[setter]
    pub fn set_trace_miss_speed_mps_tol(&mut self, new_value:f64) -> PyResult<()>{
      self.trace_miss_speed_mps_tol = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_trace_miss_time_tol(&self) -> PyResult<f64>{
      Ok(self.trace_miss_time_tol)
    }
    #[setter]
    pub fn set_trace_miss_time_tol(&mut self, new_value:f64) -> PyResult<()>{
      self.trace_miss_time_tol = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_verbose(&self) -> PyResult<bool>{
      Ok(self.verbose)
    }
    #[setter]
    pub fn set_verbose(&mut self, new_value:bool) -> PyResult<()>{
      self.verbose = new_value;
      Ok(())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RustSimDrive{
    #[pyo3(get, set)]
    pub hev_sim_count: usize,
    pub veh: RustVehicle,
    pub cyc: RustCycle,
    pub cyc0: RustCycle,
    pub sim_params: RustSimDriveParams,
    pub props: RustPhysicalProperties,
    #[pyo3(get, set)]
    pub i: usize, // 1 # initialize step counter for possible use outside sim_drive_walk()
    pub cur_max_fs_kw_out: Array1<f64>,
    pub fc_trans_lim_kw: Array1<f64>,
    pub fc_fs_lim_kw: Array1<f64>,
    pub fc_max_kw_in: Array1<f64>,
    pub cur_max_fc_kw_out: Array1<f64>,
    pub ess_cap_lim_dischg_kw: Array1<f64>,
    pub cur_max_ess_kw_out: Array1<f64>,
    pub cur_max_avail_elec_kw: Array1<f64>,
    pub ess_cap_lim_chg_kw: Array1<f64>,
    pub cur_max_ess_chg_kw: Array1<f64>,
    pub cur_max_elec_kw: Array1<f64>,
    pub mc_elec_in_lim_kw: Array1<f64>,
    pub mc_transi_lim_kw: Array1<f64>,
    pub cur_max_mc_kw_out: Array1<f64>,
    pub ess_lim_mc_regen_perc_kw: Array1<f64>,
    pub ess_lim_mc_regen_kw: Array1<f64>,
    pub cur_max_mech_mc_kw_in: Array1<f64>,
    pub cur_max_trans_kw_out: Array1<f64>,
    pub cyc_drag_kw: Array1<f64>,
    pub cyc_accel_kw: Array1<f64>,
    pub cyc_ascent_kw: Array1<f64>,
    pub cyc_trac_kw_req: Array1<f64>,
    pub cur_max_trac_kw: Array1<f64>,
    pub spare_trac_kw: Array1<f64>,
    pub cyc_rr_kw: Array1<f64>,
    pub cyc_whl_rad_per_sec: Array1<f64>,
    pub cyc_tire_inertia_kw: Array1<f64>,
    pub cyc_whl_kw_req: Array1<f64>,
    pub regen_contrl_lim_kw_perc: Array1<f64>,
    pub cyc_regen_brake_kw: Array1<f64>,
    pub cyc_fric_brake_kw: Array1<f64>,
    pub cyc_trans_kw_out_req: Array1<f64>,
    pub cyc_met: Array1<bool>,
    pub trans_kw_out_ach: Array1<f64>,
    pub trans_kw_in_ach: Array1<f64>,
    pub cur_soc_target: Array1<f64>,
    pub min_mc_kw_2help_fc: Array1<f64>,
    pub mc_mech_kw_out_ach: Array1<f64>,
    pub mc_elec_kw_in_ach: Array1<f64>,
    pub aux_in_kw: Array1<f64>,
    pub roadway_chg_kw_out_ach: Array1<f64>,
    pub min_ess_kw_2help_fc: Array1<f64>,
    pub ess_kw_out_ach: Array1<f64>,
    pub fc_kw_out_ach: Array1<f64>,
    pub fc_kw_out_ach_pct: Array1<f64>,
    pub fc_kw_in_ach: Array1<f64>,
    pub fs_kw_out_ach: Array1<f64>,
    pub fs_cumu_mj_out_ach: Array1<f64>,
    pub fs_kwh_out_ach: Array1<f64>,
    pub ess_cur_kwh: Array1<f64>,
    pub soc: Array1<f64>,
    pub regen_buff_soc: Array1<f64>,
    pub ess_regen_buff_dischg_kw: Array1<f64>,
    pub max_ess_regen_buff_chg_kw: Array1<f64>,
    pub ess_accel_buff_chg_kw: Array1<f64>,
    pub accel_buff_soc: Array1<f64>,
    pub max_ess_accell_buff_dischg_kw: Array1<f64>,
    pub ess_accel_regen_dischg_kw: Array1<f64>,
    pub mc_elec_in_kw_for_max_fc_eff: Array1<f64>,
    pub elec_kw_req_4ae: Array1<f64>,
    pub can_pwr_all_elec: Array1<bool>,
    pub desired_ess_kw_out_for_ae: Array1<f64>,
    pub ess_ae_kw_out: Array1<f64>,
    pub er_ae_kw_out: Array1<f64>,
    pub ess_desired_kw_4fc_eff: Array1<f64>,
    pub ess_kw_if_fc_req: Array1<f64>,
    pub cur_max_mc_elec_kw_in: Array1<f64>,
    pub fc_kw_gap_fr_eff: Array1<f64>,
    pub er_kw_if_fc_req: Array1<f64>,
    pub mc_elec_kw_in_if_fc_req: Array1<f64>,
    pub mc_kw_if_fc_req: Array1<f64>,
    pub fc_forced_on: Array1<bool>,
    pub fc_forced_state: Array1<u32>,
    pub mc_mech_kw_4forced_fc: Array1<f64>,
    pub fc_time_on: Array1<f64>,
    pub prev_fc_time_on: Array1<f64>,
    pub mps_ach: Array1<f64>,
    pub mph_ach: Array1<f64>,
    pub dist_m: Array1<f64>,
    pub dist_mi: Array1<f64>,
    pub high_acc_fc_on_tag: Array1<bool>,
    pub reached_buff: Array1<bool>,
    pub max_trac_mps: Array1<f64>,
    pub add_kwh: Array1<f64>,
    pub dod_cycs: Array1<f64>,
    pub ess_perc_dead: Array1<f64>,
    pub drag_kw: Array1<f64>,
    pub ess_loss_kw: Array1<f64>,
    pub accel_kw: Array1<f64>,
    pub ascent_kw: Array1<f64>,
    pub rr_kw: Array1<f64>,
    pub cur_max_roadway_chg_kw: Array1<f64>,
    pub trace_miss_iters: Array1<f64>,
    pub newton_iters: Array1<u32>,
    pub fuel_kj: f64,
}

#[pymethods]
impl RustSimDrive{
    /// method for instantiating SimDriveRust
    #[new]
    pub fn __new__(cyc: RustCycle, veh: RustVehicle) -> Self{
        let hev_sim_count: usize = 0;
        let cyc0: RustCycle = cyc.clone();
        let sim_params = RustSimDriveParams::__new__();
        let props = RustPhysicalProperties::__new__();
        let i: usize = 1; // 1 # initialize step counter for possible use outside sim_drive_walk()
        let cyc_len = cyc.time_s.len(); //get_len() as usize;
        let cur_max_fs_kw_out = Array::zeros(cyc_len);
        let fc_trans_lim_kw = Array::zeros(cyc_len);
        let fc_fs_lim_kw = Array::zeros(cyc_len);
        let fc_max_kw_in = Array::zeros(cyc_len);
        let cur_max_fc_kw_out = Array::zeros(cyc_len);
        let ess_cap_lim_dischg_kw = Array::zeros(cyc_len);
        let cur_max_ess_kw_out = Array::zeros(cyc_len);
        let cur_max_avail_elec_kw = Array::zeros(cyc_len);
        let ess_cap_lim_chg_kw = Array::zeros(cyc_len);
        let cur_max_ess_chg_kw = Array::zeros(cyc_len);
        let cur_max_elec_kw = Array::zeros(cyc_len);
        let mc_elec_in_lim_kw = Array::zeros(cyc_len);
        let mc_transi_lim_kw = Array::zeros(cyc_len);
        let cur_max_mc_kw_out = Array::zeros(cyc_len);
        let ess_lim_mc_regen_perc_kw = Array::zeros(cyc_len);
        let ess_lim_mc_regen_kw = Array::zeros(cyc_len);
        let cur_max_mech_mc_kw_in = Array::zeros(cyc_len);
        let cur_max_trans_kw_out = Array::zeros(cyc_len);
        let cyc_drag_kw = Array::zeros(cyc_len);
        let cyc_accel_kw = Array::zeros(cyc_len);
        let cyc_ascent_kw = Array::zeros(cyc_len);
        let cyc_trac_kw_req = Array::zeros(cyc_len);
        let cur_max_trac_kw = Array::zeros(cyc_len);
        let spare_trac_kw = Array::zeros(cyc_len);
        let cyc_rr_kw = Array::zeros(cyc_len);
        let cyc_whl_rad_per_sec = Array::zeros(cyc_len);
        let cyc_tire_inertia_kw = Array::zeros(cyc_len);
        let cyc_whl_kw_req = Array::zeros(cyc_len);
        let regen_contrl_lim_kw_perc = Array::zeros(cyc_len);
        let cyc_regen_brake_kw = Array::zeros(cyc_len);
        let cyc_fric_brake_kw = Array::zeros(cyc_len);
        let cyc_trans_kw_out_req = Array::zeros(cyc_len);
        let cyc_met = Array::from_vec(vec![false; cyc_len]);
        let trans_kw_out_ach = Array::zeros(cyc_len);
        let trans_kw_in_ach = Array::zeros(cyc_len);
        let cur_soc_target = Array::zeros(cyc_len);
        let min_mc_kw_2help_fc = Array::zeros(cyc_len);
        let mc_mech_kw_out_ach = Array::zeros(cyc_len);
        let mc_elec_kw_in_ach = Array::zeros(cyc_len);
        let aux_in_kw = Array::zeros(cyc_len);
        let roadway_chg_kw_out_ach = Array::zeros(cyc_len);
        let min_ess_kw_2help_fc = Array::zeros(cyc_len);
        let ess_kw_out_ach = Array::zeros(cyc_len);
        let fc_kw_out_ach = Array::zeros(cyc_len);
        let fc_kw_out_ach_pct = Array::zeros(cyc_len);
        let fc_kw_in_ach = Array::zeros(cyc_len);
        let fs_kw_out_ach = Array::zeros(cyc_len);
        let fs_cumu_mj_out_ach = Array::zeros(cyc_len);
        let fs_kwh_out_ach = Array::zeros(cyc_len);
        let ess_cur_kwh = Array::zeros(cyc_len);
        let soc = Array::zeros(cyc_len);
        let regen_buff_soc = Array::zeros(cyc_len);
        let ess_regen_buff_dischg_kw = Array::zeros(cyc_len);
        let max_ess_regen_buff_chg_kw = Array::zeros(cyc_len);
        let ess_accel_buff_chg_kw = Array::zeros(cyc_len);
        let accel_buff_soc = Array::zeros(cyc_len);
        let max_ess_accell_buff_dischg_kw = Array::zeros(cyc_len);
        let ess_accel_regen_dischg_kw = Array::zeros(cyc_len);
        let mc_elec_in_kw_for_max_fc_eff = Array::zeros(cyc_len);
        let elec_kw_req_4ae = Array::zeros(cyc_len);
        let can_pwr_all_elec = Array::from_vec(vec![false; cyc_len]);
        let desired_ess_kw_out_for_ae = Array::zeros(cyc_len);
        let ess_ae_kw_out = Array::zeros(cyc_len);
        let er_ae_kw_out = Array::zeros(cyc_len);
        let ess_desired_kw_4fc_eff = Array::zeros(cyc_len);
        let ess_kw_if_fc_req = Array::zeros(cyc_len);
        let cur_max_mc_elec_kw_in = Array::zeros(cyc_len);
        let fc_kw_gap_fr_eff = Array::zeros(cyc_len);
        let er_kw_if_fc_req = Array::zeros(cyc_len);
        let mc_elec_kw_in_if_fc_req = Array::zeros(cyc_len);
        let mc_kw_if_fc_req = Array::zeros(cyc_len);
        let fc_forced_on = Array::from_vec(vec![false; cyc_len]);
        let fc_forced_state = Array::zeros(cyc_len);
        let mc_mech_kw_4forced_fc = Array::zeros(cyc_len);
        let fc_time_on = Array::zeros(cyc_len);
        let prev_fc_time_on = Array::zeros(cyc_len);
        let mps_ach = Array::zeros(cyc_len);
        let mph_ach = Array::zeros(cyc_len);
        let dist_m = Array::zeros(cyc_len);
        let dist_mi = Array::zeros(cyc_len);
        let high_acc_fc_on_tag = Array::from_vec(vec![false; cyc_len]);
        let reached_buff = Array::from_vec(vec![false; cyc_len]);
        let max_trac_mps = Array::zeros(cyc_len);
        let add_kwh = Array::zeros(cyc_len);
        let dod_cycs = Array::zeros(cyc_len);
        let ess_perc_dead = Array::zeros(cyc_len);
        let drag_kw = Array::zeros(cyc_len);
        let ess_loss_kw = Array::zeros(cyc_len);
        let accel_kw = Array::zeros(cyc_len);
        let ascent_kw = Array::zeros(cyc_len);
        let rr_kw = Array::zeros(cyc_len);
        let cur_max_roadway_chg_kw = Array::zeros(cyc_len);
        let trace_miss_iters = Array::zeros(cyc_len);
        let newton_iters = Array::zeros(cyc_len);
        let fuel_kj: f64 = 0.0;
        RustSimDrive{
            hev_sim_count,
            veh,
            cyc,
            cyc0,
            sim_params,
            props,
            i, // 1 # initialize step counter for possible use outside sim_drive_walk()
            cur_max_fs_kw_out,
            fc_trans_lim_kw,
            fc_fs_lim_kw,
            fc_max_kw_in,
            cur_max_fc_kw_out,
            ess_cap_lim_dischg_kw,
            cur_max_ess_kw_out,
            cur_max_avail_elec_kw,
            ess_cap_lim_chg_kw,
            cur_max_ess_chg_kw,
            cur_max_elec_kw,
            mc_elec_in_lim_kw,
            mc_transi_lim_kw,
            cur_max_mc_kw_out,
            ess_lim_mc_regen_perc_kw,
            ess_lim_mc_regen_kw,
            cur_max_mech_mc_kw_in,
            cur_max_trans_kw_out,
            cyc_drag_kw,
            cyc_accel_kw,
            cyc_ascent_kw,
            cyc_trac_kw_req,
            cur_max_trac_kw,
            spare_trac_kw,
            cyc_rr_kw,
            cyc_whl_rad_per_sec,
            cyc_tire_inertia_kw,
            cyc_whl_kw_req,
            regen_contrl_lim_kw_perc,
            cyc_regen_brake_kw,
            cyc_fric_brake_kw,
            cyc_trans_kw_out_req,
            cyc_met,
            trans_kw_out_ach,
            trans_kw_in_ach,
            cur_soc_target,
            min_mc_kw_2help_fc,
            mc_mech_kw_out_ach,
            mc_elec_kw_in_ach,
            aux_in_kw,
            roadway_chg_kw_out_ach,
            min_ess_kw_2help_fc,
            ess_kw_out_ach,
            fc_kw_out_ach,
            fc_kw_out_ach_pct,
            fc_kw_in_ach,
            fs_kw_out_ach,
            fs_cumu_mj_out_ach,
            fs_kwh_out_ach,
            ess_cur_kwh,
            soc,
            regen_buff_soc,
            ess_regen_buff_dischg_kw,
            max_ess_regen_buff_chg_kw,
            ess_accel_buff_chg_kw,
            accel_buff_soc,
            max_ess_accell_buff_dischg_kw,
            ess_accel_regen_dischg_kw,
            mc_elec_in_kw_for_max_fc_eff,
            elec_kw_req_4ae,
            can_pwr_all_elec,
            desired_ess_kw_out_for_ae,
            ess_ae_kw_out,
            er_ae_kw_out,
            ess_desired_kw_4fc_eff,
            ess_kw_if_fc_req,
            cur_max_mc_elec_kw_in,
            fc_kw_gap_fr_eff,
            er_kw_if_fc_req,
            mc_elec_kw_in_if_fc_req,
            mc_kw_if_fc_req,
            fc_forced_on,
            fc_forced_state,
            mc_mech_kw_4forced_fc,
            fc_time_on,
            prev_fc_time_on,
            mps_ach,
            mph_ach,
            dist_m,
            dist_mi,
            high_acc_fc_on_tag,
            reached_buff,
            max_trac_mps,
            add_kwh,
            dod_cycs,
            ess_perc_dead,
            drag_kw,
            ess_loss_kw,
            accel_kw,
            ascent_kw,
            rr_kw,
            cur_max_roadway_chg_kw,
            trace_miss_iters,
            newton_iters,
            fuel_kj,
        }
    }

    /// Receives second-by-second cycle information, vehicle properties,
    /// and an initial state of charge and runs sim_drive_step to perform a
    /// backward facing powertrain simulation. Method 'sim_drive' runs this
    /// iteratively to achieve correct SOC initial and final conditions, as
    /// needed.
    ///
    /// Arguments
    /// ------------
    /// initSoc (optional): initial battery state-of-charge (SOC) for electrified vehicles
    /// auxInKw: auxInKw override.  Array of same length as cyc.time_s.
    ///         Default of np.zeros(1) causes veh.aux_kw to be used. If zero is actually
    ///         desired as an override, either set veh.aux_kw = 0 before instantiaton of
    ///         SimDrive*, or use `np.finfo(np.float64).tiny` for auxInKw[-1]. Setting
    ///         the final value to non-zero prevents override mechanism.
    pub fn sim_drive_walk(&mut self, init_soc:f64) {
        // TODO: implement method for `init_arrays`
        // TODO: implement method for aux_in_kw_override
        self.cyc_met[0] = true;
        self.cur_soc_target[0] = self.veh.max_soc;
        self.ess_cur_kwh[0] = init_soc * self.veh.max_ess_kwh;
        self.soc[0] = init_soc;
        self.mps_ach[0] = self.cyc0.mps[0];
        self.mph_ach[0] = self.cyc0.mph()[0];

        if self.sim_params.missed_trace_correction {
            self.cyc = self.cyc0.clone();  // reset the cycle in case it has been manipulated
        }
        self.i = 1; // time step counter
        while self.i < self.cyc.time_s.len() {
            self.sim_drive_step()
        }

    // TODO: uncomment and implement
    //    if (self.cyc.dt_s > 5).any() and self.sim_params.verbose:
    //         if self.sim_params.missed_trace_correction:
    //             print('Max time dilation factor =', (round((self.cyc.dt_s / self.cyc0.dt_s).max(), 3)))
    //         print("Warning: large time steps affect accuracy significantly.")
    //         print("To suppress this message, view the doc string for simdrive.SimDriveParams.")
    //         print('Max time step =', (round(self.cyc.dt_s.max(), 3)))
    }

    /// Step through 1 time step.
    fn sim_drive_step(&mut self) {
        self.solve_step(self.i);

        // TODO: implement and uncomment
        // if self.sim_params.missed_trace_correction && (self.cyc0.dist_m.slice(s![0..self.i]).sum() > 0){
        //     self.set_time_dilation(self.i)
        // }

        // TODO: implement something for coasting here
        // if self.impose_coast[i] == true
        //     self.set_coast_speeed(i)

        self.i += 1  // increment time step counter
    }

    /// Perform all the calculations to solve 1 time step.
    fn solve_step(&mut self, i:usize) {
        self.set_misc_calcs(i);
        self.set_comp_lims(i);
        self.set_power_calcs(i);
        self.set_ach_speed(i);
        self.set_hybrid_cont_calcs(i);
        self.set_fc_forced_state(i);
        self.set_hybrid_cont_decisions(i);
        self.set_fc_power(i);
    }


    /// Sets misc. calculations at time step 'i'
    /// Arguments:
    /// ----------
    /// i: index of time step
    fn set_misc_calcs(&mut self, i:usize) {
        // if cycle iteration is used, auxInKw must be re-zeroed to trigger the below if statement
        if self.aux_in_kw.slice(s![i..self.len()-1]).iter().all(|&x| x == 0.0) {
            // if all elements after i-1 are zero, trigger default behavior; otherwise, use override value
            if self.veh.no_elec_aux{
                self.aux_in_kw[i] = self.veh.aux_kw / self.veh.alt_eff;
            } else {
                self.aux_in_kw[i] = self.veh.aux_kw;
            }
        }
        // Is SOC below min threshold?
        if self.soc[i-1] < (self.veh.min_soc + self.veh.perc_high_acc_buf) {
            self.reached_buff[i] = false;
        } else {
            self.reached_buff[i] = true;
        }

        // Does the engine need to be on for low SOC or high acceleration
        if self.soc[i-1] < self.veh.min_soc || (self.high_acc_fc_on_tag[i-1] && !(self.reached_buff[i])){
            self.high_acc_fc_on_tag[i] = true } else{
            self.high_acc_fc_on_tag[i] = false
        }
        self.max_trac_mps[i] = self.mps_ach[i-1] + (self.veh.max_trac_mps2 * self.cyc.dt_s()[i])

    }

    /// Sets component limits for time step 'i'
    /// Arguments
    /// ------------
    /// i: index of time step
    /// initSoc: initial SOC for electrified vehicles
    fn set_comp_lims(&mut self, i:usize) {
        // max fuel storage power output
        self.cur_max_fs_kw_out[i] = min(
            self.veh.max_fuel_stor_kw,
            self.fs_kw_out_ach[i-1] +
                self.veh.max_fuel_stor_kw / self.veh.fuel_stor_secs_to_peak_pwr * self.cyc.dt_s()[i]);
        // maximum fuel storage power output rate of change
        self.fc_trans_lim_kw[i] = self.fc_kw_out_ach[i-1] + (
            self.veh.max_fuel_conv_kw / self.veh.fuel_conv_secs_to_peak_pwr * self.cyc.dt_s()[i]
        );

        self.fc_max_kw_in[i] = min(self.cur_max_fs_kw_out[i], self.veh.max_fuel_stor_kw);
        self.fc_fs_lim_kw[i] = self.veh.max_fuel_conv_kw;
        self.cur_max_fc_kw_out[i] = min(
            self.veh.max_fuel_conv_kw,
            min(self.fc_fs_lim_kw[i], self.fc_trans_lim_kw[i]));

        if self.veh.max_ess_kwh == 0.0 || self.soc[i-1] < self.veh.min_soc {
            self.ess_cap_lim_dischg_kw[i] = 0.0;
        } else {
            self.ess_cap_lim_dischg_kw[i] = self.veh.max_ess_kwh * self.veh.ess_round_trip_eff.sqrt() * 3.6e3 * (
                self.soc[i-1] - self.veh.min_soc) / self.cyc.dt_s()[i];
        }
        self.cur_max_ess_kw_out[i] = min(
            self.veh.max_ess_kw, self.ess_cap_lim_dischg_kw[i]);

        if self.veh.max_ess_kwh == 0.0 || self.veh.max_ess_kw == 0.0 {
            self.ess_cap_lim_chg_kw[i] = 0.0;
        } else {
            self.ess_cap_lim_chg_kw[i] = max(
                (self.veh.max_soc - self.soc[i-1]) * self.veh.max_ess_kwh / self.veh.ess_round_trip_eff.sqrt() /
                (self.cyc.dt_s()[i] / 3.6e3),
                0.0);
        }

        self.cur_max_ess_chg_kw[i] = min(self.ess_cap_lim_chg_kw[i], self.veh.max_ess_kw);

        // Current maximum electrical power that can go toward propulsion, not including motor limitations
        if self.veh.fc_eff_type == H2FC {
            self.cur_max_elec_kw[i] = self.cur_max_fc_kw_out[i] + self.cur_max_roadway_chg_kw[i] + self.cur_max_ess_kw_out[i] - self.aux_in_kw[i];
        } else {
            self.cur_max_elec_kw[i] = self.cur_max_roadway_chg_kw[i] + self.cur_max_ess_kw_out[i] - self.aux_in_kw[i];
        }

        // Current maximum electrical power that can go toward propulsion, including motor limitations
        self.cur_max_avail_elec_kw[i] = min(self.cur_max_elec_kw[i], self.veh.mc_max_elec_in_kw);

        if self.cur_max_elec_kw[i] > 0.0 {
            // limit power going into e-machine controller to
            if self.cur_max_avail_elec_kw[i] == arrmax(&self.veh.mc_kw_in_array) {
                self.mc_elec_in_lim_kw[i] = min(
                    self.veh.mc_kw_out_array[self.veh.mc_kw_out_array.len() - 1],
                    self.veh.max_motor_kw);
            }
            else {
                self.mc_elec_in_lim_kw[i] = min(
                    self.veh.mc_kw_out_array[first_grtr(
                        &self.veh.mc_kw_in_array, min(
                            arrmax(&self.veh.mc_kw_in_array) - 0.01,
                            self.cur_max_avail_elec_kw[i])
                    ).unwrap_or(0) - 1 as usize],
                    self.veh.max_motor_kw)}
        }
        else {
            self.mc_elec_in_lim_kw[i] = 0.0;
        }

        // Motor transient power limit
        self.mc_transi_lim_kw[i] = self.mc_mech_kw_out_ach[i-1].abs() + self.veh.max_motor_kw / self.veh.motor_secs_to_peak_pwr * self.cyc.dt_s()[i];

        self.cur_max_mc_kw_out[i] = max(
            min(min(
                self.mc_elec_in_lim_kw[i],
                self.mc_transi_lim_kw[i]),
                if self.veh.stop_start {0.0} else {1.0} * self.veh.max_motor_kw),
            -self.veh.max_motor_kw
        );

        if self.cur_max_mc_kw_out[i] == 0.0 {
            self.cur_max_mc_elec_kw_in[i] = 0.0;
        } else {
            if self.cur_max_mc_kw_out[i] == self.veh.max_motor_kw {
                self.cur_max_mc_elec_kw_in[i] = self.cur_max_mc_kw_out[i] /
                    self.veh.mc_full_eff_array[self.veh.mc_full_eff_array.len() - 1];
            } else {
                self.cur_max_mc_elec_kw_in[i] = self.cur_max_mc_kw_out[i] / self.veh.mc_full_eff_array[cmp::max(
                    1,
                    first_grtr(
                        &self.veh.mc_kw_out_array, min(
                            self.veh.max_motor_kw - 0.01, self.cur_max_mc_kw_out[i])).unwrap_or(0) - 1
                        )
                    ]
            };
        }

        if self.veh.max_motor_kw == 0.0 {
            self.ess_lim_mc_regen_perc_kw[i] = 0.0;
        }
        else {
            self.ess_lim_mc_regen_perc_kw[i] = min(
                (self.cur_max_ess_chg_kw[i] + self.aux_in_kw[i]) / self.veh.max_motor_kw, 1.0);
        }
        if self.cur_max_ess_chg_kw[i] == 0.0 {
            self.ess_lim_mc_regen_kw[i] = 0.0;
        } else {
            if self.veh.max_motor_kw == self.cur_max_ess_chg_kw[i] - self.cur_max_roadway_chg_kw[i] {
                self.ess_lim_mc_regen_kw[i] = min(
                    self.veh.max_motor_kw, self.cur_max_ess_chg_kw[i] / self.veh.mc_full_eff_array[self.veh.mc_full_eff_array.len() - 1]);
            }
            else {
                self.ess_lim_mc_regen_kw[i] = min(
                    self.veh.max_motor_kw,
                    self.cur_max_ess_chg_kw[i] / self.veh.mc_full_eff_array[
                        cmp::max(1,
                            first_grtr(
                                &self.veh.mc_kw_out_array, min(
                                    self.veh.max_motor_kw - 0.01,
                                    self.cur_max_ess_chg_kw[i] - self.cur_max_roadway_chg_kw[i]
                                )
                            ).unwrap_or(0) - 1
                        )
                    ]
                );
            }
        }
        self.cur_max_mech_mc_kw_in[i] = min(
            self.ess_lim_mc_regen_kw[i], self.veh.max_motor_kw);

        self.cur_max_trac_kw[i] = self.veh.wheel_coef_of_fric * self.veh.drive_axle_weight_frac * self.veh.veh_kg * self.props.a_grav_mps2
            / (1.0 + self.veh.veh_cg_m * self.veh.wheel_coef_of_fric / self.veh.wheel_base_m) / 1e3 * self.max_trac_mps[i];

        if self.veh.fc_eff_type == H2FC {
            if self.veh.no_elec_sys || self.veh.no_elec_aux || self.high_acc_fc_on_tag[i] {
                self.cur_max_trans_kw_out[i] = min(
                    (self.cur_max_mc_kw_out[i] - self.aux_in_kw[i]) * self.veh.trans_eff,
                    self.cur_max_trac_kw[i] / self.veh.trans_eff
                );
            } else
                {
                self.cur_max_trans_kw_out[i] = min(
                    (self.cur_max_mc_kw_out[i] - min(self.cur_max_elec_kw[i], 0.0)) * self.veh.trans_eff,
                    self.cur_max_trac_kw[i] / self.veh.trans_eff
                );
            }
        }
        else {
            if self.veh.no_elec_sys || self.veh.no_elec_aux || self.high_acc_fc_on_tag[i] {
                self.cur_max_trans_kw_out[i] = min(
                    (self.cur_max_mc_kw_out[i] + self.cur_max_fc_kw_out[i] - self.aux_in_kw[i]) * self.veh.trans_eff,
                    self.cur_max_trac_kw[i] / self.veh.trans_eff
                );
            }
            else {
                self.cur_max_trans_kw_out[i] = min(
                    (self.cur_max_mc_kw_out[i] + self.cur_max_fc_kw_out[i] - min(self.cur_max_elec_kw[i], 0.0)) * self.veh.trans_eff,
                    self.cur_max_trac_kw[i] / self.veh.trans_eff
                );
            }
        }
    }

    /// Calculate power requirements to meet cycle and determine if
    /// cycle can be met.
    /// Arguments
    /// ------------
    /// i: index of time step
    fn set_power_calcs(&mut self, i:usize) {
        let mps_ach = if &self.newton_iters[i] > &0u32 {
            self.mps_ach[i]
        } else {
            self.cyc.mps[i]
        };

        self.cyc_drag_kw[i] = 0.5 * self.props.air_density_kg_per_m3 * self.veh.drag_coef * self.veh.frontal_area_m2 * (
            (self.mps_ach[i-1] + mps_ach) / 2.0).powf(3.0) / 1e3;
        self.cyc_accel_kw[i] = self.veh.veh_kg / (2.0 * self.cyc.dt_s()[i]) * (mps_ach.powf(2.0) - self.mps_ach[i-1].powf(2.0)) / 1e3;
        self.cyc_ascent_kw[i] = self.props.a_grav_mps2 * self.cyc.grade[i].atan().sin() *
            self.veh.veh_kg * (self.mps_ach[i-1] + mps_ach) / 2.0 / 1e3;
        self.cyc_trac_kw_req[i] = self.cyc_drag_kw[i] + self.cyc_accel_kw[i] + self.cyc_ascent_kw[i];
        self.spare_trac_kw[i] = self.cur_max_trac_kw[i] - self.cyc_trac_kw_req[i];
        self.cyc_rr_kw[i] = self.veh.veh_kg * self.props.a_grav_mps2 * self.veh.wheel_rr_coef *
            self.cyc.grade[i].atan().cos() * (self.mps_ach[i-1] + mps_ach) / 2.0 / 1e3;
        self.cyc_whl_rad_per_sec[i] = mps_ach / self.veh.wheel_radius_m;
        self.cyc_tire_inertia_kw[i] = (
            0.5 * self.veh.wheel_inertia_kg_m2 * self.veh.num_wheels * self.cyc_whl_rad_per_sec[i].powf(2.0) / self.cyc.dt_s()[i] -
            0.5 * self.veh.wheel_inertia_kg_m2 * self.veh.num_wheels * (self.mps_ach[i-1] / self.veh.wheel_radius_m).powf(2.0) /
            self.cyc.dt_s()[i]) / 1e3;

        self.cyc_whl_kw_req[i] = self.cyc_trac_kw_req[i] + self.cyc_rr_kw[i] + self.cyc_tire_inertia_kw[i];
        self.regen_contrl_lim_kw_perc[i] = self.veh.max_regen / (1.0 + self.veh.regen_a * (-self.veh.regen_b * (
            (self.cyc.mph()[i] + self.mps_ach[i-1] * params::MPH_PER_MPS) / 2.0 + 1.0)).exp());
        self.cyc_regen_brake_kw[i] = max(min(
                self.cur_max_mech_mc_kw_in[i] * self.veh.trans_eff,
                self.regen_contrl_lim_kw_perc[i] * -self.cyc_whl_kw_req[i]),
            0.0
        );
        self.cyc_fric_brake_kw[i] = -min(self.cyc_regen_brake_kw[i] + self.cyc_whl_kw_req[i], 0.0);
        self.cyc_trans_kw_out_req[i] = self.cyc_whl_kw_req[i] + self.cyc_fric_brake_kw[i];

        if self.cyc_trans_kw_out_req[i] <= self.cur_max_trans_kw_out[i] {
            self.cyc_met[i] = true;
            self.trans_kw_out_ach[i] = self.cyc_trans_kw_out_req[i];
        }

        else {
            self.cyc_met[i] = false;
            self.trans_kw_out_ach[i] = self.cur_max_trans_kw_out[i];
        }

        if self.trans_kw_out_ach[i] > 0.0 {
            self.trans_kw_in_ach[i] = self.trans_kw_out_ach[i] / self.veh.trans_eff;
        }
        else {
            self.trans_kw_in_ach[i] = self.trans_kw_out_ach[i] * self.veh.trans_eff;
        }

        if self.cyc_met[i]{
            if self.veh.fc_eff_type == H2FC {
                self.min_mc_kw_2help_fc[i] = max(
                    self.trans_kw_in_ach[i], -self.cur_max_mech_mc_kw_in[i]);
            } else {
                self.min_mc_kw_2help_fc[i] = max(
                    self.trans_kw_in_ach[i] - self.cur_max_fc_kw_out[i], -self.cur_max_mech_mc_kw_in[i]);
            }
        } else {
            self.min_mc_kw_2help_fc[i] = max(
                self.cur_max_mc_kw_out[i], -self.cur_max_mech_mc_kw_in[i]);
        }
    }

    // Calculate actual speed achieved if vehicle hardware cannot achieve trace speed.
    // Arguments
    // ------------
    // i: index of time step
    fn set_ach_speed(&mut self, i:usize) {
        // Cycle is met
        if self.cyc_met[i] {
            self.mps_ach[i] = self.cyc.mps[i];
        }

        //Cycle is not met
        else {
            let drag3 = 1.0 / 16.0 * self.props.air_density_kg_per_m3 *
                self.veh.drag_coef * self.veh.frontal_area_m2;
            let accel2 = 0.5 * self.veh.veh_kg / self.cyc.dt_s()[i];
            let drag2 = 3.0 / 16.0 * self.props.air_density_kg_per_m3 *
                self.veh.drag_coef * self.veh.frontal_area_m2 * self.mps_ach[i-1];
            let wheel2 = 0.5 * self.veh.wheel_inertia_kg_m2 *
                self.veh.num_wheels / (self.cyc.dt_s()[i] * self.veh.wheel_radius_m.powf(2.0));
            let drag1 = 3.0 / 16.0 * self.props.air_density_kg_per_m3 * self.veh.drag_coef *
                self.veh.frontal_area_m2 * self.mps_ach[i-1].powf(2.0);
            let roll1 = 0.5 * self.veh.veh_kg * self.props.a_grav_mps2 * self.veh.wheel_rr_coef
                * self.cyc.grade[i].atan().cos();
            let ascent1 = 0.5 * self.props.a_grav_mps2 * self.cyc.grade[i].atan().sin() * self.veh.veh_kg;
            let accel0 = -0.5 * self.veh.veh_kg * self.mps_ach[i-1].powf(2.0) / self.cyc.dt_s()[i];
            let drag0 = 1.0 / 16.0 * self.props.air_density_kg_per_m3 * self.veh.drag_coef *
                self.veh.frontal_area_m2 * self.mps_ach[i-1].powf(3.0);
            let roll0 = 0.5 * self.veh.veh_kg * self.props.a_grav_mps2 *
                self.veh.wheel_rr_coef * self.cyc.grade[i].atan().cos() * self.mps_ach[i-1];
            let ascent0 = 0.5 * self.props.a_grav_mps2 * self.cyc.grade[i].atan().sin()
                * self.veh.veh_kg * self.mps_ach[i-1];
            let wheel0 = -0.5 * self.veh.wheel_inertia_kg_m2 * self.veh.num_wheels *
                self.mps_ach[i-1].powf(2.0) / (self.cyc.dt_s()[i] * self.veh.wheel_radius_m.powf(2.0));

            let total3 = drag3 / 1e3;
            let total2 = (accel2 + drag2 + wheel2) / 1e3;
            let total1 = (drag1 + roll1 + ascent1) / 1e3;
            let total0 = (accel0 + drag0 + roll0 + ascent0 + wheel0) / 1e3 - self.cur_max_trans_kw_out[i];

            let totals = array![total3, total2, total1, total0];

            let t3 = totals[0];
            let t2 = totals[1];
            let t1 = totals[2];
            let t0 = totals[3];
            // initial guess
            let xi = max(1.0, self.mps_ach[i-1]);
            // stop criteria
            let max_iter = self.sim_params.newton_max_iter;
            let xtol = self.sim_params.newton_xtol;
            // solver gain
            let g = self.sim_params.newton_gain;
            let yi = t3 * xi.powf(3.0) + t2 * xi.powf(2.0) + t1 * xi + t0;
            let mi = 3.0 * t3 * xi.powf(2.0) + 2.0 * t2 * xi + t1;
            let bi = yi - xi * mi;
            let mut xs = vec![xi];
            let mut ys = vec![yi];
            let mut ms = vec![mi];
            let mut bs = vec![bi];
            let mut iterate = 1;
            let mut converged = false;
            while iterate < max_iter && !converged {
                // let end = ;
                let xi = xs[xs.len() - 1] * (1.0 - g) - g * bs[xs.len() - 1] / ms[xs.len() - 1];
                let yi = t3 * xi.powf(3.0) + t2 * xi.powf(2.0) + t1 * xi + t0;
                let mi = 3.0 * t3 * xi.powf(2.0) + 2.0 * t2 * xi + t1;
                let bi = yi - xi * mi;
                xs.push(xi);
                ys.push(yi);
                ms.push(mi);
                bs.push(bi);
                converged = ((xs[xs.len()-1] - xs[xs.len()-2]) / xs[xs.len()-2]).abs() < xtol;
                iterate += 1;
            }

            self.newton_iters[i] = iterate;

            let _ys = Array::from_vec(ys).map(|x| x.abs());
            self.mps_ach[i] = xs[_ys.iter().position(|&x| x == ndarrmin(&_ys)).unwrap()];
        }

        self.set_power_calcs(i);

        self.mph_ach[i] = self.mps_ach[i] * params::MPH_PER_MPS;
        self.dist_m[i] = self.mps_ach[i] * self.cyc.dt_s()[i];
        self.dist_mi[i] = self.dist_m[i] * 1.0 / params::M_PER_MI;
    }

    /// Hybrid control calculations.
    /// Arguments
    /// ------------
    /// i: index of time step
    fn set_hybrid_cont_calcs(&mut self, i:usize) {
        if self.veh.no_elec_sys {
            self.regen_buff_soc[i] = 0.0;
        }
        else if self.veh.charging_on {
            self.regen_buff_soc[i] = max(
                self.veh.max_soc - (self.veh.max_regen_kwh() / self.veh.max_ess_kwh),
                (self.veh.max_soc + self.veh.min_soc) / 2.0);
        }
        else {
            self.regen_buff_soc[i] = max(
                (self.veh.max_ess_kwh * self.veh.max_soc -
                    0.5 * self.veh.veh_kg * (self.cyc.mps[i].powf(2.0)) * (1.0 / 1_000.0) * (1.0 / 3_600.0) *
                    self.veh.mc_peak_eff() * self.veh.max_regen) / self.veh.max_ess_kwh,
                self.veh.min_soc
            );

            self.ess_regen_buff_dischg_kw[i] = min(self.cur_max_ess_kw_out[i], max(
                0.0, (self.soc[i-1] - self.regen_buff_soc[i]) * self.veh.max_ess_kwh * 3_600.0 / self.cyc.dt_s()[i]));

            self.max_ess_regen_buff_chg_kw[i] = min(max(
                    0.0,
                    (self.regen_buff_soc[i] - self.soc[i-1]) * self.veh.max_ess_kwh * 3.6e3 / self.cyc.dt_s()[i]),
                self.cur_max_ess_chg_kw[i]
            );
        }
        if self.veh.no_elec_sys {
            self.accel_buff_soc[i] = 0.0;
        }
        else {
            self.accel_buff_soc[i] = min(
                max(
                    ((self.veh.max_accel_buffer_mph / params::MPH_PER_MPS).powf(2.0) - self.cyc.mps[i].powf(2.0)) /
                    (self.veh.max_accel_buffer_mph / params::MPH_PER_MPS).powf(2.0) * min(
                        self.veh.max_accel_buffer_perc_of_useable_soc * (self.veh.max_soc - self.veh.min_soc),
                        self.veh.max_regen_kwh() / self.veh.max_ess_kwh
                    ) * self.veh.max_ess_kwh / self.veh.max_ess_kwh + self.veh.min_soc,
                    self.veh.min_soc
                ),
                self.veh.max_soc
                );

            self.ess_accel_buff_chg_kw[i] = max(
                0.0, (self.accel_buff_soc[i] - self.soc[i-1]) * self.veh.max_ess_kwh * 3.6e3 / self.cyc.dt_s()[i]);
            self.max_ess_accell_buff_dischg_kw[i] = min(
                max(
                    0.0,
                    (self.soc[i-1] - self.accel_buff_soc[i]) * self.veh.max_ess_kwh * 3.6e3 / self.cyc.dt_s()[i]),
                self.cur_max_ess_kw_out[i]
            );
        }
        if self.regen_buff_soc[i] < self.accel_buff_soc[i] {
            self.ess_accel_regen_dischg_kw[i] = max(
                min(
                    (self.soc[i-1] - (self.regen_buff_soc[i] + self.accel_buff_soc[i]) / 2.0) * self.veh.max_ess_kwh * 3.6e3 / self.cyc.dt_s()[i],
                    self.cur_max_ess_kw_out[i]
                ),
                -self.cur_max_ess_chg_kw[i]
            );
        } else if self.soc[i-1] > self.regen_buff_soc[i] {
            self.ess_accel_regen_dischg_kw[i] = max(
                min(
                    self.ess_regen_buff_dischg_kw[i],
                    self.cur_max_ess_kw_out[i]),
                -self.cur_max_ess_chg_kw[i]
            );
        } else if self.soc[i-1] < self.accel_buff_soc[i] {
            self.ess_accel_regen_dischg_kw[i] = max(
                min(-1.0 * self.ess_accel_buff_chg_kw[i], self.cur_max_ess_kw_out[i]), -self.cur_max_ess_chg_kw[i]);
        } else {
            self.ess_accel_regen_dischg_kw[i] = max(
                min(0.0, self.cur_max_ess_kw_out[i]), -self.cur_max_ess_chg_kw[i]);
        }
        self.fc_kw_gap_fr_eff[i] = (self.trans_kw_out_ach[i] - self.veh.max_fc_eff_kw()).abs();

        if self.veh.no_elec_sys {
            self.mc_elec_in_kw_for_max_fc_eff[i] = 0.0;
        }
        else if self.trans_kw_out_ach[i] < self.veh.max_fc_eff_kw() {
            if self.fc_kw_gap_fr_eff[i] == self.veh.max_motor_kw {
                self.mc_elec_in_kw_for_max_fc_eff[i] = -self.fc_kw_gap_fr_eff[i] / self.veh.mc_full_eff_array[self.veh.mc_full_eff_array.len()-1];
            }
            else {
                self.mc_elec_in_kw_for_max_fc_eff[i] = -self.fc_kw_gap_fr_eff[i] /
                    self.veh.mc_full_eff_array[cmp::max(
                        1,
                        first_grtr(&self.veh.mc_kw_out_array,
                            min(self.veh.max_motor_kw - 0.01, self.fc_kw_gap_fr_eff[i])).unwrap_or(0) - 1)];
            }
        }
        else {
            if self.fc_kw_gap_fr_eff[i] == self.veh.max_motor_kw {
                self.mc_elec_in_kw_for_max_fc_eff[i] = self.veh.mc_kw_in_array[
                    self.veh.mc_kw_in_array.len() - 1];
            }
            else {
                self.mc_elec_in_kw_for_max_fc_eff[i] = self.veh.mc_kw_in_array[
                    first_grtr(
                        &self.veh.mc_kw_out_array,
                            min(self.veh.max_motor_kw - 0.01, self.fc_kw_gap_fr_eff[i])).unwrap_or(0) - 1];
            }
        }
        if self.veh.no_elec_sys {
            self.elec_kw_req_4ae[i] = 0.0;
        }
        else if self.trans_kw_in_ach[i] > 0.0 {
            if self.trans_kw_in_ach[i] == self.veh.max_motor_kw {
                self.elec_kw_req_4ae[i] = self.trans_kw_in_ach[i] / self.veh.mc_full_eff_array[
                    self.veh.mc_full_eff_array.len()-1] + self.aux_in_kw[i];
            }
            else {
                self.elec_kw_req_4ae[i] = self.trans_kw_in_ach[i] /
                    self.veh.mc_full_eff_array[cmp::max(
                        1,
                        first_grtr(&self.veh.mc_kw_out_array,
                            min(self.veh.max_motor_kw - 0.01, self.trans_kw_in_ach[i])).unwrap_or(0) - 1)] + self.aux_in_kw[i]
                ;
            }
        }
        else {
            self.elec_kw_req_4ae[i] = 0.0;
        }

        self.prev_fc_time_on[i] = self.fc_time_on[i-1];

        // some conditions in the following if statement have a buffer of 1e-6 to prevent false positives/negatives because these have been encountered in practice.
        if self.veh.max_fuel_conv_kw == 0.0 {
            self.can_pwr_all_elec[i] = self.accel_buff_soc[i] < self.soc[i-1] &&
                (self.trans_kw_in_ach[i] - 1e-6) <= self.cur_max_mc_kw_out[i] &&
                (self.elec_kw_req_4ae[i] < self.cur_max_elec_kw[i] || self.veh.max_fuel_conv_kw == 0.0);
        }
        else {
            self.can_pwr_all_elec[i] = self.accel_buff_soc[i] < self.soc[i-1] &&
                (self.trans_kw_in_ach[i] - 1e-6) <= self.cur_max_mc_kw_out[i] &&
                (self.elec_kw_req_4ae[i] < self.cur_max_elec_kw[i] || self.veh.max_fuel_conv_kw == 0.0)
                && ((self.cyc.mph()[i] - 1e-6) <= self.veh.mph_fc_on || self.veh.charging_on) &&
                self.elec_kw_req_4ae[i] <= self.veh.kw_demand_fc_on;
        }
        if self.can_pwr_all_elec[i] {

            if self.trans_kw_in_ach[i] < self.aux_in_kw[i] {
                self.desired_ess_kw_out_for_ae[i] = self.aux_in_kw[i] + self.trans_kw_in_ach[i];
            }
            else if self.regen_buff_soc[i] < self.accel_buff_soc[i] {
                self.desired_ess_kw_out_for_ae[i] = self.ess_accel_regen_dischg_kw[i];
            }
            else if self.soc[i-1] > self.regen_buff_soc[i] {
                self.desired_ess_kw_out_for_ae[i] = self.ess_regen_buff_dischg_kw[i];
            }
            else if self.soc[i-1] < self.accel_buff_soc[i] {
                self.desired_ess_kw_out_for_ae[i] = -self.ess_accel_buff_chg_kw[i];
            }
            else {
                self.desired_ess_kw_out_for_ae[i] = self.trans_kw_in_ach[i] + self.aux_in_kw[i] - self.cur_max_roadway_chg_kw[i];
            }
        }
        else {
            self.desired_ess_kw_out_for_ae[i] = 0.0;
        }

        if self.can_pwr_all_elec[i] {
            self.ess_ae_kw_out[i] = max(
                -self.cur_max_ess_chg_kw[i],
                max(-self.max_ess_regen_buff_chg_kw[i],
                    max(min(0.0, self.cur_max_roadway_chg_kw[i] - self.trans_kw_in_ach[i] + self.aux_in_kw[i]),
                        min(self.cur_max_ess_kw_out[i], self.desired_ess_kw_out_for_ae[i])))
            );
        }
        else {
            self.ess_ae_kw_out[i] = 0.0;
        }

        self.er_ae_kw_out[i] = min(
            max(0.0, self.trans_kw_in_ach[i] + self.aux_in_kw[i] - self.ess_ae_kw_out[i]),
            self.cur_max_roadway_chg_kw[i]);
    }

    /// Calculate control variables related to engine on/off state
    /// Arguments
    /// ------------
    /// i: index of time step
    fn set_fc_forced_state(&mut self, i:usize) {
        // force fuel converter on if it was on in the previous time step, but only if fc
        // has not been on longer than minFcTimeOn
        if self.prev_fc_time_on[i] > 0.0 && self.prev_fc_time_on[i] < self.veh.min_fc_time_on - self.cyc.dt_s()[i]{
            self.fc_forced_on[i] = true;
        } else {
            self.fc_forced_on[i] = false
        }

        if !self.fc_forced_on[i] || !self.can_pwr_all_elec[i] {
            self.fc_forced_state[i] = 1;
            self.mc_mech_kw_4forced_fc[i] = 0.0;
        } else if self.trans_kw_in_ach[i] < 0.0 {
            self.fc_forced_state[i] = 2;
            self.mc_mech_kw_4forced_fc[i] = self.trans_kw_in_ach[i];
        } else if self.veh.max_fc_eff_kw() == self.trans_kw_in_ach[i] {
            self.fc_forced_state[i] = 3;
            self.mc_mech_kw_4forced_fc[i] = 0.0 } else if self.veh.idle_fc_kw > self.trans_kw_in_ach[i] && self.cyc_accel_kw[i] >= 0.0 {
            self.fc_forced_state[i] = 4;
            self.mc_mech_kw_4forced_fc[i] = self.trans_kw_in_ach[i] - self.veh.idle_fc_kw;
        } else if self.veh.max_fc_eff_kw() > self.trans_kw_in_ach[i] {
            self.fc_forced_state[i] = 5;
            self.mc_mech_kw_4forced_fc[i] = 0.0;
        } else {
            self.fc_forced_state[i] = 6;
            self.mc_mech_kw_4forced_fc[i] = self.trans_kw_in_ach[i] - self.veh.max_fc_eff_kw();
        }
        // Ok(())
    }


    /// Hybrid control decisions.
    /// Arguments
    /// ------------
    /// i: index of time step
    fn set_hybrid_cont_decisions(&mut self, i:usize) {
        if (-self.mc_elec_in_kw_for_max_fc_eff[i] - self.cur_max_roadway_chg_kw[i]) > 0.0 {
            self.ess_desired_kw_4fc_eff[i] = (-self.mc_elec_in_kw_for_max_fc_eff[i] -
                self.cur_max_roadway_chg_kw[i]) * self.veh.ess_dischg_to_fc_max_eff_perc;
        } else {
            self.ess_desired_kw_4fc_eff[i] = (-self.mc_elec_in_kw_for_max_fc_eff[i] -
                self.cur_max_roadway_chg_kw[i]) * self.veh.ess_chg_to_fc_max_eff_perc;
        }

        if self.accel_buff_soc[i] > self.regen_buff_soc[i] {
            self.ess_kw_if_fc_req[i] = min(
                self.cur_max_ess_kw_out[i],
                min(
                    self.veh.mc_max_elec_in_kw + self.aux_in_kw[i], 
                    min(self.cur_max_mc_elec_kw_in[i] + self.aux_in_kw[i],
                        max(-self.cur_max_ess_chg_kw[i], self.ess_accel_regen_dischg_kw[i])))
                );
        } else if self.ess_regen_buff_dischg_kw[i] > 0.0 {
            self.ess_kw_if_fc_req[i] = min(
                self.cur_max_ess_kw_out[i],
                min(self.veh.mc_max_elec_in_kw + self.aux_in_kw[i], 
                    min(self.cur_max_mc_elec_kw_in[i] + self.aux_in_kw[i],
                        max(-self.cur_max_ess_chg_kw[i],
                            min(self.ess_accel_regen_dischg_kw[i],
                                min(self.mc_elec_in_lim_kw[i] + self.aux_in_kw[i],
                                    max(self.ess_regen_buff_dischg_kw[i], self.ess_desired_kw_4fc_eff[i]))
                            )
                        )
                    )
                )
            );
        } else if self.ess_accel_buff_chg_kw[i] > 0.0 {
            self.ess_kw_if_fc_req[i] = min(
                self.cur_max_ess_kw_out[i],
                min(self.veh.mc_max_elec_in_kw + self.aux_in_kw[i], 
                    min(self.cur_max_mc_elec_kw_in[i] + self.aux_in_kw[i],
                        max(-self.cur_max_ess_chg_kw[i],
                            max(-self.max_ess_regen_buff_chg_kw[i],
                                min(-self.ess_accel_buff_chg_kw[i], self.ess_desired_kw_4fc_eff[i])
                            )
                        )
                    )
                )
            );
        } else if self.ess_desired_kw_4fc_eff[i] > 0.0 {
            self.ess_kw_if_fc_req[i] = min(
                self.cur_max_ess_kw_out[i],
                min(self.veh.mc_max_elec_in_kw + self.aux_in_kw[i],
                    min(self.cur_max_mc_elec_kw_in[i] + self.aux_in_kw[i],
                        max(-self.cur_max_ess_chg_kw[i],
                            min(self.ess_desired_kw_4fc_eff[i], self.max_ess_accell_buff_dischg_kw[i])
                        )
                    )
                )
            );
        } else {
            self.ess_kw_if_fc_req[i] = min(
                self.cur_max_ess_kw_out[i],
                min(self.veh.mc_max_elec_in_kw + self.aux_in_kw[i],
                    min(self.cur_max_mc_elec_kw_in[i] + self.aux_in_kw[i],
                        max(-self.cur_max_ess_chg_kw[i],
                            max(self.ess_desired_kw_4fc_eff[i], -self.max_ess_regen_buff_chg_kw[i])
                        )
                    )
                )
            );
        }

        self.er_kw_if_fc_req[i] = max(0.0,
            min(
                self.cur_max_roadway_chg_kw[i], 
                min(self.cur_max_mech_mc_kw_in[i],
                    self.ess_kw_if_fc_req[i] - self.mc_elec_in_lim_kw[i] + self.aux_in_kw[i])
            )
        );

        self.mc_elec_kw_in_if_fc_req[i] = self.ess_kw_if_fc_req[i] + self.er_kw_if_fc_req[i] - self.aux_in_kw[i];

        if self.veh.no_elec_sys {
            self.mc_kw_if_fc_req[i] = 0.0;
        } else if self.mc_elec_kw_in_if_fc_req[i] == 0.0 {
            self.mc_kw_if_fc_req[i] = 0.0;
        }

        else if self.mc_elec_kw_in_if_fc_req[i] > 0.0 {
            if self.mc_elec_kw_in_if_fc_req[i] == arrmax(&self.veh.mc_kw_in_array){
                self.mc_kw_if_fc_req[i] = self.mc_elec_kw_in_if_fc_req[i] * self.veh.mc_full_eff_array[self.veh.mc_full_eff_array.len()-1];
            }
            else {
                self.mc_kw_if_fc_req[i] = self.mc_elec_kw_in_if_fc_req[i] * self.veh.mc_full_eff_array[
                    cmp::max(1, first_grtr(
                            &self.veh.mc_kw_in_array, min(arrmax(&self.veh.mc_kw_in_array) - 0.01, self.mc_elec_kw_in_if_fc_req[i])
                        ).unwrap_or(0) - 1
                    )
                ]
            }
        }

        else {
            if -self.mc_elec_kw_in_if_fc_req[i] == arrmax(&self.veh.mc_kw_in_array) {
                self.mc_kw_if_fc_req[i] = self.mc_elec_kw_in_if_fc_req[i] / self.veh.mc_full_eff_array[self.veh.mc_full_eff_array.len()-1];
            }
            else {
                self.mc_kw_if_fc_req[i] = self.mc_elec_kw_in_if_fc_req[i] / self.veh.mc_full_eff_array[
                    cmp::max(1, first_grtr(
                        &self.veh.mc_kw_in_array, min(
                            arrmax(&self.veh.mc_kw_in_array) - 0.01, 
                            -self.mc_elec_kw_in_if_fc_req[i])).unwrap_or(0) - 1
                    )
                ];
            }
        }

        if self.veh.max_motor_kw == 0.0 {
            self.mc_mech_kw_out_ach[i] = 0.0;
        } else if self.fc_forced_on[i] && self.can_pwr_all_elec[i] && (self.veh.veh_pt_type == HEV || self.veh.veh_pt_type == PHEV) && (self.veh.fc_eff_type != H2FC) {
            self.mc_mech_kw_out_ach[i] = self.mc_mech_kw_4forced_fc[i];
        } else if self.trans_kw_in_ach[i] <= 0.0 {
            if self.veh.fc_eff_type !=H2FC && self.veh.max_fuel_conv_kw > 0.0 {
                if self.can_pwr_all_elec[i] {
                    self.mc_mech_kw_out_ach[i] = - min(self.cur_max_mech_mc_kw_in[i], -self.trans_kw_in_ach[i]);
                } else {
                    self.mc_mech_kw_out_ach[i] = min(
                        -min(self.cur_max_mech_mc_kw_in[i], -self.trans_kw_in_ach[i]),
                        max(-self.cur_max_fc_kw_out[i], self.mc_kw_if_fc_req[i])
                    );
                } 
            } else {
                    self.mc_mech_kw_out_ach[i] = min(
                        -min(self.cur_max_mech_mc_kw_in[i], -self.trans_kw_in_ach[i]),
                        -self.trans_kw_in_ach[i]
                    );
            }
        } else if self.can_pwr_all_elec[i] {
            self.mc_mech_kw_out_ach[i] = self.trans_kw_in_ach[i]
        }

        else {
            self.mc_mech_kw_out_ach[i] = max(self.min_mc_kw_2help_fc[i], self.mc_kw_if_fc_req[i])
        }

        if self.mc_mech_kw_out_ach[i] == 0.0{
            self.mc_elec_kw_in_ach[i] = 0.0;
        }

        else if self.mc_mech_kw_out_ach[i] < 0.0 {
            if -self.mc_mech_kw_out_ach[i] == arrmax(&self.veh.mc_kw_in_array){
                self.mc_elec_kw_in_ach[i] = self.mc_mech_kw_out_ach[i] * self.veh.mc_full_eff_array[self.veh.mc_full_eff_array.len()-1]
            } else {
                self.mc_elec_kw_in_ach[i] = self.mc_mech_kw_out_ach[i] * self.veh.mc_full_eff_array[
                    cmp::max(1, first_grtr(&self.veh.mc_kw_in_array, min(
                        arrmax(&self.veh.mc_kw_in_array) - 0.01,
                        -self.mc_mech_kw_out_ach[i])).unwrap_or(0) - 1
                    )
                ];
            }
        }
        else {
            if self.veh.max_motor_kw == self.mc_mech_kw_out_ach[i] {
                self.mc_elec_kw_in_ach[i] = self.mc_mech_kw_out_ach[i] / self.veh.mc_full_eff_array[self.veh.mc_full_eff_array.len()-1]
            } else {
                self.mc_elec_kw_in_ach[i] = self.mc_mech_kw_out_ach[i] / self.veh.mc_full_eff_array[
                    cmp::max(1, first_grtr(&self.veh.mc_kw_out_array, min(
                        self.veh.max_motor_kw - 0.01,
                        self.mc_mech_kw_out_ach[i])).unwrap_or(0) - 1
                    )
                ];
            }
        }

        if self.cur_max_roadway_chg_kw[i] == 0.0 {
            self.roadway_chg_kw_out_ach[i] = 0.0 
        } else if self.veh.fc_eff_type == H2FC {
            self.roadway_chg_kw_out_ach[i] = max(
                0.0,
                max(self.mc_elec_kw_in_ach[i],
                    max(self.max_ess_regen_buff_chg_kw[i],
                        max(self.ess_regen_buff_dischg_kw[i],
                            self.cur_max_roadway_chg_kw[i]
                        )
                    )
                )
            );
        } else if self.can_pwr_all_elec[i] {
            self.roadway_chg_kw_out_ach[i] = self.er_ae_kw_out[i];
        } else {
            self.roadway_chg_kw_out_ach[i] = self.er_kw_if_fc_req[i];
        }

        self.min_ess_kw_2help_fc[i] = self.mc_elec_kw_in_ach[i] + self.aux_in_kw[i] - 
            self.cur_max_fc_kw_out[i] - self.roadway_chg_kw_out_ach[i];

        if self.veh.max_ess_kw == 0.0 || self.veh.max_ess_kwh == 0.0 {
            self.ess_kw_out_ach[i] = 0.0;
        } else if self.veh.fc_eff_type == H2FC {
            if self.trans_kw_out_ach[i] >= 0.0 {
                self.ess_kw_out_ach[i] = min(
                    self.cur_max_ess_kw_out[i],
                    min(self.mc_elec_kw_in_ach[i] + self.aux_in_kw[i] - self.roadway_chg_kw_out_ach[i],
                        max(
                            self.min_ess_kw_2help_fc[i],
                            max(self.ess_desired_kw_4fc_eff[i],
                                self.ess_accel_regen_dischg_kw[i])))
                );
            } else {
                self.ess_kw_out_ach[i] = self.mc_elec_kw_in_ach[i] + 
                    self.aux_in_kw[i] - self.roadway_chg_kw_out_ach[i];
            }
        } else if self.high_acc_fc_on_tag[i] || self.veh.no_elec_aux{
            self.ess_kw_out_ach[i] = self.mc_elec_kw_in_ach[i] - self.roadway_chg_kw_out_ach[i];
        }

        else {
            self.ess_kw_out_ach[i] = self.mc_elec_kw_in_ach[i] + self.aux_in_kw[i] - self.roadway_chg_kw_out_ach[i];
        }

        if self.veh.no_elec_sys{
            self.ess_cur_kwh[i] = 0.0
        } else if self.ess_kw_out_ach[i] < 0.0 {
            self.ess_cur_kwh[i] = self.ess_cur_kwh[i-1] - self.ess_kw_out_ach[i] * self.cyc.dt_s()[i] /
                3.6e3 * self.veh.ess_round_trip_eff.sqrt();
        } else {
            self.ess_cur_kwh[i] = self.ess_cur_kwh[i-1] - self.ess_kw_out_ach[i] * self.cyc.dt_s()[i] / 
                3.6e3 * self.veh.ess_round_trip_eff.sqrt();
        }

        if self.veh.max_ess_kwh == 0.0 {
            self.soc[i] = 0.0;
        } else {
            self.soc[i] = self.ess_cur_kwh[i] / self.veh.max_ess_kwh;
        } 

        if self.can_pwr_all_elec[i] && !self.fc_forced_on[i] && self.fc_kw_out_ach[i] == 0.0{
            self.fc_time_on[i] = 0.0
        } else {
            self.fc_time_on[i] = self.fc_time_on[i-1] + self.cyc.dt_s()[i];
        }
    }

    /// Sets fcKwOutAch and fcKwInAch.
    /// Arguments
    /// ------------
    /// i: index of time step
    fn set_fc_power(&mut self, i:usize) {
        if self.veh.max_fuel_conv_kw == 0.0 {
            self.fc_kw_out_ach[i] = 0.0;
        } else if self.veh.fc_eff_type == H2FC {
            self.fc_kw_out_ach[i] = min(
                self.cur_max_fc_kw_out[i], 
                max(0.0, 
                    self.mc_elec_kw_in_ach[i] + self.aux_in_kw[i] - self.ess_kw_out_ach[i] - self.roadway_chg_kw_out_ach[i]
                )
            );
        } else if self.veh.no_elec_sys || self.veh.no_elec_aux || self.high_acc_fc_on_tag[i] {
            self.fc_kw_out_ach[i] = min(
                self.cur_max_fc_kw_out[i], 
                max(
                    0.0, 
                    self.trans_kw_in_ach[i] - self.mc_mech_kw_out_ach[i] + self.aux_in_kw[i]
                )
            );
        } else {
            self.fc_kw_out_ach[i] = min(self.cur_max_fc_kw_out[i], max(
                0.0, self.trans_kw_in_ach[i] - self.mc_mech_kw_out_ach[i]));
            }

        if self.veh.max_fuel_conv_kw == 0.0 {
            self.fc_kw_out_ach_pct[i] = 0.0;
        } else {
            self.fc_kw_out_ach_pct[i] = self.fc_kw_out_ach[i] / self.veh.max_fuel_conv_kw;
        }

        if self.fc_kw_out_ach[i] == 0.0 {
            self.fc_kw_in_ach[i] = 0.0;
            self.fc_kw_out_ach_pct[i] = 0.0;
        } else {
            if self.veh.fc_eff_array[first_grtr(
                &self.veh.fc_kw_out_array, min(self.fc_kw_out_ach[i], self.veh.max_fuel_conv_kw)).unwrap_or(0) - 1] != 0.0 {
                self.fc_kw_in_ach[i] = self.fc_kw_out_ach[i] / (self.veh.fc_eff_array[first_grtr(
                        &self.veh.fc_kw_out_array, min(self.fc_kw_out_ach[i], self.veh.max_fuel_conv_kw)).unwrap_or(0) - 1]);
            } else {
                self.fc_kw_in_ach[i] = 0.0
            }
        }

        self.fs_kw_out_ach[i] = self.fc_kw_in_ach[i];

        self.fs_kwh_out_ach[i] = self.fs_kw_out_ach[i] * self.cyc.dt_s()[i] / 3.6e3;
    }

    /// Sets scalar variables that can be calculated after a cycle is run. 
    /// This includes mpgge, various energy metrics, and others
    pub fn set_post_scalars(& mut self) {
       // 
       // self.fs_cumu_mj_out_ach = (self.fs_kw_out_ach * self.cyc.dt_s).cumsum() * 1e-3

       // if self.fs_kwh_out_ach.sum() == 0:
       //     self.mpgge = 0.0

       // else:
       //     self.mpgge = self.dist_mi.sum() / (self.fs_kwh_out_ach.sum() / self.props.kwh_per_gge)

       // self.roadway_chg_kj = (self.roadway_chg_kw_out_ach * self.cyc.dt_s).sum()
       // self.ess_dischg_kj = - \
       //     (self.soc[-1] - self.soc[0]) * self.veh.max_ess_kwh * 3.6e3
       // self.battery_kwh_per_mi  = (
       //     self.ess_dischg_kj / 3.6e3) / self.dist_mi.sum()
       // self.electric_kwh_per_mi  = (
       //     (self.roadway_chg_kj + self.ess_dischg_kj) / 3.6e3) / self.dist_mi.sum()
       self.fuel_kj = (self.fs_kw_out_ach.clone() * self.cyc.dt_s()).sum();

       // if (self.fuel_kj + self.roadway_chg_kj) == 0:
       //     self.ess2fuel_kwh  = 1.0

       // else:
       //     self.ess2fuel_kwh  = self.ess_dischg_kj / (self.fuel_kj + self.roadway_chg_kj)

       // if self.mpgge == 0:
       //     # hardcoded conversion
       //     self.gallons_gas_equivalent_per_mile = self.electric_kwh_per_mi / self.props.kwh_per_gge
       //     grid_gallons_gas_equivalent_per_mile = self.electric_kwh_per_mi / self.props.kwh_per_gge / \
       //         self.veh.chg_eff

       // else:
       //     self.gallons_gas_equivalent_per_mile = 1 / \
       //         self.mpgge + self.electric_kwh_per_mi  / self.props.kwh_per_gge
       //     grid_gallons_gas_equivalent_per_mile = 1 / self.mpgge + \
       //         self.electric_kwh_per_mi / self.props.kwh_per_gge / self.veh.chg_eff

       // self.grid_mpgge_elec = 1 / grid_gallons_gas_equivalent_per_mile
       // self.mpgge_elec = 1 / self.gallons_gas_equivalent_per_mile

       // # energy audit calcs
       // self.drag_kw = self.cyc_drag_kw 
       // self.drag_kj = (self.drag_kw * self.cyc.dt_s).sum()
       // self.ascent_kw = self.cyc_ascent_kw
       // self.ascent_kj = (self.ascent_kw * self.cyc.dt_s).sum()
       // self.rr_kw = self.cyc_rr_kw
       // self.rr_kj = (self.rr_kw * self.cyc.dt_s).sum()

       // self.ess_loss_kw[1:] = np.array(
       //     [0 if (self.veh.max_ess_kw == 0 or self.veh.max_ess_kwh == 0)
       //     else -self.ess_kw_out_ach[i] - (-self.ess_kw_out_ach[i] * np.sqrt(self.veh.ess_round_trip_eff))
       //         if self.ess_kw_out_ach[i] < 0
       //     else self.ess_kw_out_ach[i] * (1.0 / np.sqrt(self.veh.ess_round_trip_eff)) - self.ess_kw_out_ach[i]
       //     for i in range(1, len(self.cyc.time_s))]
       // )
       // 
       // self.brake_kj = (self.cyc_fric_brake_kw * self.cyc.dt_s).sum()
       // self.trans_kj = ((self.trans_kw_in_ach - self.trans_kw_out_ach) * self.cyc.dt_s).sum()
       // self.mc_kj = ((self.mc_elec_kw_in_ach - self.mc_mech_kw_out_ach) * self.cyc.dt_s).sum()
       // self.ess_eff_kj = (self.ess_loss_kw * self.cyc.dt_s).sum()
       // self.aux_kj = (self.aux_in_kw * self.cyc.dt_s).sum()
       // self.fc_kj = ((self.fc_kw_in_ach - self.fc_kw_out_ach) * self.cyc.dt_s).sum()
       // 
       // self.net_kj = self.drag_kj + self.ascent_kj + self.rr_kj + self.brake_kj + self.trans_kj \
       //     + self.mc_kj + self.ess_eff_kj + self.aux_kj + self.fc_kj

       // self.ke_kj = 0.5 * self.veh.veh_kg * (self.mps_ach[0] ** 2 - self.mps_ach[-1] ** 2) / 1_000
       // 
       // self.energyAuditError = ((self.roadway_chg_kj + self.ess_dischg_kj + self.fuel_kj + self.ke_kj) - self.net_kj
       //     ) / (self.roadway_chg_kj + self.ess_dischg_kj + self.fuel_kj + self.ke_kj)

       // if (np.abs(self.energyAuditError) > self.sim_params.energy_audit_error_tol) and \
       //     self.sim_params.verbose:
       //     print('Warning: There is a problem with conservation of energy.')
       //     print('Energy Audit Error:', np.round(self.energyAuditError, 5))

       // self.accel_kw[1:] = (self.veh.veh_kg / (2.0 * (self.cyc.dt_s[1:]))) * (
       //     self.mps_ach[1:] ** 2 - self.mps_ach[:-1] ** 2) / 1_000

       // self.trace_miss = False
       // self.trace_miss_dist_frac = abs(self.dist_m.sum() - self.cyc0.dist_m.sum()) / self.cyc0.dist_m.sum()
       // self.trace_miss_time_frac = abs(self.cyc.time_s[-1] - self.cyc0.time_s[-1]) / self.cyc0.time_s[-1]

       // if not(self.sim_params.missed_trace_correction):
       //     if self.trace_miss_dist_frac > self.sim_params.trace_miss_dist_tol:
       //         self.trace_miss = True
       //         if self.sim_params.verbose:
       //             print('Warning: Trace miss distance fraction:', np.round(self.trace_miss_dist_frac, 5))
       //             print('exceeds tolerance of: ', np.round(self.sim_params.trace_miss_dist_tol, 5))
       // else:
       //     if self.trace_miss_time_frac > self.sim_params.trace_miss_time_tol:
       //         self.trace_miss = True
       //         if self.sim_params.verbose:
       //             print('Warning: Trace miss time fraction:', np.round(self.trace_miss_time_frac, 5))
       //             print('exceeds tolerance of: ', np.round(self.sim_params.trace_miss_time_tol, 5))

       // self.trace_miss_speed_mps = max([
       //     abs(self.mps_ach[i] - self.cyc.mps[i]) for i in range(len(self.cyc.time_s))
       // ])
       // if self.trace_miss_speed_mps > self.sim_params.trace_miss_speed_mps_tol:
       //     self.trace_miss = True
       //     if self.sim_params.verbose:
       //         print('Warning: Trace miss speed [m/s]:', np.round(self.trace_miss_speed_mps, 5))
       //         print('exceeds tolerance of: ', np.round(self.sim_params.trace_miss_speed_mps_tol, 5))
    }
    // Methods for getting and setting arrays and other complex fields
    // note that python cannot specify a specific index to set but must reset the entire array

    fn len(&self) -> usize {
        self.cyc.time_s.len()
    }

    #[getter]
    pub fn get_veh(&self) -> PyResult<RustVehicle>{
        Ok(self.veh.clone())
    }
    #[setter]
    pub fn set_veh(&mut self, new_value:RustVehicle) -> PyResult<()>{
        self.veh = new_value;
        Ok(())
    }

    #[getter]
    pub fn get_cyc(&self) -> PyResult<RustCycle>{
        Ok(self.cyc.clone())
    }
    #[setter]
    pub fn set_cyc(&mut self, new_value:RustCycle) -> PyResult<()>{
        self.cyc = new_value;
        Ok(())
    }

    #[getter]
    pub fn get_cyc0(&self) -> PyResult<RustCycle>{
        Ok(self.cyc0.clone())
    }
    #[setter]
    pub fn set_cyc0(&mut self, new_value:RustCycle) -> PyResult<()>{
        self.cyc0 = new_value;
        Ok(())
    }

    #[getter]
    pub fn get_sim_params(&self) -> PyResult<RustSimDriveParams>{
        Ok(self.sim_params.clone())
    }
    #[setter]
    pub fn set_sim_params(&mut self, new_value:RustSimDriveParams) -> PyResult<()>{
        self.sim_params = new_value;
        Ok(())
    }

    #[getter]
    pub fn get_props(&self) -> PyResult<RustPhysicalProperties>{
        Ok(self.props.clone())
    }
    #[setter]
    pub fn set_props(&mut self, new_value:RustPhysicalProperties) -> PyResult<()>{
        self.props = new_value;
        Ok(())
    }

    #[getter]
    pub fn get_accel_buff_soc(&self) -> PyResult<Vec<f64>>{
      Ok(self.accel_buff_soc.to_vec())
    }
    #[setter]
    pub fn set_accel_buff_soc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.accel_buff_soc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_accel_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.accel_kw.to_vec())
    }
    #[setter]
    pub fn set_accel_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.accel_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_add_kwh(&self) -> PyResult<Vec<f64>>{
      Ok(self.add_kwh.to_vec())
    }
    #[setter]
    pub fn set_add_kwh(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.add_kwh = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ascent_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ascent_kw.to_vec())
    }
    #[setter]
    pub fn set_ascent_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ascent_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_aux_in_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.aux_in_kw.to_vec())
    }
    #[setter]
    pub fn set_aux_in_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.aux_in_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_can_pwr_all_elec(&self) -> PyResult<Vec<bool>>{
      Ok(self.can_pwr_all_elec.to_vec())
    }
    #[setter]
    pub fn set_can_pwr_all_elec(&mut self, new_value:Vec<bool>) -> PyResult<()>{
      self.can_pwr_all_elec = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_avail_elec_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_avail_elec_kw.to_vec())
    }
    #[setter]
    pub fn set_cur_max_avail_elec_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_avail_elec_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_elec_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_elec_kw.to_vec())
    }
    #[setter]
    pub fn set_cur_max_elec_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_elec_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_ess_chg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_ess_chg_kw.to_vec())
    }
    #[setter]
    pub fn set_cur_max_ess_chg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_ess_chg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_ess_kw_out(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_ess_kw_out.to_vec())
    }
    #[setter]
    pub fn set_cur_max_ess_kw_out(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_ess_kw_out = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_fc_kw_out(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_fc_kw_out.to_vec())
    }
    #[setter]
    pub fn set_cur_max_fc_kw_out(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_fc_kw_out = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_fs_kw_out(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_fs_kw_out.to_vec())
    }
    #[setter]
    pub fn set_cur_max_fs_kw_out(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_fs_kw_out = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_mc_elec_kw_in(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_mc_elec_kw_in.to_vec())
    }
    #[setter]
    pub fn set_cur_max_mc_elec_kw_in(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_mc_elec_kw_in = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_mc_kw_out(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_mc_kw_out.to_vec())
    }
    #[setter]
    pub fn set_cur_max_mc_kw_out(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_mc_kw_out = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_mech_mc_kw_in(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_mech_mc_kw_in.to_vec())
    }
    #[setter]
    pub fn set_cur_max_mech_mc_kw_in(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_mech_mc_kw_in = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_roadway_chg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_roadway_chg_kw.to_vec())
    }
    #[setter]
    pub fn set_cur_max_roadway_chg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_roadway_chg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_trac_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_trac_kw.to_vec())
    }
    #[setter]
    pub fn set_cur_max_trac_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_trac_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_max_trans_kw_out(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_max_trans_kw_out.to_vec())
    }
    #[setter]
    pub fn set_cur_max_trans_kw_out(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_max_trans_kw_out = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cur_soc_target(&self) -> PyResult<Vec<f64>>{
      Ok(self.cur_soc_target.to_vec())
    }
    #[setter]
    pub fn set_cur_soc_target(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cur_soc_target = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_accel_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_accel_kw.to_vec())
    }
    #[setter]
    pub fn set_cyc_accel_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_accel_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_ascent_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_ascent_kw.to_vec())
    }
    #[setter]
    pub fn set_cyc_ascent_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_ascent_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_drag_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_drag_kw.to_vec())
    }
    #[setter]
    pub fn set_cyc_drag_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_drag_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_fric_brake_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_fric_brake_kw.to_vec())
    }
    #[setter]
    pub fn set_cyc_fric_brake_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_fric_brake_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_met(&self) -> PyResult<Vec<bool>>{
      Ok(self.cyc_met.to_vec())
    }
    #[setter]
    pub fn set_cyc_met(&mut self, new_value:Vec<bool>) -> PyResult<()>{
      self.cyc_met = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_regen_brake_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_regen_brake_kw.to_vec())
    }
    #[setter]
    pub fn set_cyc_regen_brake_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_regen_brake_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_rr_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_rr_kw.to_vec())
    }
    #[setter]
    pub fn set_cyc_rr_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_rr_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_tire_inertia_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_tire_inertia_kw.to_vec())
    }
    #[setter]
    pub fn set_cyc_tire_inertia_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_tire_inertia_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_trac_kw_req(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_trac_kw_req.to_vec())
    }
    #[setter]
    pub fn set_cyc_trac_kw_req(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_trac_kw_req = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_trans_kw_out_req(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_trans_kw_out_req.to_vec())
    }
    #[setter]
    pub fn set_cyc_trans_kw_out_req(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_trans_kw_out_req = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_whl_kw_req(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_whl_kw_req.to_vec())
    }
    #[setter]
    pub fn set_cyc_whl_kw_req(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_whl_kw_req = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_cyc_whl_rad_per_sec(&self) -> PyResult<Vec<f64>>{
      Ok(self.cyc_whl_rad_per_sec.to_vec())
    }
    #[setter]
    pub fn set_cyc_whl_rad_per_sec(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.cyc_whl_rad_per_sec = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_desired_ess_kw_out_for_ae(&self) -> PyResult<Vec<f64>>{
      Ok(self.desired_ess_kw_out_for_ae.to_vec())
    }
    #[setter]
    pub fn set_desired_ess_kw_out_for_ae(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.desired_ess_kw_out_for_ae = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_dist_m(&self) -> PyResult<Vec<f64>>{
      Ok(self.dist_m.to_vec())
    }
    #[setter]
    pub fn set_dist_m(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.dist_m = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_dist_mi(&self) -> PyResult<Vec<f64>>{
      Ok(self.dist_mi.to_vec())
    }
    #[setter]
    pub fn set_dist_mi(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.dist_mi = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_dod_cycs(&self) -> PyResult<Vec<f64>>{
      Ok(self.dod_cycs.to_vec())
    }
    #[setter]
    pub fn set_dod_cycs(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.dod_cycs = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_drag_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.drag_kw.to_vec())
    }
    #[setter]
    pub fn set_drag_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.drag_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_elec_kw_req_4ae(&self) -> PyResult<Vec<f64>>{
      Ok(self.elec_kw_req_4ae.to_vec())
    }
    #[setter]
    pub fn set_elec_kw_req_4ae(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.elec_kw_req_4ae = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_er_ae_kw_out(&self) -> PyResult<Vec<f64>>{
      Ok(self.er_ae_kw_out.to_vec())
    }
    #[setter]
    pub fn set_er_ae_kw_out(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.er_ae_kw_out = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_er_kw_if_fc_req(&self) -> PyResult<Vec<f64>>{
      Ok(self.er_kw_if_fc_req.to_vec())
    }
    #[setter]
    pub fn set_er_kw_if_fc_req(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.er_kw_if_fc_req = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_accel_buff_chg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_accel_buff_chg_kw.to_vec())
    }
    #[setter]
    pub fn set_ess_accel_buff_chg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_accel_buff_chg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_accel_regen_dischg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_accel_regen_dischg_kw.to_vec())
    }
    #[setter]
    pub fn set_ess_accel_regen_dischg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_accel_regen_dischg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_ae_kw_out(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_ae_kw_out.to_vec())
    }
    #[setter]
    pub fn set_ess_ae_kw_out(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_ae_kw_out = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_cap_lim_chg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_cap_lim_chg_kw.to_vec())
    }
    #[setter]
    pub fn set_ess_cap_lim_chg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_cap_lim_chg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_cap_lim_dischg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_cap_lim_dischg_kw.to_vec())
    }
    #[setter]
    pub fn set_ess_cap_lim_dischg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_cap_lim_dischg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_cur_kwh(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_cur_kwh.to_vec())
    }
    #[setter]
    pub fn set_ess_cur_kwh(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_cur_kwh = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_desired_kw_4fc_eff(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_desired_kw_4fc_eff.to_vec())
    }
    #[setter]
    pub fn set_ess_desired_kw_4fc_eff(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_desired_kw_4fc_eff = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_kw_if_fc_req(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_kw_if_fc_req.to_vec())
    }
    #[setter]
    pub fn set_ess_kw_if_fc_req(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_kw_if_fc_req = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_kw_out_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_kw_out_ach.to_vec())
    }
    #[setter]
    pub fn set_ess_kw_out_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_kw_out_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_lim_mc_regen_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_lim_mc_regen_kw.to_vec())
    }
    #[setter]
    pub fn set_ess_lim_mc_regen_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_lim_mc_regen_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_lim_mc_regen_perc_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_lim_mc_regen_perc_kw.to_vec())
    }
    #[setter]
    pub fn set_ess_lim_mc_regen_perc_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_lim_mc_regen_perc_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_loss_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_loss_kw.to_vec())
    }
    #[setter]
    pub fn set_ess_loss_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_loss_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_perc_dead(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_perc_dead.to_vec())
    }
    #[setter]
    pub fn set_ess_perc_dead(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_perc_dead = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_ess_regen_buff_dischg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.ess_regen_buff_dischg_kw.to_vec())
    }
    #[setter]
    pub fn set_ess_regen_buff_dischg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.ess_regen_buff_dischg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_forced_on(&self) -> PyResult<Vec<bool>>{
      Ok(self.fc_forced_on.to_vec())
    }
    #[setter]
    pub fn set_fc_forced_on(&mut self, new_value:Vec<bool>) -> PyResult<()>{
      self.fc_forced_on = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_forced_state(&self) -> PyResult<Vec<u32>>{
      Ok(self.fc_forced_state.to_vec())
    }
    #[setter]
    pub fn set_fc_forced_state_arr(&mut self, new_value:Vec<u32>) -> PyResult<()>{
      self.fc_forced_state = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_fs_lim_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_fs_lim_kw.to_vec())
    }
    #[setter]
    pub fn set_fc_fs_lim_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_fs_lim_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_kw_gap_fr_eff(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_kw_gap_fr_eff.to_vec())
    }
    #[setter]
    pub fn set_fc_kw_gap_fr_eff(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_kw_gap_fr_eff = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_kw_in_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_kw_in_ach.to_vec())
    }
    #[setter]
    pub fn set_fc_kw_in_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_kw_in_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_kw_out_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_kw_out_ach.to_vec())
    }
    #[setter]
    pub fn set_fc_kw_out_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_kw_out_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_kw_out_ach_pct(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_kw_out_ach_pct.to_vec())
    }
    #[setter]
    pub fn set_fc_kw_out_ach_pct(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_kw_out_ach_pct = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_max_kw_in(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_max_kw_in.to_vec())
    }
    #[setter]
    pub fn set_fc_max_kw_in(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_max_kw_in = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_time_on(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_time_on.to_vec())
    }
    #[setter]
    pub fn set_fc_time_on(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_time_on = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fc_trans_lim_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.fc_trans_lim_kw.to_vec())
    }
    #[setter]
    pub fn set_fc_trans_lim_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fc_trans_lim_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fs_cumu_mj_out_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.fs_cumu_mj_out_ach.to_vec())
    }
    #[setter]
    pub fn set_fs_cumu_mj_out_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fs_cumu_mj_out_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fs_kw_out_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.fs_kw_out_ach.to_vec())
    }
    #[setter]
    pub fn set_fs_kw_out_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fs_kw_out_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fs_kwh_out_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.fs_kwh_out_ach.to_vec())
    }
    #[setter]
    pub fn set_fs_kwh_out_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.fs_kwh_out_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_hev_sim_count(&self) -> PyResult<usize>{
      Ok(self.hev_sim_count)
    }
    #[setter]
    pub fn set_hev_sim_count(&mut self, new_value:usize) -> PyResult<()>{
      self.hev_sim_count = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_high_acc_fc_on_tag(&self) -> PyResult<Vec<bool>>{
      Ok(self.high_acc_fc_on_tag.to_vec())
    }
    #[setter]
    pub fn set_high_acc_fc_on_tag(&mut self, new_value:Vec<bool>) -> PyResult<()>{
      self.high_acc_fc_on_tag = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_i(&self) -> PyResult<usize>{
      Ok(self.i)
    }
    #[setter]
    pub fn set_i(&mut self, new_value:usize) -> PyResult<()>{
      self.i = new_value;
      Ok(())
    }

    #[getter]
    pub fn get_max_ess_accell_buff_dischg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.max_ess_accell_buff_dischg_kw.to_vec())
    }
    #[setter]
    pub fn set_max_ess_accell_buff_dischg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.max_ess_accell_buff_dischg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_max_ess_regen_buff_chg_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.max_ess_regen_buff_chg_kw.to_vec())
    }
    #[setter]
    pub fn set_max_ess_regen_buff_chg_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.max_ess_regen_buff_chg_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_max_trac_mps(&self) -> PyResult<Vec<f64>>{
      Ok(self.max_trac_mps.to_vec())
    }
    #[setter]
    pub fn set_max_trac_mps(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.max_trac_mps = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_elec_in_kw_for_max_fc_eff(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_elec_in_kw_for_max_fc_eff.to_vec())
    }
    #[setter]
    pub fn set_mc_elec_in_kw_for_max_fc_eff(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_elec_in_kw_for_max_fc_eff = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_elec_in_lim_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_elec_in_lim_kw.to_vec())
    }
    #[setter]
    pub fn set_mc_elec_in_lim_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_elec_in_lim_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_elec_kw_in_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_elec_kw_in_ach.to_vec())
    }
    #[setter]
    pub fn set_mc_elec_kw_in_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_elec_kw_in_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_elec_kw_in_if_fc_req(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_elec_kw_in_if_fc_req.to_vec())
    }
    #[setter]
    pub fn set_mc_elec_kw_in_if_fc_req(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_elec_kw_in_if_fc_req = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_kw_if_fc_req(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_kw_if_fc_req.to_vec())
    }
    #[setter]
    pub fn set_mc_kw_if_fc_req(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_kw_if_fc_req = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_mech_kw_4forced_fc(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_mech_kw_4forced_fc.to_vec())
    }
    #[setter]
    pub fn set_mc_mech_kw_4forced_fc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_mech_kw_4forced_fc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_mech_kw_out_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_mech_kw_out_ach.to_vec())
    }
    #[setter]
    pub fn set_mc_mech_kw_out_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_mech_kw_out_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mc_transi_lim_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.mc_transi_lim_kw.to_vec())
    }
    #[setter]
    pub fn set_mc_transi_lim_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mc_transi_lim_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_min_ess_kw_2help_fc(&self) -> PyResult<Vec<f64>>{
      Ok(self.min_ess_kw_2help_fc.to_vec())
    }
    #[setter]
    pub fn set_min_ess_kw_2help_fc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.min_ess_kw_2help_fc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_min_mc_kw_2help_fc(&self) -> PyResult<Vec<f64>>{
      Ok(self.min_mc_kw_2help_fc.to_vec())
    }
    #[setter]
    pub fn set_min_mc_kw_2help_fc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.min_mc_kw_2help_fc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mph_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.mph_ach.to_vec())
    }
    #[setter]
    pub fn set_mph_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mph_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_mps_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.mps_ach.to_vec())
    }
    #[setter]
    pub fn set_mps_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.mps_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_newton_iters(&self) -> PyResult<Vec<u32>>{
      Ok(self.newton_iters.to_vec())
    }
    #[setter]
    pub fn set_newton_iters(&mut self, new_value:Vec<u32>) -> PyResult<()>{
      self.newton_iters = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_prev_fc_time_on(&self) -> PyResult<Vec<f64>>{
      Ok(self.prev_fc_time_on.to_vec())
    }
    #[setter]
    pub fn set_prev_fc_time_on(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.prev_fc_time_on = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_reached_buff(&self) -> PyResult<Vec<bool>>{
      Ok(self.reached_buff.to_vec())
    }
    #[setter]
    pub fn set_reached_buff(&mut self, new_value:Vec<bool>) -> PyResult<()>{
      self.reached_buff = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_regen_buff_soc(&self) -> PyResult<Vec<f64>>{
      Ok(self.regen_buff_soc.to_vec())
    }
    #[setter]
    pub fn set_regen_buff_soc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.regen_buff_soc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_regen_contrl_lim_kw_perc(&self) -> PyResult<Vec<f64>>{
      Ok(self.regen_contrl_lim_kw_perc.to_vec())
    }
    #[setter]
    pub fn set_regen_contrl_lim_kw_perc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.regen_contrl_lim_kw_perc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_roadway_chg_kw_out_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.roadway_chg_kw_out_ach.to_vec())
    }
    #[setter]
    pub fn set_roadway_chg_kw_out_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.roadway_chg_kw_out_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_rr_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.rr_kw.to_vec())
    }
    #[setter]
    pub fn set_rr_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.rr_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_soc(&self) -> PyResult<Vec<f64>>{
      Ok(self.soc.to_vec())
    }
    #[setter]
    pub fn set_soc(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.soc = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_spare_trac_kw(&self) -> PyResult<Vec<f64>>{
      Ok(self.spare_trac_kw.to_vec())
    }
    #[setter]
    pub fn set_spare_trac_kw(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.spare_trac_kw = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_trace_miss_iters(&self) -> PyResult<Vec<f64>>{
      Ok(self.trace_miss_iters.to_vec())
    }
    #[setter]
    pub fn set_trace_miss_iters(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.trace_miss_iters = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_trans_kw_in_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.trans_kw_in_ach.to_vec())
    }
    #[setter]
    pub fn set_trans_kw_in_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.trans_kw_in_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_trans_kw_out_ach(&self) -> PyResult<Vec<f64>>{
      Ok(self.trans_kw_out_ach.to_vec())
    }
    #[setter]
    pub fn set_trans_kw_out_ach(&mut self, new_value:Vec<f64>) -> PyResult<()>{
      self.trans_kw_out_ach = Array::from_vec(new_value);
      Ok(())
    }

    #[getter]
    pub fn get_fuel_kj(&self) -> PyResult<f64>{
      Ok(self.fuel_kj)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sim_drive_walk() {
//         // CYCLE
//         let cyc = RustCycle::test_cyc();
//         let cycle_length = cyc.len();

//         // VEHICLE

//         let veh = RustVehicle::test_veh();

//         // SIM DRIVE
//         let mut sd = RustSimDrive::__new__(cyc, veh);
//         let init_soc: f64 = 0.5;
//         sd.sim_drive_walk(init_soc);

//         let expected_final_i: usize = cycle_length;
//         assert_eq!(sd.i, expected_final_i);
//     }
// }