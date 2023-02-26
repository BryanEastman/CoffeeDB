CREATE TABLE IF NOT EXISTS roast (
    RoastID INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY( INCREMENT BY 1 START 100)
    , RoastDate DATE NOT NULL
    , RoastTime TIME NOT NULL
    , RoasterName VARCHAR(64)
    , ChargeET REAL NOT NULL
    , ChargeBT REAL NOT NULL
    , TurningTime REAL NOT NULL
    , TurningET REAL NOT NULL
    , TurningBT REAL NOT NULL
    , DryTime: REAL NOT NULL
    , DryET: REAL NOT NULL
    , DryBT: REAL NOT NULL
    , FCTime: REAL NOT NULL
    , FCET: REAL NOT NULL
    , FCBT: REAL NOT NULL
    , DropTime: REAL NOT NULL
    , DropET: REAL NOT NULL
    , DropBT: REAL NOT NULL
    , TotalTime: REAL NOT NULL
    , PhaseDryTfime: REAL NOT NULL
    , PhaseMidTIme: REAL NOT NULL
    , PhaseFinishTime: REAL NOT NULL
    , PhaseDryROR: REAL NOT NULL
    , PhaseMidROR: REAL NOT NULL
    , PhaseFinishROR: REAL NOT NULL
    , TotalROR: REAL NOT NULL
    , FCROR: REAL NOT NULL
    , TSTotal: REAL NOT NULL
    , TSTotalET: REAL NOT NULL
    , TSTotalBT: REAL NOT NULL
    , AUC: REAL NOT NULL
    , AUCDry: REAL NOT NULL
    , AUCMid: REAL NOT NULL
    , AUCFinish: REAL NOT NULL
    , WeightLoss: REAL NOT NULL
    , VolumeIn: REAL NOT NULL
    , VolumeOut: REAL NOT NULL
    , WeightIn: REAL NOT NULL
    , WeightOut: REAL NOT NULL
    , TempAmbient: REAL NOT NULL
    , DET: REAL NOT NULL
    , DBT: REAL NOT NULL
)