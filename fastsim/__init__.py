"""Package containing modules for running FASTSim.  
For example usage, see """

from pathlib import Path 

from . import parameters as params
from . import utilities as utils
from . import simdrive, vehicle, cycle, calibration, tests
from . import calibration as cal
from . import auxiliaries

from pkg_resources import get_distribution

__version__ = get_distribution('fastsim').version

__doc__ += f"{Path(__file__).parent / 'docs/README.md'}"
