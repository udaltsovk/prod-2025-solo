DEFINE EVENT OVERWRITE scored ON TABLE scored WHEN $event ∈ ["CREATE", "UPDATE"] THEN {
    LET $scores = fn::ml_scores();
    LET $min: float = math::min($scores.min, $value);
    LET $max: float = math::max($scores.max, $value);

    UPDATE ONLY system:ml_score
        SET 
            min = $min,
            max = $max,
            delta = 1dec / ($max - $min);
};