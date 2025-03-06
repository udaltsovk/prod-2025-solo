DEFINE EVENT OVERWRITE day_changed ON TABLE system WHEN $value.id.id() == "time" && $event == "UPDATE" THEN {
    FOR $stats IN SELECT id, campaign_id, daily, current FROM stats_campaign {
        UPDATE ONLY $stats.campaign_id
			MERGE {
				is_active: true
			};
		UPDATE ONLY $stats.id
			MERGE {
				daily: IF $stats.campaign_id.is_active {
                    $stats.daily.append($stats.current)
                } ELSE {
                    $stats.daily
                },
				current: fn::default_stats(),
			};
	};
    FOR $stats IN SELECT id, daily, current FROM stats_advertiser {
		UPDATE ONLY $stats.id
			MERGE {
				daily: $stats.daily.append($stats.current),
				current: fn::default_stats(),
			};
	};
};