extern crate spiral_matrix;
use spiral_matrix::*;

#[test]
fn empty_spiral() {
    assert_eq!(spiral_matrix(0), vec![] as Vec<Vec<u32>>);
}

#[ignore]
#[test]
fn trivial_spiral() {
    assert_eq!(spiral_matrix(1), vec![vec![1]]);
}

#[ignore]
#[test]
fn spiral_of_size_2() {
    assert_eq!(spiral_matrix(2), vec![vec![1, 2], vec![4, 3]]);
}

#[ignore]
#[test]
fn spiral_of_size_3() {
    assert_eq!(
        spiral_matrix(3),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );
}

#[ignore]
#[test]
fn spiral_of_size_4() {
    assert_eq!(
        spiral_matrix(4),
        vec![
            vec![1, 2, 3, 4],
            vec![12, 13, 14, 5],
            vec![11, 16, 15, 6],
            vec![10, 9, 8, 7],
        ]
    );
}

#[ignore]
#[test]
fn spiral_of_size_5() {
    assert_eq!(
        spiral_matrix(5),
        vec![
            vec![1, 2, 3, 4, 5],
            vec![16, 17, 18, 19, 6],
            vec![15, 24, 25, 20, 7],
            vec![14, 23, 22, 21, 8],
            vec![13, 12, 11, 10, 9],
        ]
    );
}
