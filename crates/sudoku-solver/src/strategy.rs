use crate::{
    board::{Board, Candidates, CellId},
    solver::SolverContext,
};

#[derive(Clone, Copy)]
pub enum HighlightRole {
    Focus,
    Supporting,
    Eliminated,
}

pub struct CellHighlight {
    pub cell: CellId,
    pub role: HighlightRole,
}

pub struct CandidateHighlight {
    pub cell: CellId,
    pub digit: u8,
    pub role: HighlightRole,
}

pub struct Highlights {
    pub cells: Vec<CellHighlight>,
    pub candidates: Vec<CandidateHighlight>,
}

pub struct Explanation {
    pub summary: String,
    pub detail: String,
    pub highlights: Vec<Highlights>,
}

impl Explanation {
    pub fn peer_elimination(source: CellId, digit: u8) -> Self {
        Explanation {
            summary: format!("{} placed at {}", digit, source),
            detail: format!(
                "{} is {}, so {} is eliminated from every cell that sees it.",
                source, digit, digit
            ),
            highlights: vec![Highlights {
                cells: vec![CellHighlight {
                    cell: source,
                    role: HighlightRole::Focus,
                }],
                candidates: vec![CandidateHighlight {
                    cell: source,
                    digit,
                    role: HighlightRole::Focus,
                }],
            }],
        }
    }

    pub fn variant_elimination(rule: &str, eliminations: &[(CellId, u8)]) -> Self {
        let removed: Vec<String> = eliminations
            .iter()
            .map(|(cell, digit)| format!("{} from {}", digit, cell))
            .collect();
        Explanation {
            summary: format!("{} removes {}", rule, removed.join(", ")),
            detail: format!(
                "No digit that satisfies the {} can go there, so {} is eliminated.",
                rule,
                removed.join(", ")
            ),
            highlights: vec![Highlights {
                cells: vec![],
                candidates: eliminations
                    .iter()
                    .map(|&(cell, digit)| CandidateHighlight {
                        cell,
                        digit,
                        role: HighlightRole::Eliminated,
                    })
                    .collect(),
            }],
        }
    }

    pub fn naked_single(cell: CellId, digit: u8) -> Self {
        Explanation {
            summary: format!("Naked Single: {} = {}", cell, digit),
            detail: format!(
                "{} has only one remaining candidate. All other digits have been \
                 eliminated by the cells it sees, so it must be {}.",
                cell, digit
            ),
            highlights: vec![Highlights {
                cells: vec![CellHighlight {
                    cell,
                    role: HighlightRole::Focus,
                }],
                candidates: vec![CandidateHighlight {
                    cell,
                    digit,
                    role: HighlightRole::Focus,
                }],
            }],
        }
    }

    pub fn hidden_single(cell: CellId, digit: u8, house_name: &str) -> Self {
        Explanation {
            summary: format!("Hidden Single: {} = {}", cell, digit),
            detail: format!(
                "{} is the only cell in {} that can hold {}, so it must be {}.",
                cell, house_name, digit, digit
            ),
            highlights: vec![Highlights {
                cells: vec![CellHighlight {
                    cell,
                    role: HighlightRole::Focus,
                }],
                candidates: vec![CandidateHighlight {
                    cell,
                    digit,
                    role: HighlightRole::Focus,
                }],
            }],
        }
    }

    pub fn pointing(box_name: &str, digit: u8, line_name: &str, locked: &[CellId]) -> Self {
        let kind_word = if locked.len() == 2 { "Pair" } else { "Triple" };
        Explanation {
            summary: format!(
                "Pointing {}: {} in {} locked to {}",
                kind_word, digit, box_name, line_name
            ),
            detail: format!(
                "In {}, {} can only go in cells of {}. So {} can be \
                 eliminated from the rest of {}.",
                box_name, digit, line_name, digit, line_name
            ),
            highlights: vec![Highlights {
                cells: locked
                    .iter()
                    .map(|&cell| CellHighlight {
                        cell,
                        role: HighlightRole::Focus,
                    })
                    .collect(),
                candidates: locked
                    .iter()
                    .map(|&cell| CandidateHighlight {
                        cell,
                        digit,
                        role: HighlightRole::Focus,
                    })
                    .collect(),
            }],
        }
    }

