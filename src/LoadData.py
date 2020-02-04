"""Module containing classes for loading drive cycle data (e.g. speed trace)
and vehicle attribute data.  For example usage, 
see ../README.md"""

import pandas as pd
from Globals import *
import numpy as np
import re
from numba import jitclass                 # import the decorator
from numba import float64, int32, bool_    # import the types
from numba import types

class Cycle(object):
    """Object for containing time, speed, road grade, and road charging vectors 
    for drive cycle."""
    def __init__(self, std_cyc_name=None, cyc_dict=None):
        """Runs other methods, depending on provided keyword argument. Only one keyword
        argument should be provided.  Keyword arguments are identical to 
        arguments required by corresponding methods.  The argument 'std_cyc_name' can be
        optionally passed as a positional argument."""
        super().__init__()
        if std_cyc_name:
            self.set_standard_cycle(std_cyc_name)
        if cyc_dict:
            self.set_from_dict(cyc_dict)
        
    def get_numba_cyc(self):
        """Returns numba jitclass version of Cycle object."""
        numba_cyc = TypedCycle(len(self.cycSecs))
        for key in self.__dict__.keys():
            numba_cyc.__setattr__(key, self.__getattribute__(key).astype(np.float64))
        return numba_cyc

    def set_standard_cycle(self, std_cyc_name):
        """Load time trace of speed, grade, and road type in a pandas dataframe.
        Argument:
        ---------
        std_cyc_name: cycle name string (e.g. 'udds', 'us06', 'hwfet')"""
        csv_path = '..//cycles//' + std_cyc_name + '.csv'
        cyc = pd.read_csv(csv_path)
        for column in cyc.columns:
            self.__setattr__(column, cyc[column].to_numpy())
        self.set_dependents()

    def set_from_dict(self, cyc_dict):
        """Set cycle attributes from dict with keys 'cycGrade', 'cycMps', 'cycSecs', 'cycRoadType'
        and numpy arrays of equal length for values.
        Arguments
        ---------
        cyc_dict: dict containing cycle data
        """

        for key in cyc_dict.keys():
            self.__setattr__(key, cyc_dict[key])
        self.set_dependents()
    
    def set_dependents(self):
        """Sets values dependent on cycle info loaded from file."""
        self.cycMph = self.cycMps * mphPerMps
        self.secs = np.insert(np.diff(self.cycSecs), 0, 0)

# type specifications for attributes of Cycle class
cyc_spec = [('cycSecs', float64[:]),
            ('cycMps', float64[:]),
            ('cycGrade', float64[:]),
            ('cycRoadType', float64[:]),
            ('cycMph', float64[:]),
            ('secs', float64[:])
]

@jitclass(cyc_spec)
class TypedCycle(object):
    """Just-in-time compiled version of Cycle using numba."""
    def __init__(self, len_cyc):
        """This method initialized type numpy arrays as required by 
        numba jitclass."""
        self.cycSecs = np.zeros(len_cyc, dtype=np.float64)
        self.cycMps = np.zeros(len_cyc, dtype=np.float64)
        self.cycGrade = np.zeros(len_cyc, dtype=np.float64)
        self.cycRoadType = np.zeros(len_cyc, dtype=np.float64)
        self.cycMph = np.zeros(len_cyc, dtype=np.float64)
        self.secs = np.zeros(len_cyc, dtype=np.float64)

