DEFINE TABLE OVERWRITE interacted_with
    SCHEMAFULL
    TYPE RELATION FROM client TO campaign ENFORCED;

DEFINE FIELD OVERWRITE impressed ON interacted_with
    TYPE number
    VALUE IF type::is::number($before) {
        $before
    } ELSE { fn::current_day() };
DEFINE FIELD OVERWRITE clicked ON interacted_with
    TYPE option<number>
    VALUE IF $value != none {
        fn::current_day()
    };