    pub fn box_line_reduction(
        box_name: &str,
        digit: u8,
        line_name: &str,
        locked: &[CellId],
    ) -> Self {
        Explanation {
            summary: format!(
                "Box/Line Reduction: {} in {} locked to {}",
                digit, line_name, box_name
            ),
            detail: format!(
                "In {}, {} can only go in cells inside {}. So {} can be \
                 eliminated from the rest of {}.",
                line_name, digit, box_name, digit, box_name
            ),
            highlights: vec![Highlights {
                cells: locked
                    .iter()
                    .map(|&cell| CellHighlight {
                        cell,
                        role: HighlightRole::Focus,
                    })
                    .collect(),
                candidates: locked
                    .iter()
                    .map(|&cell| CandidateHighlight {
                        cell,
                        digit,
                        role: HighlightRole::Focus,
                    })
                    .collect(),
            }],
        }
    }

    pub fn naked_subset(cells: &[CellId], digits: &[u8]) -> Self {
        let kind_word = match cells.len() {
            2 => "Pair",
            3 => "Triple",
            _ => "Subset",
        };
        let cells_str: Vec<String> = cells.iter().map(|c| c.to_string()).collect();
        let digits_str: Vec<String> = digits.iter().map(|d| d.to_string()).collect();
        Explanation {
            summary: format!(
                "Naked {}: {} share candidates {{{}}}",
                kind_word,
                cells_str.join(", "),
                digits_str.join(", "),
            ),
            detail: format!(
                "Cells {} contain only candidates {{{}}}. These digits can be \
                 eliminated from all other cells that share a house with them.",
                cells_str.join(" and "),
                digits_str.join(", "),
            ),
            highlights: vec![Highlights {
                cells: cells
                    .iter()
                    .map(|&cell| CellHighlight {
                        cell,
                        role: HighlightRole::Focus,
                    })
                    .collect(),
                candidates: cells
                    .iter()
                    .flat_map(|&cell| {
                        digits.iter().map(move |&digit| CandidateHighlight {
                            cell,
                            digit,
                            role: HighlightRole::Focus,
                        })
                    })
                    .collect(),
            }],
        }
    }

    pub fn hidden_subset(cells: &[CellId], digits: &[u8]) -> Self {
        let kind_word = match cells.len() {
            2 => "Pair",
            3 => "Triple",
            _ => "Subset",
        };
        let cells_str: Vec<String> = cells.iter().map(|c| c.to_string()).collect();
        let digits_str: Vec<String> = digits.iter().map(|d| d.to_string()).collect();
        Explanation {
            summary: format!(
                "Hidden {}: digits {{{}}} only in {}",
                kind_word,
                digits_str.join(", "),
                cells_str.join(", "),
            ),
            detail: format!(
                "Digits {{{}}} can only go in cells {}. All other candidates \
                 can be eliminated from these cells.",
                digits_str.join(", "),
                cells_str.join(" and "),
            ),
            highlights: vec![Highlights {
                cells: cells
                    .iter()
                    .map(|&cell| CellHighlight {
                        cell,
                        role: HighlightRole::Focus,
                    })
                    .collect(),
                candidates: cells
                    .iter()
                    .flat_map(|&cell| {
                        digits.iter().map(move |&digit| CandidateHighlight {
                            cell,
                            digit,
                            role: HighlightRole::Supporting,
                        })
                    })
                    .collect(),
            }],
        }
    }

