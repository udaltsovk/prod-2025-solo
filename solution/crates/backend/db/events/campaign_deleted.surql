DEFINE EVENT OVERWRITE campaign_deleted ON TABLE campaign WHEN $event == "DELETE" THEN {
    DELETE fn::stats_id_from_obj_id($this.id);
};