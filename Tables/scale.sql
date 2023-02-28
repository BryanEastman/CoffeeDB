CREATE TABLE IF NOT EXISTS scale (
    ScaleID INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY( INCREMENT BY 1 START 100)
    , ScaleMake VARCHAR(64) NOT NULL
    , ScaleModel VARCHAR(64) NOT NULL
    , ScaleMaxCapacity_g REAL
    , ScaleReadability_g REAL
    , ScaleRepeatability REAL
    , ScaleLinearity REAL
)