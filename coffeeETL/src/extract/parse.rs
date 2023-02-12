use crate::config::{Conf};
use std::fs::File;
use glob::{glob};
use std::io::BufReader;
use std::path::PathBuf;

use crate::extract::structures;

pub fn parse_json(log_path: &PathBuf) -> Result<(), std::io::Error> {
    let f = File::open(log_path).expect("failed at file open");
    let reader = BufReader::new(f);
    let log: structures::RawLog = serde_json::from_reader(reader).unwrap();
    
    let roast = structures::Roast {

        RoastDate: log.roastisodate,
        RoastTime: log.roasttime,
        RoasterName: log.operator,
        ChargeET: log.computed.CHARGE_ET,
        ChargeBT: log.computed.CHARGE_BT,
        TurningTime: log.computed.TP_time,
        TurningET: log.computed.TP_ET,
        TurningBT: log.computed.TP_BT,
        DryTime: log.computed.DRY_time,
        DryET: log.computed.DRY_ET,
        DryBT: log.computed.DRY_BT,
        FCTime: log.computed.FCs_time,
        FCET: log.computed.FCs_ET,
        FCBT: log.computed.FCs_BT, 
        DropTime: log.computed.DROP_time,
        DropET: log.computed.DROP_ET,
        DropBT: log.computed.DROP_BT,
        TotalTime: log.computed.totaltime,
        PhaseDryTime: log.computed.dryphasetime,
        PhaseMidTime: log.computed.midphasetime,
        PhaseFinishTime: log.computed.finishphasetime,
        PhaseDryROR: log.computed.dry_phase_ror,
        PhaseMidROR: log.computed.mid_phase_ror,
        PhaseFinishROR: log.computed.finish_phase_ror,
        TotalROR: log.computed.total_ror,
        FCROR: log.computed.fcs_ror,
        TSTotal: log.computed.total_ts,
        TSTotalET: log.computed.total_ts_ET,
        TSTotalBT: log.computed.total_ts_BT,
        AUC: log.computed.AUC,
        AUCDry: log.computed.dry_phase_AUC,
        AUCMid: log.computed.mid_phase_AUC,
        AUCFinish: log.computed.finish_phase_AUC,
        WeightLoss: log.computed.weight_loss,
        VolumeIn: log.computed.volumein,
        VolumeOut: log.computed.volumeout,
        WeightIn: log.computed.weightin,
        WeightOut: log.computed.weightout,
        TempAmbient: log.computed.ambient_temperature,
        DET: log.computed.det,
        DBT: log.computed.dbt
    };

    println!("{}", roast.DropET);

    Ok(())
}

pub fn file_loop() -> Result<(), std::io::Error> {
    let cred_file = File::open("../config.json")?;
    let reader = BufReader::new(cred_file);
    let creds: Conf = serde_json::from_reader(reader)?;

    let raw: String = creds.data_source;

    // path to move parsed files
    let _finished: String = creds.data_dump;

    let search_string: String = format!("{}/*.json", raw);

    for entry in glob(&search_string).expect("files not found") {
        println!("file:{}", &entry.as_ref().expect("").display());
        parse_json(&entry.expect("error parsing file"))?;
    }

    Ok(())
}