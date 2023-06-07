#![allow(unused)]

fn merge_sort(mut list: Vec<usize>) -> Vec<usize> {
    if list.len() == 1 {
        return list;
    } else if list.len() == 2 {
        list.sort();

        return list;
    }

    let rest = list.len() % 2;

    let sorted_chunks = list
        .chunks(match rest {
            0 => list.len() / 2,
            _ => (list.len() + 1) / 2,
        })
        .map(|d| merge_sort(d.to_vec()))
        .collect::<Vec<_>>();

    let mut res = vec![];
    let left = &sorted_chunks[0];
    let right = &sorted_chunks[1];
    let mut left_cursor = 0;
    let mut right_cursor = 0;

    loop {
        let lv = left[left_cursor];
        let rv = right[right_cursor];

        if lv < rv {
            res.push(lv);
            left_cursor += 1;
        } else {
            res.push(right[right_cursor]);
            right_cursor += 1;
        }

        if right_cursor > right.len() - 1 {
            res.append(&mut left[left_cursor..left.len()].to_vec());

            break;
        } else if left_cursor > left.len() - 1 {
            res.append(&mut right[right_cursor..right.len()].to_vec());

            break;
        }
    }

    res
}

#[test]
fn test_merge_sort() {
    let test = vec![3, 2, 5, 4, 6, 8, 0, 1, 2, 6];

    let res = merge_sort(test);

    assert_eq!(res, vec![0, 1, 2, 2, 3, 4, 5, 6, 6, 8]);
}
