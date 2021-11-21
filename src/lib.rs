use pyo3::prelude::*;

#[pyfunction]
fn levensthein(str1: String, str2: String) -> usize{
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
        }
        
    }
    return v[str1_len - 1][str2_len - 1];
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn fastlevensthein(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levensthein, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levensthein() {
        assert_eq!(3 as usize, levensthein(String::from("kitten"), String::from("sitting")));
        assert_eq!(3 as usize, levensthein(String::from("akitten"), String::from("asitting")));
        assert_eq!(0, levensthein(String::from("examen"), String::from("examen")));
        assert_eq!(1, levensthein(String::from("examen"), String::from("examan")));
        assert_eq!(4, levensthein(String::from("niche"), String::from("chien")));
        assert_eq!(1, levensthein(String::from("test"), String::from("tests")));
        assert_eq!(5, levensthein(String::from("chiens"), String::from("niche")));
        assert_eq!(5, levensthein(String::from("niche"), String::from("chiens")));
    }
}