    pub fn fish(locked: &[CellId], digit: u8, bases: &[String], covers: &[String]) -> Self {
        let kind = match bases.len() {
            2 => "X-Wing",
            3 => "Swordfish",
            4 => "Jellyfish",
            _ => "Fish",
        };
        Explanation {
            summary: format!(
                "{}: {} in {} locked to {}",
                kind,
                digit,
                bases.join(", "),
                covers.join(", "),
            ),
            detail: format!(
                "In {}, {} only appears in {}. \
                 So {} can be eliminated from the rest of {}.",
                bases.join(" and "),
                digit,
                covers.join(" and "),
                digit,
                covers.join(" and "),
            ),
            highlights: vec![Highlights {
                cells: locked
                    .iter()
                    .map(|&cell| CellHighlight {
                        cell,
                        role: HighlightRole::Focus,
                    })
                    .collect(),
                candidates: locked
                    .iter()
                    .map(|&cell| CandidateHighlight {
                        cell,
                        digit,
                        role: HighlightRole::Focus,
                    })
                    .collect(),
            }],
        }
    }

    pub fn y_wing(pivot: CellId, wing1: CellId, wing2: CellId, a: u8, b: u8, c: u8) -> Self {
        Explanation {
            summary: format!(
                "Y-Wing: pivot {} {{{}, {}}}, wings {} and {} eliminate {}",
                pivot, a, b, wing1, wing2, c
            ),
            detail: format!(
                "{} must be {} or {}. \
                 If {}, {} must be {}. If {}, {} must be {}. \
                 Either way, {} can be eliminated from cells seeing both wings.",
                pivot, a, b, a, wing1, c, b, wing2, c, c
            ),
            highlights: vec![Highlights {
                cells: vec![
                    CellHighlight {
                        cell: pivot,
                        role: HighlightRole::Focus,
                    },
                    CellHighlight {
                        cell: wing1,
                        role: HighlightRole::Supporting,
                    },
                    CellHighlight {
                        cell: wing2,
                        role: HighlightRole::Supporting,
                    },
                ],
                candidates: vec![
                    CandidateHighlight {
                        cell: pivot,
                        digit: a,
                        role: HighlightRole::Focus,
                    },
                    CandidateHighlight {
                        cell: pivot,
                        digit: b,
                        role: HighlightRole::Focus,
                    },
                    CandidateHighlight {
                        cell: wing1,
                        digit: c,
                        role: HighlightRole::Supporting,
                    },
                    CandidateHighlight {
                        cell: wing2,
                        digit: c,
                        role: HighlightRole::Supporting,
                    },
                ],
            }],
        }
    }

    /// `on` and `off` are the chain's two colors. With `wrap`, `on` sees
    /// itself and is removed; otherwise cells seeing both colors are.
    pub fn simple_coloring(digit: u8, on: &[CellId], off: &[CellId], wrap: bool) -> Self {
        let (summary, conclusion) = if wrap {
            (
                format!("Simple Coloring: {} can't be in {}", digit, cell_list(on)),
                format!(
                    "Two cells of the first color see each other, so that color can't be {}.",
                    digit
                ),
            )
        } else {
            (
                format!("Simple Coloring on {}", digit),
                format!("A cell that sees both colors can't be {}.", digit),
            )
        };
        Explanation {
            summary,
            detail: format!(
                "Following the cells where {} has only two spots in a house splits them \
                 into two colors, {} and {}, and exactly one color holds {}. {}",
                digit,
                cell_list(on),
                cell_list(off),
                digit,
                conclusion
            ),
            highlights: vec![marked(
                digit,
                on.iter()
                    .map(|&c| (c, HighlightRole::Focus))
                    .chain(off.iter().map(|&c| (c, HighlightRole::Supporting))),
            )],
        }
    }

