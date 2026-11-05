// SPDX-License-Identifier: MIT
//! Browser-facing calendar operations backed by the external Rust puzzle library.
use calendar_puzzle::calendar::{CalendarDate, CalendarLayout, MONTH_LENGTHS, MONTHS, WEEKDAYS};
use calendar_puzzle::{Board, Pentomino, Placement, solve_with_stats, validate_solution};
use wasm_bindgen::prelude::*;

/// A layout parsed once by the original library, retained for subsequent solves.
#[wasm_bindgen]
pub struct CalendarSolver {
    layout: CalendarLayout,
}

#[wasm_bindgen]
impl CalendarSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(layout_text: &str) -> Result<CalendarSolver, JsValue> {
        Self::parse(layout_text).map_err(js_error)
    }

    pub fn weekdays(&self) -> Vec<String> {
        WEEKDAYS.iter().map(|label| (*label).to_owned()).collect()
    }

    pub fn months(&self) -> Vec<String> {
        MONTHS.iter().map(|label| (*label).to_owned()).collect()
    }

    pub fn days_in_month(&self, month: &str) -> Result<u8, JsValue> {
        MONTHS
            .iter()
            .position(|&label| label == month)
            .map(|index| MONTH_LENGTHS[index])
            .ok_or_else(|| js_error(format!("Unrecognized month '{month}'")))
    }

    /// Return a solved board or a proven-unsolvable result; errors throw in JS.
    pub fn solve(
        &self,
        weekday: &str,
        month: &str,
        day: &str,
    ) -> Result<CalendarSolution, JsValue> {
        self.solve_date(weekday, month, day).map_err(js_error)
    }
}

impl CalendarSolver {
    fn parse(layout_text: &str) -> Result<Self, String> {
        CalendarLayout::parse(layout_text).map(|layout| Self { layout })
    }

    // Native tests exercise exactly the computation called by the Wasm method.
    fn solve_date(
        &self,
        weekday: &str,
        month: &str,
        day: &str,
    ) -> Result<CalendarSolution, String> {
        let date = CalendarDate::parse(weekday, month, day)?;
        let board = self.layout.board(date).map_err(|error| error.to_string())?;
        let (solution, stats) = solve_with_stats(&board).map_err(|error| error.to_string())?;
        let Some(placements) = solution else {
            return Ok(CalendarSolution {
                solved: false,
                cells: Vec::new(),
                nodes_visited: stats.nodes_visited,
            });
        };
        let mut cells = encode_solution(&board, &placements)?;
        let labels = [
            date.weekday().to_owned(),
            date.month().to_owned(),
            date.day().to_string(),
        ];
        for ((x, y), label) in self.layout.exclusions(date).into_iter().zip(labels) {
            cells[y * board.width() + x] = label;
        }
        Ok(CalendarSolution {
            solved: true,
            cells,
            nodes_visited: stats.nodes_visited,
        })
    }
}

/// Display data only; placements and bitmasks stay inside Rust.
#[wasm_bindgen]
pub struct CalendarSolution {
    solved: bool,
    cells: Vec<String>,
    nodes_visited: u64,
}

#[wasm_bindgen]
impl CalendarSolution {
    #[wasm_bindgen(getter)]
    pub fn solved(&self) -> bool {
        self.solved
    }

    /// 63 row-major cells when solved; an empty array when unsolvable.
    #[wasm_bindgen(getter)]
    pub fn cells(&self) -> Vec<String> {
        self.cells.clone()
    }

    /// Exposed as a JavaScript BigInt, preserving the library's exact u64 count.
    #[wasm_bindgen(getter)]
    pub fn nodes_visited(&self) -> u64 {
        self.nodes_visited
    }
}

fn js_error(error: String) -> JsValue {
    JsValue::from_str(&error)
}

