
use std::mem;

fn levensthein(mut str1: String, mut str2: String) -> usize{
    let mut first_min: usize;
    let mut second_min: usize;

    if str1.len() < str2.len() {
        mem::swap(&mut str1, &mut str2);
    }

    let mut v: Vec<Vec<usize>> = vec![vec![Default::default(); str2.len()]; str1.len()];
    for _i in 1..str1.len(){
        v[_i] = vec![0; str2.len()];
        for _j in 1..str2.len(){
            if str1.chars().nth(_i) == str2.chars().nth(_j) {
                v[_i][_j] = v[_i-1][_j-1];
            }
            else {
                //println!("{0}", [v[_i][_j - 1],v[_i - 1][_j]].iter().min().unwrap());
                first_min = *[v[_i][_j-1],v[_i-1][_j]].iter().min().unwrap();
                second_min = v[_i - 1][_j - 1];
                v[_i][_j] = 1 + *[first_min, second_min].iter().min().unwrap();
            }
        }
        
    }
    println!("{0}", v[str1.len() - 1][str2.len() - 1]);
    return v[str1.len() - 1][str2.len() - 1];
}


fn main() {
    println!("Hello, world!");
    let res : usize;
    res = levensthein(String::from("kitten"), String::from("sitting"));
    println!("{0}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levensthein() {
        assert_eq!(3 as usize, levensthein(String::from("kitten"), String::from("sitting")));
        assert_eq!(0, levensthein(String::from("examen"), String::from("examen")));
        assert_eq!(1, levensthein(String::from("examen"), String::from("examan")));
        assert_eq!(5, levensthein(String::from("niche"), String::from("chiens")));
        assert_eq!(1, levensthein(String::from("test"), String::from("tests")));
    }
}

