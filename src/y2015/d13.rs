use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
// use std::str;
// All permutations with Heaps Algorithm?
//
// procedure permutations(n : integer, A : array of any):
//     // c is an encoding of the stack state.
//     // c[k] encodes the for-loop counter for when permutations(k + 1, A) is called
//     c : array of int

//     for i := 0; i < n; i += 1 do
//         c[i] := 0
//     end for

//     output(A)

//     // i acts similarly to a stack pointer
//     i := 1;
//     while i < n do
//         if  c[i] < i then
//             if i is even then
//                 swap(A[0], A[i])
//             else
//                 swap(A[c[i]], A[i])
//             end if
//             output(A)
//             // Swap has occurred ending the while-loop. Simulate the increment of the while-loop counter
//             c[i] += 1
//             // Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
//             i := 1
//         else
//             // Calling permutations(i+1, A) has ended as the while-loop terminated. Reset the state and simulate popping the stack by incrementing the pointer.
//             c[i] := 0
//             i += 1
//         end if
//     end while

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut persons: HashMap<&str, HashMap<&str, i64>> = HashMap::with_capacity(10);
    for line in content.lines() {
        let splitted: Vec<&str> = line.split(' ').collect();
        println!(
            "{}->{} {} {}",
            splitted[0],
            splitted[splitted.len() - 1].strip_suffix(".").unwrap(),
            splitted[2],
            splitted[3]
        );
        persons.entry(splitted[0]).or_default().insert(
            splitted[splitted.len() - 1].strip_suffix('.').unwrap(),
            splitted[3].parse::<i64>()? * if splitted[2] == "lose" { -1 } else { 1 },
        );
    }
    dbg!(&persons);

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
