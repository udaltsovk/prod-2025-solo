DEFINE TABLE OVERWRITE scored 
    SCHEMAFULL
    TYPE RELATION FROM advertiser TO client ENFORCED;

DEFINE FIELD OVERWRITE score ON scored
    TYPE number
    ASSERT $value >= 0;