DEFINE EVENT OVERWRITE campaign_created ON TABLE campaign WHEN $event == "CREATE" THEN {
    fn::create_stats($after.id);
};