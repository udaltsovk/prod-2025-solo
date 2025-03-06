DEFINE FUNCTION OVERWRITE fn::calc_conversion($clicks_count: number, $impressions_count: number) -> float {
	LET $convertion: float = ($clicks_count * 100f) ÷ $impressions_count;
	
	IF $convertion ∈ 0..=100 {
	    RETURN math::fixed($convertion, 2)
	} ELSE {
	    RETURN 0
	}
};

UPSERT ONLY system:time 
	SET current_date = 1;

DEFINE FUNCTION OVERWRITE fn::current_day() -> number {
	RETURN SELECT VALUE current_date FROM ONLY system:time;
};

DEFINE FUNCTION OVERWRITE fn::update_stats($obj: {
	impressions_count: number,
	clicks_count: number,
	conversion: float,
	spent_impressions: float,
	spent_clicks: float,
	spent_total: float,
}) -> {
	impressions_count: number,
	clicks_count: number,
	conversion: float,
	spent_impressions: float,
	spent_clicks: float,
	spent_total: float,
} {
	RETURN {
        impressions_count: $obj.impressions_count,
        clicks_count: $obj.clicks_count,
        conversion: fn::calc_conversion($obj.clicks_count, $obj.impressions_count),
        spent_impressions: $obj.spent_impressions,
        spent_clicks: $obj.spent_clicks,
        spent_total: $obj.spent_impressions + $obj.spent_clicks
    };
};

DEFINE FUNCTION OVERWRITE fn::default_stats() -> {
	impressions_count: number,
	clicks_count: number,
	conversion: float,
	spent_impressions: float,
	spent_clicks: float,
	spent_total: float,
} {
	RETURN {
        impressions_count: 0,
        clicks_count: 0,
        conversion: 0,
        spent_impressions: 0,
        spent_clicks: 0,
        spent_total: 0
    };
};

DEFINE FUNCTION OVERWRITE fn::stats_id_from_obj_id($id: record<campaign | advertiser>) -> record<stats_campaign | stats_advertiser> {
	RETURN type::thing("stats_" + $id.tb(), $id.id());
};

DEFINE FUNCTION OVERWRITE fn::create_stats($record_id: record<campaign | advertiser>) {
    LET $tb: string = $record_id.tb();
    LET $stats = object::from_entries([
        [ "id", fn::stats_id_from_obj_id($record_id) ],
        [ $tb + "_id", $record_id ],
        [ "current", fn::default_stats() ],
        [ "total", fn::default_stats() ],
        [ "daily", [] ]
    ]);
    IF $tb == "campaign" {
        INSERT INTO stats_campaign $stats;
    } ELSE {
        INSERT INTO stats_advertiser $stats;
    };
};

DEFINE FUNCTION OVERWRITE fn::update_campaign_active($campaign: option<record<campaign>>, $stats: option<record<stats_campaign>>) -> bool {
	LET $time: number = fn::current_day();
    LET $campaign: record<campaign> = $campaign ?? $stats.campaign_id;
	LET $impressions_count: option<number> = ($stats ?? (SELECT impressions_count FROM ONLY type::thing('stats_campaign', $campaign.id()))).impressions_count;
    LET $impressions_count: number = if type::is::number($impressions_count)
        THEN {$impressions_count}
        ELSE {0}
        END;
	RETURN $campaign.start_date <= $time && $time<= $campaign.end_date
        && $impressions_count < $campaign.impressions_limit;
};

UPSERT ONLY system:ml_score 
	CONTENT {
        min: 0f,
        max: 0f,
        c: 0f 
    };

DEFINE FUNCTION OVERWRITE fn::ml_scores() -> {
    min: float,
    max: float,
    c: float
} {
	RETURN SELECT * OMIT id FROM ONLY system:ml_score;
};

DEFINE FUNCTION OVERWRITE fn::grade_score($score: float) -> float {
	LET $scores = fn::ml_scores();
    RETURN $score.c * ($score - $scores.min);
};

DEFINE FUNCTION OVERWRITE fn::score_campaign($client: record<client>, $campaign: record<campaign>) -> float {
    LET $interaction = SELECT impressed, clicked FROM type::thing("interacted_with", [$client.id(), $campaign.id()].join("_"));

    LET $coeff_impressed: float = IF type::is::number($interaction.impressed) {
        0
    } ELSE {1.0};
    LET $coeff_clicked: float = IF type::is::number($interaction.clicked) {
        0
    } ELSE {1.0};

    LET $clicks_count: number = SELECT VALUE total.clocks_count FROM ONLY fn::stats_id_from_obj_id($campaign);
    LET $score: float = SELECT VALUE score FROM ONLY type::thing("scored", [$campaign.advertiser_id.id(), $client.id()].join("_"));

    LET $coeff: float = $graded_score * IF $clicks_count == $campaign.clicks_limit {
        -0.5
    } ELSE {1};

    RETURN $campaign.cost_per_impression * $coeff_impressed + $campaign.cost_per_click * $coeff * $coeff_clicked;
};