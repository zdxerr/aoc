use std::fs;
// use std::io::BufReader;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    // let _reader = BufReader::new(fs::File::open(input_path)?);
    //
    let packages: Vec<u16> = content.lines().map(|line| line.parse()).flatten().collect();

    dbg!(&packages);

    for mut n in 0..3_usize.pow(packages.len() as u32) {
        // println!("{n}");
        // let mut v = vec![];
        let mut a = 0;
        let mut al = 0;
        let mut aqe = 1;
        let mut b = 0;
        let mut bl = 0;
        let mut c = 0;
        let mut cl = 0;
        // print!(".{n}");
        for p in 0..packages.len() {
            // let s = n.rem_euclid(3);
            // v.push(n.rem_euclid(3));
            // n = n.div_euclid(3);

            match n.rem_euclid(3) {
                0 => {
                    a += packages[p];
                    al += 1;
                    aqe *= packages[p];
                }
                1 => {
                    b += packages[p];
                    bl += 1;
                }
                2 => {
                    c += packages[p];
                    cl += 1;
                }
                _ => panic!("unexpected compartment: {}", n.rem_euclid(3)),
            }
            n = n.div_euclid(3);
        }
        println!("Not found {a} ({al}, {aqe}), {b} ({bl}), {c} ({cl})");

        if a == b && a == c {
            println!("Found {a} ({al}, {aqe}), {b} ({bl}), {c} ({cl})");
        }
        // println!(" _ {v:?}");
    }
    // def get_all_combinations_base3(n, values=[0, 1, 2]):
    //     total = 3 ** n
    //     result = []
    //     for i in range(total):
    //         comb = []
    //         num = i
    //         for _ in range(n):
    //             comb.append(values[num % 3])
    //             num //= 3
    //         result.append(comb[::-1])  # reverse to get correct order
    //     return result
    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
