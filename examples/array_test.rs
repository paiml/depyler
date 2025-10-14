#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_array_literals()  -> DynamicType {
    let arr1 = vec ! [1, 2, 3, 4, 5];
    let arr2 = vec ! [0, 0, 0, 0];
    let arr3 = vec ! [true, false, true];
    let arr4 = vec ! [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    return(arr1, arr2, arr3, arr4);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_array_multiplication()  -> DynamicType {
    let _cse_temp_0 = [0;
    10];
    let zeros = _cse_temp_0;
    let _cse_temp_1 = [1;
    5];
    let ones = _cse_temp_1;
    let _cse_temp_2 = [42;
    8];
    let pattern = _cse_temp_2;
    let _cse_temp_3 = [0;
    10];
    let reverse_zeros = _cse_temp_3;
    let _cse_temp_4 = [1;
    5];
    let reverse_ones = _cse_temp_4;
    return(zeros, ones, pattern, reverse_zeros, reverse_ones);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_array_init_functions()  -> DynamicType {
    let z = [0;
    10];
    let o = [1;
    5];
    let f = [42;
    8];
    return(z, o, f);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_large_arrays()  -> DynamicType {
    let _cse_temp_0 = vec ! [0] * 50;
    let large = _cse_temp_0;
    let _cse_temp_1 = vec ! [1] * 100;
    let very_large = _cse_temp_1;
    let _cse_temp_2 = [5;
    10];
    let dynamic = _cse_temp_2;
    return(large, very_large, dynamic);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_nested_arrays()  -> DynamicType {
    let matrix = vec ! [vec ! [0, 0], vec ! [0, 0], vec ! [0, 0]];
    let identity = vec ! [vec ! [1, 0, 0], vec ! [0, 1, 0], vec ! [0, 0, 1]];
    return(matrix, identity)
}