fn encode_solution(board: &Board, solution: &[Placement]) -> Result<Vec<String>, String> {
    validate_solution(board, solution)
        .map_err(|error| format!("Solution validation failed: {error}"))?;
    let mut cells = vec![".".to_owned(); board.cell_count()];
    for placement in solution {
        let letter = match placement.piece {
            Pentomino::F => "F",
            Pentomino::I => "I",
            Pentomino::L => "L",
            Pentomino::P => "P",
            Pentomino::N => "N",
            Pentomino::T => "T",
            Pentomino::U => "U",
            Pentomino::V => "V",
            Pentomino::W => "W",
            Pentomino::X => "X",
            Pentomino::Y => "Y",
            Pentomino::Z => "Z",
        };
        for (index, cell) in cells.iter_mut().enumerate() {
            if placement.mask & (1u64 << index) != 0 {
                *cell = letter.to_owned();
            }
        }
    }
    Ok(cells)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TWO_SWAPS: &str = include_str!("../layouts/two-swaps.txt");

    #[test]
    fn computed_board_has_the_expected_wire_format() {
        let solver = CalendarSolver::parse(TWO_SWAPS).unwrap();
        let result = solver.solve_date("Sat", "Oct", "31").unwrap();
        assert!(result.solved());
        assert_eq!(result.cells.len(), 63);
        assert_eq!(result.nodes_visited, 1251);
        assert_eq!(result.cells[8], "Sat");
        assert_eq!(result.cells[23], "Oct");
        assert_eq!(result.cells[62], "31");
        for letter in "FILPNTUVWXYZ".chars() {
            assert_eq!(
                result
                    .cells
                    .iter()
                    .filter(|cell| **cell == letter.to_string())
                    .count(),
                5
            );
        }
    }

    #[test]
    fn encoding_preserves_validated_placements_and_coordinates() {
        let solver = CalendarSolver::parse(TWO_SWAPS).unwrap();
        let date = CalendarDate::parse("Sat", "Oct", "31").unwrap();
        let board = solver.layout.board(date).unwrap();
        let solution = solve_with_stats(&board).unwrap().0.unwrap();
        validate_solution(&board, &solution).unwrap();
        let cells = encode_solution(&board, &solution).unwrap();
        assert_eq!(
            cells
                .iter()
                .enumerate()
                .filter_map(|(i, c)| (c == ".").then_some(i))
                .collect::<Vec<_>>(),
            [8, 23, 62]
        );
        for (index, cell) in cells.iter().enumerate() {
            let placement = solution.iter().find(|p| p.mask & (1u64 << index) != 0);
            match placement {
                Some(placement) => assert_eq!(*cell, format!("{:?}", placement.piece)),
                None => assert_eq!(cell, "."),
            }
        }
    }

    #[test]
    fn encoding_rejects_incomplete_and_corrupt_solutions() {
        let board = CalendarLayout::parse(TWO_SWAPS)
            .unwrap()
            .board(CalendarDate::parse("Sat", "Oct", "31").unwrap())
            .unwrap();
        let mut solution = solve_with_stats(&board).unwrap().0.unwrap();
        assert!(
            encode_solution(&board, &solution[..11])
                .unwrap_err()
                .contains("Solution validation failed")
        );
        solution[1] = solution[0];
        assert!(
            encode_solution(&board, &solution)
                .unwrap_err()
                .contains("Solution validation failed")
        );
    }

    #[test]
    fn representative_dates_have_valid_cells_and_selected_labels() {
        let solver = CalendarSolver::parse(TWO_SWAPS).unwrap();
        for (weekday, month, day, expected) in [
            ("Sat", "Oct", "31", [(8, 0), (5, 2), (8, 6)]),
            ("Mon", "Jan", "1", [(1, 0), (1, 1), (0, 3)]),
            ("Fri", "Jun", "30", [(7, 0), (7, 1), (7, 6)]),
            ("Wed", "Feb", "29", [(4, 0), (2, 1), (6, 6)]),
        ] {
            let date = CalendarDate::parse(weekday, month, day).unwrap();
            assert_eq!(solver.layout.exclusions(date), expected);
            let result = solver.solve_date(weekday, month, day).unwrap();
            assert!(result.solved);
            assert_eq!(result.cells.len(), 63);
            for ((x, y), label) in expected.into_iter().zip([weekday, month, day]) {
                assert_eq!(result.cells[y * 9 + x], label);
            }
            let board = solver.layout.board(date).unwrap();
            let placements: Vec<_> = Pentomino::ALL
                .into_iter()
                .map(|piece| {
                    let letter = format!("{piece:?}");
                    let mask = result
                        .cells
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| **c == letter)
                        .fold(0, |mask, (i, _)| mask | (1u64 << i));
                    assert_eq!(mask.count_ones(), 5);
                    Placement { piece, mask }
                })
                .collect();
            validate_solution(&board, &placements).unwrap();
        }
    }

    #[test]
    fn invalid_layouts_and_dates_are_errors() {
        assert!(CalendarSolver::parse("invalid").is_err());
        assert!(CalendarSolver::parse(&TWO_SWAPS.replacen("Sun", "Mon", 1)).is_err());
        let solver = CalendarSolver::parse(TWO_SWAPS).unwrap();
        for (weekday, month, day) in [
            ("Bad", "Jan", "1"),
            ("Mon", "Bad", "1"),
            ("Mon", "Feb", "30"),
            ("Mon", "Apr", "31"),
            ("Mon", "Jan", "0"),
            ("Mon", "Jan", "-1"),
            ("Mon", "Jan", "hello"),
            ("Mon", "Jan", "999999999999999999999"),
        ] {
            assert!(solver.solve_date(weekday, month, day).is_err());
        }
    }

    #[test]
    fn month_controls_use_library_labels_and_leap_year_lengths() {
        let solver = CalendarSolver::parse(TWO_SWAPS).unwrap();
        assert_eq!(solver.weekdays(), WEEKDAYS);
        assert_eq!(solver.months(), MONTHS);
        for (month, length) in MONTHS.into_iter().zip(MONTH_LENGTHS) {
            assert_eq!(solver.days_in_month(month).unwrap(), length);
        }
        assert_eq!(solver.days_in_month("Feb").unwrap(), 29);
        assert!(solver.solve_date("Mon", "Feb", "29").is_ok());
    }

    #[test]
    fn unsolvable_results_have_no_stale_cells_and_do_not_prevent_later_solves() {
        let original = TWO_SWAPS.replacen(
            ". Jan Feb Mar . Apr May Jun .",
            "Jan . Feb Mar . Apr May . Jun",
            1,
        );
        let solver = CalendarSolver::parse(&original).unwrap();
        assert!(solver.solve_date("Sat", "Oct", "31").unwrap().solved);
        let result = solver.solve_date("Mon", "Jan", "1").unwrap();
        assert!(!result.solved);
        assert!(result.cells.is_empty());
        assert_eq!(result.nodes_visited, 1);
        assert!(solver.solve_date("Sat", "Feb", "30").is_err());
        assert!(solver.solve_date("Sat", "Oct", "31").unwrap().solved);
    }
}
