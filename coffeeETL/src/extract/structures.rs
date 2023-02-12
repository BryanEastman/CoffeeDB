use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct InOutScale(f32, f32, String);

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WeightVol(f32, String, f32, String);

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ComputedField{
    pub CHARGE_ET: f32,
    pub CHARGE_BT: f32,
    pub TP_idx: f32,
    pub TP_time: f32,
    pub TP_ET: f32,
    pub TP_BT: f32,
    pub MET: f32,
    pub DRY_time: f32,
    pub DRY_ET: f32,
    pub DRY_BT: f32,
    pub FCs_time: f32,
    pub FCs_ET: f32,
    pub FCs_BT: f32,
    pub DROP_time: f32,
    pub DROP_ET: f32,
    pub DROP_BT: f32,
    pub totaltime: f32,
    pub dryphasetime: f32,
    pub midphasetime: f32,
    pub finishphasetime: f32,
    pub dry_phase_ror: f32,
    pub mid_phase_ror: f32,
    pub finish_phase_ror: f32,
    pub total_ror: f32,
    pub fcs_ror: f32,
    pub total_ts: f32,
    pub total_ts_ET: f32,
    pub total_ts_BT: f32,
    pub AUC: f32,
    pub AUCbegin: String,
    pub AUCbase: f32,
    pub AUCfromeventflag: f32,
    pub dry_phase_AUC: f32,
    pub mid_phase_AUC: f32,
    pub finish_phase_AUC: f32,
    pub weight_loss: f32,
    pub volumein: f32,
    pub volumeout: f32,
    pub weightin: f32,
    pub weightout: f32,
    pub ambient_temperature: f32,
    pub det: f32,
    pub dbt: f32,
    pub BTU_preheat: f32,
    pub CO2_preheat: f32,
    pub BTU_bbp: f32,
    pub CO2_bbp: f32,
    pub BTU_cooling: f32,
    pub CO2_cooling: f32,
    pub BTU_LPG: f32,
    pub BTU_NG: f32,
    pub BTU_ELEC: f32,
    pub BTU_batch: f32,
    pub BTU_batch_per_green_kg: f32,
    pub BTU_roast: f32,
    pub BTU_roast_per_green_kg: f32,
    pub CO2_batch: f32,
    pub CO2_batch_per_green_kg: f32,
    pub CO2_roast: f32,
    pub CO2_roast_per_green_kg: f32,
    pub KWH_batch_per_green_kg: f32,
    pub KWH_roast_per_green_kg: f32
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RawLog {
    pub version: String,
    pub revision: String,
    pub build: String,
    pub artisan_os: String,
    pub artisan_os_version: String,
    pub mode: String,
    pub viewerMode: bool,
    pub timeindex: Vec<f32>,
    pub flavors: Vec<f32>,
    pub flavorlabels: Vec<String>, 
    pub flavorstartangle: f64,
    pub flavoraspect: f64,
    pub title: String,
    pub locale: String,
    pub beans: String,
    pub weight: InOutScale,
    pub volume: InOutScale,
    pub density:  WeightVol,
    pub density_roasted: WeightVol, 
    pub roastertype: String,
    pub roastersize: f32,
    pub roasterheating: f32,
    pub machinesetup: String,
    pub operator: String,
    pub organization: String,
    pub drumspeed: String,
    pub heavyFC: bool,
    pub lowFC: bool,
    pub lightCut: bool,
    pub darkCut: bool,
    pub drops: bool, 
    pub oily: bool,
    pub uneven: bool,
    pub tipping: bool, 
    pub scorching: bool,
    pub divots: bool,
    pub whole_color: f32,
    pub ground_color: f32,
    pub color_system: String,
    pub volumeCalcWeightIn: String,
    pub volumeCalcWeightOut: String,
    pub roastdate: String,
    pub roastisodate: String,
    pub roasttime: String,
    pub roastepoch: f32,
    pub roasttzoffset: f32,
    pub roastbatchnr: f32,
    pub roastbatchprefix: String,
    pub roastbatchpos: f32,
    pub roastUUID: String,
    pub beansize: String,
    pub beansize_min: String,
    pub beansize_max: String,
    pub specialevents: Vec<String>,
    pub specialeventstype: Vec<String>,
    pub specialeventsvalue: Vec<String>, 
    pub specialeventsStrings: Vec<String>,
    pub etypes: Vec<String>, 
    pub roastingnotes: String, 
    pub cuppingnotes: String,
    pub timex: Vec<f32>,
    pub temp1: Vec<f32>,
    pub phases: Vec<f32>,
    pub zmax: f32,
    pub zmin: f32,
    pub ymax: f32,
    pub ymin: f32,
    pub xmin: f32,
    pub xmax: f32,
    pub ambientTemp: f32,
    pub ambient_humidity: f32,
    pub ambient_pressure: f32,
    pub moisture_greens: f32,
    pub greens_temp: f32,
    pub moisture_roasted: f32,
    pub extradevices: Vec<f32>,
    pub extraname1: Vec<String>,
    pub extraname2: Vec<String>,
    pub extratimex: Vec<Vec<f32>>,
    pub extratemp2: Vec<Vec<f32>>,
    pub extramathexpression1: Vec<String>,
    pub extramathexpression2: Vec<String>,
    pub extradevicecolor1: Vec<String>,
    pub extradevicecolor2: Vec<String>,
    pub extraLCDvisibility1: Vec<bool>,
    pub extraLCDvisibility2: Vec<bool>,
    pub extraCurveVisibility1: Vec<bool>,
    pub extraCurveVisibility2: Vec<bool>,
    pub extraDelta2: Vec<bool>,
    pub extraFill1: Vec<f32>,
    pub extraFill2: Vec<f32>,
    pub extramarkersizes1: Vec<f32>,
    pub extramarkersizes2: Vec<f32>,
    pub extramarkers1: Option<Vec<String>>,
    pub extramarkers2: Option<Vec<String>>,
    pub extralinewidths1: Option<Vec<f32>>,
    pub extralinewidths2: Option<Vec<f32>>,
    pub extralinestyles1: Option<Vec<String>>,
    pub extralinestyles2: Option<Vec<String>>,
    pub extradrawstyles1: Option<Vec<String>>,
    pub extradrawstyles2: Option<Vec<String>>,
    pub externalprogram: String,
    pub externaloutprogram: String,
    pub extraNoneTempHint1: Option<Vec<bool>>, 
    pub extraNoneTempHint2: Option<Vec<bool>>,
    pub alarmsetlabel: String,
    pub alarmflag: Option<Vec<String>>,
    pub alarmguard: Option<Vec<String>>,
    pub alarmnegguard: Option<Vec<String>>,
    pub alarmtime: Option<Vec<String>>,
    pub alarmoffset: Option<Vec<String>>,
    pub alarmcond: Option<Vec<String>>,
    pub alarmsource: Option<Vec<String>>,
    pub alarmtemperature: Option<Vec<String>>,
    pub alarmaction: Option<Vec<String>>,
    pub alarmbeep: Option<Vec<String>>,
    pub alarmstrings: Option<Vec<String>>,
    pub backgroundpath: String, 
    pub backgroundUUID: String,
    pub samplinginterval: f32,
    pub oversampling: bool,
    pub svLabel: String,
    pub svValues: Vec<f32>,
    pub svRamps: Vec<f32>,
    pub svSoaks: Vec<f32>,
    pub svActions: Vec<f32>,
    pub svBeeps: Vec<bool>,
    pub svDescriptions: Vec<String>,
    pub devices: Vec<String>,
    pub elevation: f32,
    pub computed: ComputedField,   
    pub anno_positions: Vec<Vec<f32>>, 
    pub flag_positions: Option<Vec<f32>>,
    pub loadlabels: Vec<String>,
    pub loadratings: Vec<f32>,
    pub ratingunits: Vec<f32>,
    pub sourcetypes: Vec<f32>
} 

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Flavors {
    pub acidity: f32,
    pub aftertaste: f32,
    pub clean_cup: f32,
    pub head: f32,
    pub fragrance: f32, 
    pub sweetness: f32,
    pub aroma: f32,
    pub balance: f32,
    pub body: f32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Defects {
    pub heavy_fc: bool,
    pub low_fc: bool,
    pub light_cut: bool,
    pub dark_cut: bool,
    pub drops: bool,
    pub oily: bool,
    pub uneven: bool,
    pub tipping: bool,
    pub scorching: bool,
    pub divots: bool 
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Roaster {
    pub roastertype: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Roast {
    pub roasterisodate: String,
    pub roasttime: String,
    pub roastepoch: f32,
    pub roasttzzoffset: f32,
    pub roastbatchnr: f32,
    pub roastbatchprefix: String,
    pub roastbatchpos: f32,
    pub roastUUID: String,

}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Operator {
    pub operatorname: String,
    
}
