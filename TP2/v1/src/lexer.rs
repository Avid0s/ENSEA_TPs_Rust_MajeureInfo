use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // One more sequential digits from 0 to 9 will be mapped to a "number"
        "DEFAULT" | "number" = pattern r"[0-9]+";

        // Whitespace " " will be skipped
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
        
        // Orders will be mapped to their respective tokens
        "DEFAULT" | "FORWARD" = pattern "forward";
        "DEFAULT" | "BACKWARD" = pattern "backward";
        "DEFAULT" | "LEFT" = pattern "left";
        "DEFAULT" | "RIGHT" = pattern "right";

     )
}