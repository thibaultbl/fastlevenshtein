use pyo3::prelude::*;

#[pyfunction]
fn levenshtein(str1: String, str2: String, exit_after: usize) -> usize{
    let mut first_min: usize;
    let mut second_min: usize;

    let str1_len = str1.len() + 1;
    let str2_len = str2.len() + 1;

    let mut v: Vec<Vec<usize>> = vec![vec![Default::default(); str2_len]; str1_len];

    for _i in 0..str1_len {
        v[_i][0] = _i;
    }
    for _j in 0..str2_len {
        v[0][_j] = _j;
    }


    for _i in 1..str1_len{
        for _j in 1..str2_len{
            if str1.chars().nth(_i - 1) == str2.chars().nth(_j - 1) {
                v[_i][_j] = v[_i-1][_j-1];
            }
            else {
                first_min = *[v[_i][_j-1],v[_i-1][_j]].iter().min().unwrap();
                second_min = v[_i - 1][_j - 1];
                v[_i][_j] = 1 + *[first_min, second_min].iter().min().unwrap();
            }
            if v[_i][_j] >= exit_after {
                return v[_i][_j]
            }
        }
        
    }
    return v[str1_len - 1][str2_len - 1];
}

#[pyfunction]
fn levenshtein_list(slice: Vec<&str>, str2: &str, exit_after: usize) -> Vec<usize>{
    let mut res = Vec::new();

    for i in 0..slice.len(){
        println!("{0}", String::from(slice[i]));
        //println!("{0}", &str2);
        
        res.push(levenshtein(String::from(slice[i]), String::from(str2), exit_after));
    }
    return res;
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn fastlevenshtein(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(levenshtein_list, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        assert_eq!(3 as usize, levenshtein(String::from("kitten"), String::from("sitting"), 999));
        assert_eq!(3 as usize, levenshtein(String::from("akitten"), String::from("asitting"), 999));
        assert_eq!(0, levenshtein(String::from("examen"), String::from("examen"), 999));
        assert_eq!(1, levenshtein(String::from("examen"), String::from("examan"), 999));
        assert_eq!(4, levenshtein(String::from("niche"), String::from("chien"), 999));
        assert_eq!(1, levenshtein(String::from("test"), String::from("tests"), 999));
        assert_eq!(5, levenshtein(String::from("chiens"), String::from("niche"), 999));
        assert_eq!(5, levenshtein(String::from("niche"), String::from("chiens"), 999));
        assert_eq!(3, levenshtein(String::from("aaaaaa"), String::from("bbbbbbbbb"), 3));
        assert_eq!(6, levenshtein(String::from("aabaaaaaa"), String::from("bbbbbbbbb"), 6));
    }

    #[test]
    fn test_levenshtein_list() {
        let mylist = vec!["kitten", "sitting", "akitten"];
        let mystring = "kitten";

        let expected = vec![0 as usize, 3 as usize, 1 as usize];

        assert_eq!(expected, levenshtein_list(mylist, mystring, 999));
    }
}

