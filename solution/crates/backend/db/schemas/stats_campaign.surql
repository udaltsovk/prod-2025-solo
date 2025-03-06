DEFINE TABLE OVERWRITE stats_campaign SCHEMAFULL;

DEFINE FIELD OVERWRITE campaign_id ON stats_campaign
    TYPE record<campaign>;
DEFINE FIELD OVERWRITE current ON stats_campaign
    TYPE {
        impressions_count: number,
        clicks_count: number,
        conversion: float,
        spent_impressions: float,
        spent_clicks: float,
        spent_total: float,
    }
    VALUE IF $this.id == NONE { fn::default_stats() }
        ELSE { fn::update_stats($this.current) };
DEFINE FIELD OVERWRITE total ON stats_campaign
    TYPE {
        impressions_count: number,
        clicks_count: number,
        conversion: float,
        spent_impressions: float,
        spent_clicks: float,
        spent_total: float,
    }
    VALUE IF $this.id == NONE { fn::default_stats() }
        ELSE { fn::update_stats($this.total) };
DEFINE FIELD OVERWRITE daily ON stats_campaign
    TYPE array<{
        impressions_count: number,
        clicks_count: number,
        conversion: float,
        spent_impressions: float,
        spent_clicks: float,
        spent_total: float,
    }>;