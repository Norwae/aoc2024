use std::error::Error;

pub trait Day {
    type ParseError: Error;
    type ParsedInput;
    type Part1Memoization;

    fn parse(&self, input: String) -> Result<Self::ParsedInput, Self::ParseError>;
    fn solve_part_1(&self, input: &Self::ParsedInput) -> (String, Self::Part1Memoization);
    fn solve_part_2(&self, input: &Self::ParsedInput, memo: Self::Part1Memoization) -> String;
}

struct OneShotDay<F> {
    function: F
}

impl <T> Day for OneShotDay<T>
where T: Fn(&str) -> (String, String) {
    type ParseError = !;
    type ParsedInput = String;
    type Part1Memoization = String;

    fn parse(&self, input: String) -> Result<Self::ParsedInput, Self::ParseError> {
        Ok(input)
    }

    fn solve_part_1(&self, input: &Self::ParsedInput) -> (String, Self::Part1Memoization) {
        (self.function)(&input)
    }

    fn solve_part_2(&self, input: &Self::ParsedInput, memo: Self::Part1Memoization) -> String {
        memo
    }
}

struct NonMemoizingFullDay<
    Parse,
    SolvePart1,
    SolvePart2
> {
    parser: Parse,
    solver_1: SolvePart1,
    solver_2: SolvePart2,
}

impl<Parse, SolvePart1, SolvePart2> NonMemoizingFullDay<Parse, SolvePart1, SolvePart2> {
    fn new(parser: Parse, solver_1: SolvePart1, solver_2: SolvePart2) -> Self {
        NonMemoizingFullDay { parser, solver_1, solver_2 }
    }
}

impl<PInput, PError, Parser, SolvePart1, SolvePart2> Day
for NonMemoizingFullDay<Parser, SolvePart1, SolvePart2>
where
    PError: Error,
    Parser: Fn(&str) -> Result<PInput, PError>,
    SolvePart1: Fn(&PInput) -> String,
    SolvePart2: Fn(&PInput) -> String,
{
    type ParseError = PError;
    type ParsedInput = PInput;
    type Part1Memoization = ();

    fn parse(&self, input: String) -> Result<Self::ParsedInput, Self::ParseError> {
        (self.parser)(&input)
    }

    fn solve_part_1(&self, input: &Self::ParsedInput) -> (String, Self::Part1Memoization) {
        ((self.solver_1)(input), ())
    }

    fn solve_part_2(&self, input: &Self::ParsedInput, _memo: Self::Part1Memoization) -> String {
        (self.solver_2)(input)
    }
}

struct MemoizingFullDay<
    Parse,
    SolvePart1,
    SolvePart2
> {
    parser: Parse,
    solver_1: SolvePart1,
    solver_2: SolvePart2,
}

impl<Parse, SolvePart1, SolvePart2> MemoizingFullDay<Parse, SolvePart1, SolvePart2> {
    fn new(parser: Parse, solver_1: SolvePart1, solver_2: SolvePart2) -> Self {
        MemoizingFullDay { parser, solver_1, solver_2 }
    }
}


impl<PInput, PError, Memo, Parser, SolvePart1, SolvePart2> Day
for MemoizingFullDay<Parser, SolvePart1, SolvePart2>
where
    PError: Error,
    Parser: Fn(&str) -> Result<PInput, PError>,
    SolvePart1: Fn(&PInput) -> (String, Memo),
    SolvePart2: Fn(&PInput, Memo) -> String,
{
    type ParseError = PError;
    type ParsedInput = PInput;
    type Part1Memoization = Memo;

    fn parse(&self, input: String) -> Result<Self::ParsedInput, Self::ParseError> {
        (self.parser)(&input)
    }

    fn solve_part_1(&self, input: &Self::ParsedInput) -> (String, Self::Part1Memoization) {
        (self.solver_1)(input)
    }

    fn solve_part_2(&self, input: &Self::ParsedInput, memo: Self::Part1Memoization) -> String {
        (self.solver_2)(input, memo)
    }
}