use core::iter::StepBy;
use core::slice::{Iter, IterMut};
use std::fmt;
use std::marker::Copy;

#[derive(Clone)]
pub struct Grid<T> {
    g: Vec<T>,
    pub width: i32,
    pub height: i32,
}

impl<T> Grid<T>
where
    T: Clone,
    T: Copy,
{
    pub fn new(g: Vec<Vec<T>>) -> Self {
        Self {
            width: g[0].len() as i32,
            height: g.len() as i32,
            g: g.into_iter().flatten().collect::<Vec<T>>(),
        }
    }

    pub fn size(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn at(&self, x: i32, y: i32) -> Option<&T> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let idx = y * self.width + x;
            Some(&self.g[idx as usize])
        } else {
            None
        }
    }

    pub fn at_mut(&mut self, x: i32, y: i32) -> &mut T {
        assert!(x < self.width && y < self.height);
        let i: usize = (x + y * self.width) as usize;
        &mut self.g[i as usize]
    }

    pub fn at_unsafe(&self, x: i32, y: i32) -> &T {
        let idx = y * self.width + x;
        &self.g[idx as usize]
    }

    pub fn iter(&self) -> Iter<T> {
        self.g.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.g.iter_mut()
    }

    pub fn iter_points(&self) -> GridPointsIter<T> {
        GridPointsIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn row_slice(&self, r: i32) -> &[T] {
        let start: usize = (r * self.width) as usize;
        let end = start + self.width as usize;
        &self.g[start..end]
    }

    pub fn row_iter(&self) -> GridRowIterator<T> {
        GridRowIterator { grid: self, row: 0 }
    }

    pub fn row(&self, i: i32) -> Iter<T> {
        assert!(i < self.height);
        let start: usize = (i * self.width) as usize;
        let end: usize = start + self.width as usize;
        self.g[start..end].into_iter()
    }

    pub fn row_mut(&mut self, i: i32) -> IterMut<T> {
        assert!(i < self.height);
        let start: usize = (i * self.width) as usize;
        let end: usize = start + self.width as usize;
        self.g[start..end].iter_mut()
    }

    pub fn col(&self, i: i32) -> StepBy<Iter<T>> {
        assert!(i < self.width);
        assert!(self.width >= 0);
        let start: usize = i as usize;
        self.g[start..].iter().step_by(self.width as usize)
    }

    pub fn col_mut(&mut self, i: i32) -> StepBy<IterMut<T>> {
        assert!(i < self.width);
        assert!(self.width >= 0);
        let start: usize = i as usize;
        self.g[start..].iter_mut().step_by(self.width as usize)
    }

    pub fn line(&self, p1: (i32, i32), p2: (i32, i32)) -> GridLineIter<T> {
        let dx: i32 = p1.0 - p2.0;
        let dy: i32 = p1.1 - p2.1;

        let (mut startx, mut starty) = if p1.0 < p2.0 { p1 } else { p2 };

        while self.in_bounds(startx, starty) {
            startx += dx;
            starty += dy;
        }

        GridLineIter {
            grid: self,
            start: (startx - dx, starty - dy),
            dxy: (dx, dy),
        }
    }

    pub fn transpose(&mut self) -> &mut Self {
        let mut t: Vec<T> = Vec::with_capacity((self.width * self.height) as usize);

        for i in 0..self.width {
            t.extend(self.col(i).collect::<Vec<_>>());
        }
        core::mem::swap(&mut self.height, &mut self.width);
        self.g = t;
        self
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.g.reverse();
        self
    }

    pub fn walk<F>(&mut self, mut start: (i32, i32), f: F)
    where
        F: Fn(&mut T, &(i32, i32)) -> Option<(i32, i32)>,
    {
        let mut val: &mut T = self.at_mut(start.0, start.1);
        while let Some(coords) = f(val, &start) {
            if self.in_bounds(coords.0, coords.1) {
                val = self.at_mut(coords.0, coords.1);
                start = coords;
            } else {
                break;
            }
        }
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: Clone + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in self.row_iter() {
            for v in r.iter() {
                write!(f, "{}", v)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub struct GridRowIterator<'g, T> {
    grid: &'g Grid<T>,
    row: usize,
}

impl<'g, T> Iterator for GridRowIterator<'g, T>
where
    T: Clone + Copy,
{
    type Item = &'g [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.grid.height as usize {
            let result = Some(self.grid.row_slice(self.row as i32));
            self.row += 1;
            result
        } else {
            None
        }
    }
}

pub struct GridPointsIter<'g, T> {
    grid: &'g Grid<T>,
    x: i32,
    y: i32,
}

