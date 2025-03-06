DEFINE TABLE OVERWRITE stats_advertiser SCHEMAFULL;

DEFINE FIELD OVERWRITE advertiser_id ON stats_advertiser
    TYPE record<advertiser>;
DEFINE FIELD OVERWRITE current ON stats_advertiser
    TYPE {
        impressions_count: number,
        clicks_count: number,
        conversion: float,
        spent_impressions: float,
        spent_clicks: float,
        spent_total: float,
    }
    VALUE fn::update_stats($after);
DEFINE FIELD OVERWRITE total ON stats_advertiser
    TYPE {
        impressions_count: number,
        clicks_count: number,
        conversion: float,
        spent_impressions: float,
        spent_clicks: float,
        spent_total: float,
    }
    VALUE fn::update_stats($after);
DEFINE FIELD OVERWRITE daily ON stats_advertiser
    TYPE array<{
        impressions_count: number,
        clicks_count: number,
        conversion: float,
        spent_impressions: float,
        spent_clicks: float,
        spent_total: float,
    }>;