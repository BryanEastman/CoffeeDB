CREATE TABLE IF NOT EXISTS cupping (
    CuppingSessionID INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY( INCREMENT BY 1 START 100)
    , CuppingSessionDate DATE NOT NULL
    , CuppingRoastLevel TEXT
    , CuppingAromaScore INT 
    , CuppingFlavorScore INT
    , CuppingAftertasteScore INT
    , CuppingAcidityScore INT
    , CuppingAcidityIntensity INT
    , CuppingBodyScore INT
    , CuppingBodyLevel INT
    , CuppingUniformityScore INT
    , CuppingCleanScore INT
    , CuppingSweetnessScore INT
    , CuppingOverall INT
    , CuppingNotes TEXT
    , CuppingCoffeeWeight_g REAL 
    , CuppingWaterWeight_g REAL
    , SaleID INT NOT NULL REFERENCES scale
    , RoastID INT NOT NULL REFERENCES roast
    , OperatorID INT NOT NULL REFERENCES operator

)