impl<'g, T> Iterator for GridPointsIter<'g, T>
where
    T: Clone + Copy,
{
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.in_bounds(self.x, self.y) {
            let result = (self.x, self.y);
            self.x = (self.x + 1) % self.grid.width;
            if self.x == 0 {
                self.y += 1;
            }
            Some(result)
        } else {
            None
        }
    }
}

pub struct GridLineIter<'g, T> {
    grid: &'g Grid<T>,
    start: (i32, i32),
    dxy: (i32, i32),
}

impl<'g, T> Iterator for GridLineIter<'g, T>
where
    T: Clone + Copy,
{
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.in_bounds(self.start.0, self.start.1) {
            let result = (self.start.0, self.start.1);
            self.start = (self.start.0 - self.dxy.0, self.start.1 - self.dxy.1);
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let g = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        //Test access
        let mut g: Grid<i32> = Grid::<i32>::new(g);
        assert_eq!(Some(&5), g.at(1, 1));
        assert_eq!(5, *g.at_unsafe(1, 1));

        let v = g.at_mut(1, 1);
        assert_eq!(5, *v);
        *v += 1;
        assert_eq!(6, *g.at_unsafe(1, 1));

        //Test row iter
        let expected: Vec<i32> = vec![4, 6, 6];
        let result: Vec<i32> = g.row(1).cloned().collect::<Vec<_>>();
        assert_eq!(expected, result);

        //Test row_mut iter
        let mut result: Vec<&mut i32> = g.row_mut(1).collect::<Vec<_>>();
        *result[0] += 1;

        let expected: Vec<i32> = vec![5, 6, 6];
        let result: Vec<i32> = g.row(1).cloned().collect::<Vec<_>>();
        assert_eq!(expected, result);

        //Test col iter
        let expected: Vec<i32> = vec![2, 6, 8];
        let result: Vec<i32> = g.col(1).cloned().collect::<Vec<_>>();
        assert_eq!(expected, result);

        //Test col_mut iter
        let mut result: Vec<&mut i32> = g.col_mut(2).collect::<Vec<_>>();
        *result[0] += 1;

        let expected: Vec<i32> = vec![4, 6, 9];
        let result: Vec<i32> = g.col(2).cloned().collect::<Vec<_>>();
        assert_eq!(expected, result);

        //Test walk
        // println!("{}\n", g);
        // g.walk((1, 1), |v, &(x, y)| {
        //     println!("{}", v);
        //     *v = 0;
        //     Some((x - 1, y - 1))
        // });
        // println!("\n{}", g);
        //
        // let v: Vec<Vec<char>> = vec![
        //     vec!['a', 'b', 'c'],
        //     vec!['d', 'e', 'f'],
        //     vec!['g', 'h', 'i'],
        // ];
        // let mut g: Grid<char> = Grid::new(v);
        // println!("{}\n", g);
        // g.walk((0, 0), |v, &(x, y)| {
        //     println!("{}", v);
        //     *v = v.to_ascii_uppercase();
        //     Some((x + 1, y))
        // });
        // println!("\n{}\n", g);
        //

        //Test iter_points
        let grid: Grid<i32> = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let expected: Vec<(i32, i32)> = vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)];
        assert_eq!(expected, grid.iter_points().collect::<Vec<_>>());
    }

    #[test]
    fn test_line_iter() {
        let g = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let grid: Grid<i32> = Grid::new(g);

        let expected = vec![(0, 0), (1, 1), (2, 2), (3, 3)];
        let result: Vec<(i32, i32)> = grid.line((1, 1), (2, 2)).collect();
        assert_eq!(expected, result);

        let expected = vec![(0, 3), (1, 2), (2, 1), (3, 0)];
        let result: Vec<(i32, i32)> = grid.line((1, 2), (2, 1)).collect();
        assert_eq!(expected, result);
    }
}
