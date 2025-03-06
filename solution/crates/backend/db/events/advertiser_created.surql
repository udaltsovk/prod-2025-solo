DEFINE EVENT OVERWRITE advertiser_created ON TABLE advertiser WHEN $event == "CREATE" THEN {
    fn::create_stats($this.id);
};