use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut contents: HashMap<String, Vec<String>> = HashMap::new();

        for path in paths.into_iter() {
            let (dir, files) = path.split_once(' ').unwrap();

            for file in files.split(' ') {
                let (name, content) = file[..file.len() - 1].rsplit_once('(').unwrap();
                let full_path = String::from(dir) + "/" + name;

                contents.entry(String::from(content)).or_default().push(full_path);
            }
        }

        contents.into_values().filter(|v| v.len() > 1).collect()
    }
}

fn main() {
    let inputs = vec![
        vec!["root/a 1.txt(abcd) 2.txt(efgh)","root/c 3.txt(abcd)","root/c/d 4.txt(efgh)","root 4.txt(efgh)"],
        vec!["root/a 1.txt(abcd) 2.txt(efgh)","root/c 3.txt(abcd)","root/c/d 4.txt(efgh)"]
    ];

    for input in inputs {
        println!("{:?}", Solution::find_duplicate(input.into_iter().map(String::from).collect()));
    }
}