class Vehicle(object):
    """Class for loading and contaning vehicle attributes
    Optional Argument:
    ---------
    vnum: row number of vehicle to simulate in 'FASTSim_py_veh_db.csv'"""

    def __init__(self, vnum=None):
        super().__init__()
        if vnum:
            self.load_vnum(vnum)

    def get_numba_veh(self):
        if 'numba_veh' not in self.__dict__:
            self.numba_veh = TypedVehicle()
        for item in veh_spec:
            if (type(self.__getattribute__(item[0])) == np.ndarray) | (type(self.__getattribute__(item[0])) == np.float64):
                self.numba_veh.__setattr__(item[0], self.__getattribute__(item[0]).astype(np.float64))
            elif type(self.__getattribute__(item[0])) == np.int64:
                self.numba_veh.__setattr__(item[0], self.__getattribute__(item[0]).astype(np.int32))
            else:
                self.numba_veh.__setattr__(
                    item[0], self.__getattribute__(item[0]))
            
        return self.numba_veh
    
    def load_vnum(self, vnum):
        """Load vehicle parameters based on vnum and assign to self.
        Argument:
        ---------
        vnum: row number of vehicle to simulate in 'FASTSim_py_veh_db.csv'"""

        vehdf = pd.read_csv('..//docs//FASTSim_py_veh_db.csv')
        vehdf.set_index('Selection', inplace=True, drop=False)

        def clean_data(raw_data):
            """Cleans up data formatting.
            Argument:
            ------------
            raw_data: cell of vehicle dataframe
            
            Output:
            clean_data: cleaned up data"""
            
            # convert data to string types
            data = str(raw_data)
            # remove percent signs if any are found
            if '%' in data:
                data = data.replace('%', '')
                data = float(data)
                data = data / 100.0
            # replace string for TRUE with Boolean True
            elif re.search('(?i)true', data) != None:
                data = True
            # replace string for FALSE with Boolean False
            elif re.search('(?i)false', data) != None:
                data = False
            else:
                try:
                    data = float(data)
                except:
                    pass
            
            return data
        
        vehdf.loc[vnum].apply(clean_data)

        ### selects specified vnum from vehdf
        for col in vehdf.columns:
            col1 = col.replace(' ', '_')
            
            # assign dataframe columns 
            self.__setattr__(col1, vehdf.loc[vnum, col])
        
        self.set_init_calcs()
        self.set_veh_mass()

    def set_init_calcs(self):
        """Set parameters that can be calculated after loading vehicle data"""
        ### Build roadway power lookup table
        self.MaxRoadwayChgKw = np.zeros(6)
        self.chargingOn = False

        # Checking if a vehicle has any hybrid components
        if self.maxEssKwh == 0 or self.maxEssKw == 0 or self.maxMotorKw == 0:
            self.noElecSys = True

        else:
            self.noElecSys = False

        # Checking if aux loads go through an alternator
        if self.noElecSys == True or self.maxMotorKw <= self.auxKw or self.forceAuxOnFC == True:
            self.noElecAux = True

        else:
            self.noElecAux = False

        # Copying vehPtType to additional key
        self.vehTypeSelection = self.vehPtType
        # to be consistent with Excel version but not used in Python version

        ### Defining Fuel Converter efficiency curve as lookup table for %power_in vs power_out
        ### see "FC Model" tab in FASTSim for Excel

        # Power and efficiency arrays are defined in Globals.py
        
        if self.fcEffType == 1:  # SI engine
            eff = eff_si + self.fcAbsEffImpr

        elif self.fcEffType == 2:  # Atkinson cycle SI engine -- greater expansion
            eff = eff_atk + self.fcAbsEffImpr

        elif self.fcEffType == 3:  # Diesel (compression ignition) engine
            eff = eff_diesel + self.fcAbsEffImpr

        elif self.fcEffType == 4:  # H2 fuel cell
            eff = eff_fuel_cell + self.fcAbsEffImpr

        elif self.fcEffType == 5:  # heavy duty Diesel engine
            eff = eff_hd_diesel + self.fcAbsEffImpr

        # discrete array of possible engine power outputs
        inputKwOutArray = fcPwrOutPerc * self.maxFuelConvKw
        # Relatively continuous array of possible engine power outputs
        fcKwOutArray = self.maxFuelConvKw * fcPercOutArray
        # Initializes relatively continuous array for fcEFF
        fcEffArray = np.zeros(len(fcPercOutArray))

        # the following for loop populates fcEffArray
        for j in range(0, len(fcPercOutArray) - 1):
            low_index = np.argmax(inputKwOutArray >= fcKwOutArray[j])
            fcinterp_x_1 = inputKwOutArray[low_index-1]
            fcinterp_x_2 = inputKwOutArray[low_index]
            fcinterp_y_1 = eff[low_index-1]
            fcinterp_y_2 = eff[low_index]
            fcEffArray[j] = (fcKwOutArray[j] - fcinterp_x_1)/(fcinterp_x_2 -
                                fcinterp_x_1) * (fcinterp_y_2 - fcinterp_y_1) + fcinterp_y_1

        # populate final value
        fcEffArray[-1] = eff[-1]

        # assign corresponding values in veh dict
        self.fcEffArray = fcEffArray
        self.fcKwOutArray = fcKwOutArray
        self.maxFcEffKw = self.fcKwOutArray[np.argmax(fcEffArray)]
        self.fcMaxOutkW = np.max(inputKwOutArray)
            
        ### Defining MC efficiency curve as lookup table for %power_in vs power_out
        ### see "Motor" tab in FASTSim for Excel

        maxMotorKw = self.maxMotorKw
        
        # Power and efficiency arrays are defined in Globals.py

        modern_diff = modern_max - max(large_baseline_eff)

        large_baseline_eff_adj = large_baseline_eff + modern_diff

        mcKwAdjPerc = max(0.0, min((maxMotorKw - 7.5)/(75.0 - 7.5), 1.0))
        mcEffArray = np.zeros(len(mcPwrOutPerc))

        for k in range(0, len(mcPwrOutPerc)):
            mcEffArray[k] = mcKwAdjPerc * large_baseline_eff_adj[k] + \
                (1 - mcKwAdjPerc)*(small_baseline_eff[k])

        mcInputKwOutArray = mcPwrOutPerc * maxMotorKw
        mcFullEffArray = np.zeros(len(mcPercOutArray))
        mcKwOutArray = np.linspace(0, 1, len(mcPercOutArray)) * maxMotorKw

        for m in range(1, len(mcPercOutArray) - 1):
            low_index = np.argmax(mcInputKwOutArray >= mcKwOutArray[m])

            fcinterp_x_1 = mcInputKwOutArray[low_index-1]
            fcinterp_x_2 = mcInputKwOutArray[low_index]
            fcinterp_y_1 = mcEffArray[low_index-1]
            fcinterp_y_2 = mcEffArray[low_index]

            mcFullEffArray[m] = (mcKwOutArray[m] - fcinterp_x_1)/(
                fcinterp_x_2 - fcinterp_x_1)*(fcinterp_y_2 - fcinterp_y_1) + fcinterp_y_1

        mcFullEffArray[0] = 0
        mcFullEffArray[-1] = mcEffArray[-1]

        mcKwInArray = mcKwOutArray / mcFullEffArray
        mcKwInArray[0] = 0

        self.mcKwInArray = mcKwInArray
        self.mcKwOutArray = mcKwOutArray
        self.mcMaxElecInKw = max(mcKwInArray)
        self.mcFullEffArray = mcFullEffArray
        self.mcEffArray = mcEffArray

        if 'motorAccelAssist' in self.__dir__() and np.isnan(self.__getattribute__('motorAccelAssist')):
            self.motorAccelAssist = True

        self.mcMaxElecInKw = max(self.mcKwInArray)

        ### Specify shape of mc regen efficiency curve
        ### see "Regen" tab in FASTSim for Excel
        self.regenA = 500.0  # hardcoded
        self.regenB = 0.99  # hardcoded

    def set_veh_mass(self):
        """Calculate total vehicle mass.  Sum up component masses if 
        positive real number is not specified for vehOverrideKg"""
        if not(self.vehOverrideKg > 0):
            if self.maxEssKwh == 0 or self.maxEssKw == 0:
                ess_mass_kg = 0.0
            else:
                ess_mass_kg = ((self.maxEssKwh * self.essKgPerKwh) +
                            self.essBaseKg) * self.compMassMultiplier
            if self.maxMotorKw == 0:
                mc_mass_kg = 0.0
            else:
                mc_mass_kg = (self.mcPeBaseKg+(self.mcPeKgPerKw
                                                * self.maxMotorKw)) * self.compMassMultiplier
            if self.maxFuelConvKw == 0:
                fc_mass_kg = 0.0
            else:
                fc_mass_kg = (((1 / self.fuelConvKwPerKg) * self.maxFuelConvKw +
                            self.fuelConvBaseKg)) * self.compMassMultiplier
            if self.maxFuelStorKw == 0:
                fs_mass_kg = 0.0
            else:
                fs_mass_kg = ((1 / self.fuelStorKwhPerKg) *
                            self.fuelStorKwh) * self.compMassMultiplier
            self.vehKg = self.cargoKg + self.gliderKg + self.transKg * \
                self.compMassMultiplier + ess_mass_kg + \
                mc_mass_kg + fc_mass_kg + fs_mass_kg
        # if positive real number is specified for vehOverrideKg, use that
        else:
            self.vehKg = self.vehOverrideKg

        self.maxTracMps2 = ((((self.wheelCoefOfFric * self.driveAxleWeightFrac * self.vehKg * gravityMPerSec2) /
                              (1+((self.vehCgM * self.wheelCoefOfFric) / self.wheelBaseM))))/(self.vehKg * gravityMPerSec2)) * gravityMPerSec2
        self.maxRegenKwh = 0.5 * self.vehKg * (27**2) / (3600 * 1000)

