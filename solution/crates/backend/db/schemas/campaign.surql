DEFINE TABLE OVERWRITE campaign SCHEMAFULL;

DEFINE FIELD OVERWRITE advertiser_id ON campaign
    TYPE record<advertiser>;
DEFINE FIELD OVERWRITE impressions_limit ON campaign
    TYPE number
    ASSERT $value > 0;
DEFINE FIELD OVERWRITE clicks_limit ON campaign
    TYPE number
    ASSERT $value >= 0;
DEFINE FIELD OVERWRITE cost_per_impression ON campaign
    TYPE float
    ASSERT $value > 0f;
DEFINE FIELD OVERWRITE cost_per_click ON campaign
    TYPE float
    ASSERT $value > 0f;
DEFINE FIELD OVERWRITE ad_title ON campaign
    TYPE string;
DEFINE FIELD OVERWRITE ad_text ON campaign
    TYPE string;
DEFINE FIELD OVERWRITE start_date ON campaign
    TYPE number;
DEFINE FIELD OVERWRITE end_date ON campaign
    TYPE number;
DEFINE FIELD OVERWRITE targeting ON campaign
    TYPE {
        gender: option<"MALE" | "FEMALE" | "ALL">,
        age_from: option<number>,
        age_to: option<number>,
        location: option<string>
    };
DEFINE FIELD OVERWRITE is_active ON campaign
    TYPE bool
    VALUE fn::update_campaign_active($this.id);