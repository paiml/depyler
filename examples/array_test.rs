#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_literals() -> (Vec<i32>, Vec<i32>, Vec<bool>, Vec<i32>) {
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![0, 0, 0, 0];
    let arr3 = vec![true, false, true];
    let arr4 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    (arr1, arr2, arr3, arr4)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_multiplication() -> ([i32; 10], [i32; 5], [i32; 8], [i32; 10], [i32; 5]) {
    let _cse_temp_0 = [0; 10];
    let zeros = _cse_temp_0;
    let _cse_temp_1 = [1; 5];
    let ones = _cse_temp_1;
    let _cse_temp_2 = [42; 8];
    let pattern = _cse_temp_2;
    let _cse_temp_3 = [0; 10];
    let reverse_zeros = _cse_temp_3;
    let _cse_temp_4 = [1; 5];
    let reverse_ones = _cse_temp_4;
    (zeros, ones, pattern, reverse_zeros, reverse_ones)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_array_init_functions() -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let z = vec![0; 10];
    let o = vec![1; 5];
    let f = vec![42; 8];
    (z, o, f)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_large_arrays() -> (Vec<i32>, Vec<i32>, [i32; 10]) {
    let _cse_temp_0 = vec![0; 50];
    let large = _cse_temp_0;
    let _cse_temp_1 = vec![1; 100];
    let very_large = _cse_temp_1;
    let x = 5;
    let _cse_temp_2 = [x; 10];
    let dynamic = _cse_temp_2;
    (large, very_large, dynamic)
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_nested_arrays() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let matrix = vec![vec![0, 0], vec![0, 0], vec![0, 0]];
    let identity = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    (matrix, identity)
}
