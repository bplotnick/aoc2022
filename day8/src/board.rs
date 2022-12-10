pub struct Board {
    pub matrix: Vec<Vec<u32>>,
}

impl Board {
    pub fn from<I>(lines: I) -> Board
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let matrix = lines
            .into_iter()
            .map(|l| {
                l.as_ref()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        return Board{matrix};
    }
}