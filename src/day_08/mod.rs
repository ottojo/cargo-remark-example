use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

extern crate nalgebra as na;

type DMatrixi32 = na::OMatrix<i32, na::Dyn, na::Dyn>;

fn visible_trees_from_top(map: &DMatrixi32) -> Vec<(usize, usize)> {
    let mut coords = vec![];

    for col in 0..map.shape().1 {
        let mut current_max = -1;

        for row in 0..map.shape().0 {
            if map[(row, col)] > current_max {
                coords.push((row, col));
                current_max = map[(row, col)];
                continue;
            }
        }
    }

    coords
}

// Coords of the same point after rotating matrix to left
fn rotate_left_coords(_rows: usize, cols: usize, c: (usize, usize)) -> (usize, usize) {
    let (row, col) = c;
    (cols - 1 - col, row)
}

// Coords of the same point after rotating matrix to right
fn rotate_right_coords(rows: usize, _cols: usize, c: (usize, usize)) -> (usize, usize) {
    let (row, col) = c;
    (col, rows - 1 - row)
}

fn rotated_right_mat(m: &DMatrixi32) -> DMatrixi32 {
    let (rows, cols) = m.shape();
    DMatrixi32::from_fn(cols, rows, |row, col| {
        m[rotate_left_coords(cols, rows, (row, col))]
    })
}

fn rotated_left_mat(m: &DMatrixi32) -> DMatrixi32 {
    let (rows, cols) = m.shape();
    DMatrixi32::from_fn(cols, rows, |row, col| {
        m[rotate_right_coords(cols, rows, (row, col))]
    })
}

fn visible_from_left(map: &DMatrixi32) -> Vec<(usize, usize)> {
    let rotated = rotated_right_mat(map);
    let (r_rows, r_cols) = rotated.shape();

    visible_trees_from_top(&rotated)
        .iter()
        .map(|(row, col)| rotate_left_coords(r_rows, r_cols, (*row, *col)))
        .collect()
}

fn visible_from_right(map: &DMatrixi32) -> Vec<(usize, usize)> {
    let rotated = rotated_left_mat(map);
    let (r_rows, r_cols) = rotated.shape();

    visible_trees_from_top(&rotated)
        .iter()
        .map(|(row, col)| rotate_right_coords(r_rows, r_cols, (*row, *col)))
        .collect()
}

fn visible_from_bottom(map: &DMatrixi32) -> Vec<(usize, usize)> {
    let rotated = rotated_left_mat(&rotated_left_mat(map));
    let (r_rows, r_cols) = rotated.shape();

    visible_trees_from_top(&rotated)
        .iter()
        .map(|(row, col)| {
            rotate_right_coords(
                r_cols,
                r_rows,
                rotate_right_coords(r_rows, r_cols, (*row, *col)),
            )
        })
        .collect()
}

fn visible_trees(map: &DMatrixi32) -> HashSet<(usize, usize)> {
    let mut hs = HashSet::new();
    hs.extend(visible_trees_from_top(map).iter());
    hs.extend(visible_from_left(map).iter());
    hs.extend(visible_from_right(map).iter());
    hs.extend(visible_from_bottom(map).iter());

    hs
}

fn parse_map(input: Vec<String>) -> DMatrixi32 {
    let cols = input[0].len();

    let mut rows = vec![];
    for r in input {
        rows.push(na::RowDVector::from_row_iterator(
            cols,
            r.chars().map(|f| f.to_digit(10).unwrap() as i32),
        ));
    }

    DMatrixi32::from_rows(rows.as_slice())
}

fn viewing_distance_up(map: &DMatrixi32, coordinates: (usize, usize)) -> usize {
    let (row, col) = coordinates;

    let mut distance = 0;
    let treehouse_height = map[coordinates];

    for row in (0..row).rev() {
        distance += 1;
        if map[(row, col)] >= treehouse_height {
            break;
        }
    }

    distance
}

fn viewing_distance_down(map: &DMatrixi32, coordinates: (usize, usize)) -> usize {
    let (row, col) = coordinates;

    let mut distance = 0;
    let treehouse_height = map[coordinates];

    for row in (row + 1)..map.shape().0 {
        distance += 1;
        if map[(row, col)] >= treehouse_height {
            break;
        }
    }

    distance
}

fn viewing_distance_left(map: &DMatrixi32, coordinates: (usize, usize)) -> usize {
    let (row, col) = coordinates;

    let mut distance = 0;
    let treehouse_height = map[coordinates];

    for col in (0..col).rev() {
        distance += 1;
        if map[(row, col)] >= treehouse_height {
            break;
        }
    }

    distance
}

fn viewing_distance_right(map: &DMatrixi32, coordinates: (usize, usize)) -> usize {
    let (row, col) = coordinates;

    let mut distance = 0;
    let treehouse_height = map[coordinates];

    for col in (col + 1)..map.shape().1 {
        distance += 1;
        if map[(row, col)] >= treehouse_height {
            break;
        }
    }

    distance
}

fn visibility_score(map: &DMatrixi32, coordinates: (usize, usize)) -> usize {
    viewing_distance_up(map, coordinates)
        * viewing_distance_left(map, coordinates)
        * viewing_distance_right(map, coordinates)
        * viewing_distance_down(map, coordinates)
}

pub fn run() {
    let file = File::open("./src/day_08/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut lines_vec = vec![];
    for line in lines {
        lines_vec.push(line.unwrap())
    }

    let map = parse_map(lines_vec);

    let visible = visible_trees(&map);
    let visible_count = visible.len();
    println!("Day 8: {visible_count} trees are visible from the outside.");

    let mut max_visibility_score = 0;
    for row in 0..map.shape().0 {
        for col in 0..map.shape().1 {
            let vs = visibility_score(&map, (row, col));
            if vs > max_visibility_score {
                max_visibility_score = vs;
            }
        }
    }
    println!("       The highest possible scenic score is {max_visibility_score}")
}
