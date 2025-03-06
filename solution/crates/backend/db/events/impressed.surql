DEFINE EVENT OVERWRITE impressed ON TABLE interacted_with WHEN $event == "CREATE" THEN {
    LET $impressions_count_delta: number = 1;
    LET $spent_impressions_delta = $value.out.cost_per_impression;
    UPDATE type::thing("stats_campaign", $value.out.id()), type::thing("stats_advertiser", $value.out.advertiser_id.id()) 
        SET 
            current.impressions_count += $impressions_count_delta,
            current.spent_impressions += $spent_impressions_delta,
            total.impressions_count += $impressions_count_delta,
            total.spent_impressions += $spent_impressions_delta;
    fn::update_campaign_active($value.out);
};