# type specifications for attributions of Vehcile class
veh_spec = [('Selection', int32),
    ('Scenario_name', types.unicode_type),
    ('vehPtType', int32),
    ('dragCoef', float64),
    ('frontalAreaM2', float64),
    ('gliderKg', float64),
    ('vehCgM', float64),
    ('driveAxleWeightFrac', float64),
    ('wheelBaseM', float64),
    ('cargoKg', float64),
    ('vehOverrideKg', float64),
    ('maxFuelStorKw', float64),
    ('fuelStorSecsToPeakPwr', float64),
    ('fuelStorKwh', float64),
    ('fuelStorKwhPerKg', float64),
    ('maxFuelConvKw', float64),
    ('fcEffType', int32),
    ('fcAbsEffImpr', float64),
    ('fuelConvSecsToPeakPwr', float64),
    ('fuelConvBaseKg', float64),
    ('fuelConvKwPerKg', float64),
    ('maxMotorKw', float64),
    ('motorPeakEff', float64),
    ('motorSecsToPeakPwr', float64),
    ('stopStart', bool_),
    ('mcPeKgPerKw', float64),
    ('mcPeBaseKg', float64),
    ('maxEssKw', float64),
    ('maxEssKwh', float64),
    ('essKgPerKwh', float64),
    ('essBaseKg', float64),
    ('essRoundTripEff', float64),
    ('essLifeCoefA', float64),
    ('essLifeCoefB', float64),
    ('wheelInertiaKgM2', float64),
    ('numWheels', float64),
    ('wheelRrCoef', float64),
    ('wheelRadiusM', float64),
    ('wheelCoefOfFric', float64),
    ('minSoc', float64),
    ('maxSoc', float64),
    ('essDischgToFcMaxEffPerc', float64),
    ('essChgToFcMaxEffPerc', float64),
    ('maxAccelBufferMph', float64),
    ('maxAccelBufferPercOfUseableSoc', float64),
    ('percHighAccBuf', float64),
    ('mphFcOn', float64),
    ('kwDemandFcOn', float64),
    ('altEff', float64),
    ('chgEff', float64),
    ('auxKw', float64),
    ('forceAuxOnFC', bool_),
    ('transKg', float64),
    ('transEff', float64),
    ('compMassMultiplier', float64),
    ('essToFuelOkError', float64),
    ('maxRegen', float64),
    ('valUddsMpgge', float64),
    ('valHwyMpgge', float64),
    ('valCombMpgge', float64),
    ('valUddsKwhPerMile', float64),
    ('valHwyKwhPerMile', float64),
    ('valCombKwhPerMile', float64),
    ('valCdRangeMi', float64),
    ('valConst65MphKwhPerMile', float64),
    ('valConst60MphKwhPerMile', float64),
    ('valConst55MphKwhPerMile', float64),
    ('valConst45MphKwhPerMile', float64),
    ('valUnadjUddsKwhPerMile', float64),
    ('valUnadjHwyKwhPerMile', float64),
    ('val0To60Mph', float64),
    ('valEssLifeMiles', float64),
    ('valRangeMiles', float64),
    ('valVehBaseCost', float64),
    ('valMsrp', float64),
    ('minFcTimeOn', float64),
    ('idleFcKw', float64),
    ('MaxRoadwayChgKw', float64[:]),
    ('chargingOn', bool_),
    ('noElecSys', bool_),
    ('noElecAux', bool_),
    ('vehTypeSelection', int32),
    ('fcEffArray', float64[:]),
    ('fcKwOutArray', float64[:]),
    ('maxFcEffKw', float64),
    ('fcMaxOutkW', float64),
    ('mcKwInArray', float64[:]),
    ('mcKwOutArray', float64[:]),
    ('mcMaxElecInKw', float64),
    ('mcFullEffArray', float64[:]),
    ('mcEffArray', float64[:]),
    ('regenA', float64),
    ('regenB', float64),
    ('vehKg', float64),
    ('maxTracMps2', float64),
    ('maxRegenKwh', float64)
]

@jitclass(veh_spec)
class TypedVehicle(object):
    """Just-in-time compiled version of Vehicle using numba."""
    
    def __init__(self):
        """This method initialized type numpy arrays as required by
        numba jitclass."""
        self.MaxRoadwayChgKw = np.zeros(6, dtype=np.float64)
        self.fcEffArray = np.zeros(100, dtype=np.float64)
        self.fcKwOutArray = np.zeros(100, dtype=np.float64)
        self.mcKwInArray = np.zeros(101, dtype=np.float64)
        self.mcKwOutArray = np.zeros(101, dtype=np.float64)
        self.mcFullEffArray = np.zeros(101, dtype=np.float64)
        self.mcEffArray = np.zeros(11, dtype=np.float64)