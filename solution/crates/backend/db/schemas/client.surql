DEFINE TABLE OVERWRITE client SCHEMAFULL;

DEFINE FIELD OVERWRITE login ON client 
    TYPE string;
DEFINE FIELD OVERWRITE age ON client 
    TYPE number
    ASSERT $value >= 0;
DEFINE FIELD OVERWRITE location ON client 
    TYPE string;
DEFINE FIELD OVERWRITE gender ON client 
    TYPE "MALE" | "FEMALE";