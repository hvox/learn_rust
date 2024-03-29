// segment tree without range updates
//
// supported operations:
//   O(log(n)) - set value of an element
//   O(log(n)) - get value of an element
//   O(log(n)) - get sum/max/min on range
//
// [:::::::::::::::0::::::::::::::::::]
// [::::::1:::::::] [::::::::2::::::::]
// [::3::] [::4:::] [:::5:::] [:::6:::]
// [7] [8] [9] [10] [11] [12] [13] [14] <- indexes of nodes
// {0} {1} {2} {3}  {4}  {5}  {6}  {7} <- indexes of stored elements
//
// navigation:
//   (k - 1) >> 1 - parent
//   k * 2 + 1    - first child
//   k * 2 + 2    - second child
//   i + (N >> 1) - index of element into index of node
//   k - (N >> 1) - node index to element's index
//   where N is a number of nodes in a tree
//     2n-1 - number of nodes in best case(n is power of two)
//     4n-5 - number of nodes in worst case(n = 2^k + 1)
//

fn segment_tree(elements: Vec<i64>) -> Vec<i64> {
	let n = elements.len().next_power_of_two() * 2 - 1;
	let mut tree = vec![0; n];
	for (i, value) in elements.into_iter().enumerate() {
		tree[n / 2 + i] = value;
	}
	for i in (0..(n / 2)).rev() {
		tree[i] = tree[i * 2 + 1] + tree[i * 2 + 2];
	}
	tree
}

fn set_nth(tree: &mut Vec<i64>, i: usize, value: i64) {
	let mut i = tree.len() / 2 + i;
	tree[i] = value;
	while i > 0 {
		i = (i - 1) / 2;
		tree[i] = tree[2 * i + 1] + tree[2 * i + 2];
	}
}

fn nth(tree: &Vec<i64>, i: usize) -> i64 { tree[tree.len() / 2 + i] }

fn sum(tree: &Vec<i64>, first: usize, last: usize) -> i64 {
	fn f(tree: &Vec<i64>, i: usize, l: usize, r: usize, first: usize, last: usize) -> i64 {
		if first > r || last < l {
			0
		} else if first <= l && r <= last {
			tree[i]
		} else {
			f(tree, i * 2 + 1, l, (l + r) / 2, first, last)
				+ f(tree, i * 2 + 2, (l + r) / 2 + 1, r, first, last)
		}
	}
	f(tree, 0, 0, tree.len() / 2, first, last)
}

#[test]
fn test_representation() {
	let tree = segment_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	assert_eq!(tree, [31, 9, 22, 4, 5, 14, 8, 3, 1, 4, 1, 5, 9, 2, 6]);
}

#[test]
fn test_setter_and_getter() {
	let mut tree = segment_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	set_nth(&mut tree, 2, -42);
	let elements: Vec<i64> = (0..8).map(|i| nth(&tree, i)).collect();
	assert_eq!(elements, [3, 1, -42, 1, 5, 9, 2, 6]);
}

#[test]
fn test_sum_on_range() {
	let tree = segment_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	let sums: Vec<i64> = (2..8).map(|i| sum(&tree, i - 2, i)).collect();
	assert_eq!(sums, [8, 6, 10, 15, 16, 17]);
}

fn main() {
	use std::io;
	use std::io::*;
	let input = io::stdin().lock().lines().next().unwrap().unwrap();
	let array: Vec<i64> = input.split(' ').map(|w| w.parse().unwrap()).collect();
	let n = array.len();
	let mut tree = segment_tree(array);
	println!("nodes = {:?}", tree);
	println!("elements = {:?}", (0..n).map(|i| nth(&tree, i)).collect::<Vec<_>>());
	println!("elements[2] = 42");
	set_nth(&mut tree, 2, 42);
	println!("elements = {:?}", (0..n).map(|i| nth(&tree, i)).collect::<Vec<_>>());
	println!("perfix sums: {:?}", (0..n).map(|i| sum(&tree, 0, i)).collect::<Vec<_>>());
}