    /// `chain` alternates off and on cells, from the cell assumed off to
    /// the cell that ends up on.
    pub fn x_cycle(digit: u8, chain: &[CellId]) -> Self {
        let (start, end) = (chain[0], chain[chain.len() - 1]);
        let steps: Vec<String> = chain
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{} {}", c, if i % 2 == 0 { "off" } else { "on" }))
            .collect();
        Explanation {
            summary: format!(
                "X-Cycle: {} or {} is {}, eliminating {}",
                start, end, digit, digit
            ),
            detail: format!(
                "Follow {} through houses where it has only two spots and cells that see each \
                 other: {}. If {} isn't {}, {} is, so one of them holds it. \
                 {} can be eliminated from cells seeing both.",
                digit,
                steps.join(" → "),
                start,
                digit,
                end,
                digit
            ),
            highlights: vec![chain_highlights(chain, digit)],
        }
    }

    /// `chain` runs from the cell assumed not to be `z` to the cell forced to `z`.
    pub fn xy_chain(z: u8, chain: &[CellId]) -> Self {
        let (start, end) = (chain[0], chain[chain.len() - 1]);
        let path: Vec<String> = chain.iter().map(|c| c.to_string()).collect();
        Explanation {
            summary: format!("XY-Chain: {} or {} is {}, eliminating {}", start, end, z, z),
            detail: format!(
                "Each cell in {} has two candidates and sees the next. If {} isn't {}, \
                 every cell along the chain is forced in turn until {} is {}. \
                 {} can be eliminated from cells seeing both ends.",
                path.join(" → "),
                start,
                z,
                end,
                z,
                z
            ),
            highlights: vec![chain_highlights(chain, z)],
        }
    }

    pub fn xyz_wing(
        pivot: CellId,
        wing1: CellId,
        wing2: CellId,
        pivot_digits: Candidates,
        z: u8,
    ) -> Self {
        let digits: Vec<String> = pivot_digits.iter().map(|d| d.to_string()).collect();
        Explanation {
            summary: format!(
                "XYZ-Wing: pivot {} {{{}}}, wings {} and {} eliminate {}",
                pivot,
                digits.join(", "),
                wing1,
                wing2,
                z
            ),
            detail: format!(
                "{} must be {}. Whichever it is, {}, {} or {} is {}. \
                 {} can be eliminated from cells seeing all three.",
                pivot,
                digits.join(", "),
                pivot,
                wing1,
                wing2,
                z,
                z
            ),
            highlights: vec![marked(
                z,
                [
                    (pivot, HighlightRole::Focus),
                    (wing1, HighlightRole::Supporting),
                    (wing2, HighlightRole::Supporting),
                ],
            )],
        }
    }
}

pub trait Strategy {
    fn name(&self) -> &'static str;
    /// 1 (easiest) and up; the solver tries strategies in this order.
    fn difficulty(&self) -> u8;
    fn find(&self, board: &Board, ctx: &SolverContext) -> Option<Deduction>;
}

pub struct Deduction {
    pub strategy: &'static str,
    pub difficulty: u8,
    pub placements: Vec<(CellId, u8)>,
    pub eliminations: Vec<(CellId, u8)>,
    pub explanation: Explanation,
}

/// The chain's ends as focus, the cells between as supporting.
fn chain_highlights(chain: &[CellId], digit: u8) -> Highlights {
    let last = chain.len() - 1;
    marked(
        digit,
        chain.iter().enumerate().map(|(i, &cell)| {
            let role = if i == 0 || i == last {
                HighlightRole::Focus
            } else {
                HighlightRole::Supporting
            };
            (cell, role)
        }),
    )
}

/// Each cell, and its `digit` candidate, highlighted with its role.
fn marked(digit: u8, cells: impl IntoIterator<Item = (CellId, HighlightRole)>) -> Highlights {
    let (cells, candidates) = cells
        .into_iter()
        .map(|(cell, role)| {
            (
                CellHighlight { cell, role },
                CandidateHighlight { cell, digit, role },
            )
        })
        .unzip();
    Highlights { cells, candidates }
}

/// "R1C1, R2C5, ..."
fn cell_list(cells: &[CellId]) -> String {
    cells
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
