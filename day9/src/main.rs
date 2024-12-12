use std::{fs::File, io::Read};

#[derive(Clone, Copy, Debug)]
struct FileLayout {
    id: Option<u32>,
    len: usize,
}

fn main() {
    let mut string = String::new();
    match File::open("day9/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(data.as_slice());
    part2(data.as_slice());
}

fn parse_data(string: &str) -> Vec<FileLayout> {
    string
        .trim_end()
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, slice)| match slice.len() {
            2 => vec![
                FileLayout {
                    id: Some(id as u32),
                    len: slice[0].to_digit(10).unwrap() as usize,
                },
                FileLayout {
                    id: None,
                    len: slice[1].to_digit(10).unwrap() as usize,
                },
            ],
            1 => vec![FileLayout {
                id: Some(id as u32),
                len: slice[0].to_digit(10).unwrap() as usize,
            }],
            _ => panic!(),
        })
        .collect()
}

fn part1(data: &[FileLayout]) {
    //create the actual blockmap
    let mut blocks = Vec::new();
    for file_layout in data {
        if file_layout.len > 0 {
            blocks.append(&mut vec![file_layout.id; file_layout.len as usize]);
        }
    }

    let mut fwd_idx = 0;
    let mut back_idx = blocks.len() - 1;
    while blocks.as_slice()[fwd_idx..back_idx]
        .into_iter()
        .any(|e| e.is_none())
    {
        while let Some(_) = blocks[fwd_idx] {
            fwd_idx = fwd_idx + 1;
        }
        blocks[fwd_idx] = blocks[back_idx];
        blocks[back_idx] = None;
        back_idx = back_idx - 1;
    }

    let final_result = blocks
        .into_iter()
        .enumerate()
        .fold(0u64, |acc, (idx, id)| match id {
            Some(id) => acc + (idx as u64 * id as u64),
            None => acc,
        });
    println!("Day 9 Part 1 result: {final_result}");
}

fn part2(data: &[FileLayout]) {
    //create the actual blockmap
    let mut blocks = Vec::from(data);

    let mut back_idx = blocks.len() - 1;
    while back_idx > 0 {
        if let Some(_) = blocks[back_idx].id {
            let mut insert_idx = 0;
            loop {
                if let None = blocks[insert_idx].id {
                    if blocks[insert_idx].len > blocks[back_idx].len {
                        let free_size = blocks[insert_idx].len;
                        let file_size = blocks[back_idx].len;
                        blocks[insert_idx] = blocks[back_idx];
                        blocks[back_idx] = FileLayout {
                            id: None,
                            len: file_size,
                        };
                        blocks.insert(
                            insert_idx + 1,
                            FileLayout {
                                id: None,
                                len: free_size - file_size,
                            },
                        );
                        break;
                    }

                    if blocks[insert_idx].len == blocks[back_idx].len {
                        blocks[insert_idx] = blocks[back_idx];
                        blocks[back_idx] = FileLayout {
                            id: None,
                            len: blocks[insert_idx].len,
                        };
                        break;
                    }
                }

                if insert_idx == back_idx {
                    break;
                }

                insert_idx += 1;
            }
        }
        back_idx -= 1;
    }

    //create the actual blockmap for checksumming, too lazy to just use the existing strucutres
    let mut blocks_expanded = Vec::new();
    for file_layout in blocks {
        if file_layout.len > 0 {
            blocks_expanded.append(&mut vec![file_layout.id; file_layout.len as usize]);
        }
    }

    let final_result =
        blocks_expanded
            .into_iter()
            .enumerate()
            .fold(0u64, |acc, (idx, id)| match id {
                Some(id) => acc + (idx as u64 * id as u64),
                None => acc,
            });
    println!("Day 9 Part 2 result: {final_result}");
}
