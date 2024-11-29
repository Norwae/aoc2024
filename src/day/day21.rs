use crate::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pattern2x2 {
    OO_OO,
    XO_OO,
    XX_OO,
    OX_XO,
    XX_XO,
    XX_XX,
}

struct PatternMatch2x2 {
    matchers: [[bool; 4]; 4],
    pattern: Pattern2x2,
}

const fn create_pattern_match_2x2(flat: [bool; 4], pattern: Pattern2x2) -> PatternMatch2x2 {
    // 01
    // 23
    let simple = [flat[0], flat[1], flat[2], flat[3]];
    // 20
    // 31
    let once = [flat[2], flat[0], flat[3], flat[1]];
    // 32
    // 10
    let twice = [flat[3], flat[2], flat[1], flat[0]];
    // 13
    // 02
    let trice = [flat[1], flat[3], flat[0], flat[2]];

    PatternMatch2x2 { matchers: [simple, once, twice, trice], pattern }
}
const MATCHERS_2X2: [PatternMatch2x2; 6] = const {
    use Pattern2x2::*;
    [
        create_pattern_match_2x2([false, false, false, false], OO_OO),
        create_pattern_match_2x2([true, false, false, false], XO_OO),
        create_pattern_match_2x2([true, true, false, false], XX_OO),
        create_pattern_match_2x2([false, true, true, false], OX_XO),
        create_pattern_match_2x2([true, true, true, false], XX_XO),
        create_pattern_match_2x2([true, true, true, true], XX_XX),
    ]
};

#[allow(non_camel_case_types)]
enum Pattern3x3 {
    XXO_OOO_OOO,
    XOX_OOO_OOO,
    XXX_OOO_OOO,
    OXO_XOO_OOO,
    XXO_XOO_OOO,
    OOX_XOO_OOO,
    XOX_XOO_OOO,
    OXX_XOO_OOO,
    XXX_XOO_OOO,
    OOO_OXO_OOO,
    XOO_OXO_OOO,
    OXO_OXO_OOO,
    XXO_OXO_OOO,
    XOX_OXO_OOO,
    XXX_OXO_OOO,
    OXO_XXO_OOO,
    XXO_XXO_OOO,
    OOX_XXO_OOO,
    XOX_XXO_OOO,
    OXX_XXO_OOO,
    XXX_XXO_OOO,
    OOO_XOX_OOO,
    XOO_XOX_OOO,
    OXO_XOX_OOO,
    XXO_XOX_OOO,
    XOX_XOX_OOO,
    XXX_XOX_OOO,
    OOO_XXX_OOO,
    XOO_XXX_OOO,
    OXO_XXX_OOO,
    XXO_XXX_OOO,
    XOX_XXX_OOO,
    XXX_XXX_OOO,
    OOX_OOO_XOO,
    XOX_OOO_XOO,
    OXX_OOO_XOO,
    XXX_OOO_XOO,
    OXX_XOO_XOO,
    XXX_XOO_XOO,
    OOX_OXO_XOO,
    XOX_OXO_XOO,
    OXX_OXO_XOO,
    XXX_OXO_XOO,
    OXX_XXO_XOO,
    XXX_XXO_XOO,
    XOO_OOX_XOO,
    OXO_OOX_XOO,
    XXO_OOX_XOO,
    XOX_OOX_XOO,
    OXX_OOX_XOO,
    XXX_OOX_XOO,
    XOO_XOX_XOO,
    OXO_XOX_XOO,
    XXO_XOX_XOO,
    OOX_XOX_XOO,
    XOX_XOX_XOO,
    OXX_XOX_XOO,
    XXX_XOX_XOO,
    XOO_OXX_XOO,
    OXO_OXX_XOO,
    XXO_OXX_XOO,
    XOX_OXX_XOO,
    OXX_OXX_XOO,
    XXX_OXX_XOO,
    XOO_XXX_XOO,
    OXO_XXX_XOO,
    XXO_XXX_XOO,
    OOX_XXX_XOO,
    XOX_XXX_XOO,
    OXX_XXX_XOO,
    XXX_XXX_XOO,
    OXO_XOX_OXO,
    XXO_XOX_OXO,
    XOX_XOX_OXO,
    XXX_XOX_OXO,
    OXO_XXX_OXO,
    XXO_XXX_OXO,
    XOX_XXX_OXO,
    XXX_XXX_OXO,
    XOX_OOX_XXO,
    XXX_OOX_XXO,
    OXX_XOX_XXO,
    XXX_XOX_XXO,
    XOX_OXX_XXO,
    XXX_OXX_XXO,
    OXX_XXX_XXO,
    XXX_XXX_XXO,
    XOX_OOO_XOX,
    XXX_OOO_XOX,
    XXX_XOO_XOX,
    XOX_OXO_XOX,
    XXX_OXO_XOX,
    XXX_XXO_XOX,
    XOX_XOX_XOX,
    XXX_XOX_XOX,
    XOX_XXX_XOX,
    XXX_XXX_XOX,
    XXX_XOX_XXX,
    XXX_XXX_XXX,
}


struct PatternMatch3x3 {
    matchers: [[bool; 9]; 4],
    pattern: Pattern3x3,
}

const fn create_pattern_match_3x3(flat: [bool; 9], pattern: Pattern3x3) -> PatternMatch3x3 {
    // 012
    // 345
    // 678
    let simple = [flat[0], flat[1], flat[2], flat[3],flat[4], flat[5], flat[6], flat[7], flat[8]];
    // 630
    // 741
    // 852
    let once = [flat[6], flat[3], flat[0], flat[7],flat[4], flat[1], flat[8], flat[5], flat[2]];
    // 876
    // 543
    // 210
    let twice = [flat[8], flat[7], flat[6], flat[5],flat[4], flat[3], flat[2], flat[1], flat[0]];
    // 258
    // 147
    // 036
    let trice = [flat[2], flat[5], flat[8], flat[1],flat[4], flat[7], flat[0], flat[3], flat[6]];

    PatternMatch3x3 { matchers: [simple, once, twice, trice], pattern }
}

unimplemented_day!();