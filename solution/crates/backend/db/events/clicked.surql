DEFINE EVENT OVERWRITE clicked ON TABLE interacted_with WHEN $event == "UPDATE" THEN {
    LET $clicks_count_delta: number = 1;
    LET $spent_clicks_delta = $value.out.cost_per_click;
    UPDATE type::thing("stats_campaign", $value.out.id()), type::thing("stats_advertiser", $value.out.advertiser_id.id()) 
        SET 
            current.clicks_count += $clicks_count_delta,
            current.spent_clicks += $spent_clicks_delta,
            total.clicks_count += $clicks_count_delta,
            total.spent_clicks += $spent_clicks_delta;
};