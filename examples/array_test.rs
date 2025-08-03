#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_array_literals()  -> serde_json::Value {
    let mut arr1 = [1, 2, 3, 4, 5];
    let mut arr2 = [0, 0, 0, 0];
    let mut arr3 = [true, false, true];
    let mut arr4 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    return(arr1, arr2, arr3, arr4);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_array_multiplication()  -> serde_json::Value {
    let mut zeros = [0;
    10];
    let mut ones = [1;
    5];
    let mut pattern = [42;
    8];
    let mut reverse_zeros = [0;
    10];
    let mut reverse_ones = [1;
    5];
    return(zeros, ones, pattern, reverse_zeros, reverse_ones);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_array_init_functions()  -> serde_json::Value {
    let mut z = [0;
    10];
    let mut o = [1;
    5];
    let mut f = [42;
    8];
    return(z, o, f);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_large_arrays()  -> serde_json::Value {
    let mut large  = ([0] * 50);
    let mut very_large  = ([1] * 100);
    let mut x = 5;
    let mut dynamic = [x;
    10];
    return(large, very_large, dynamic);
   
}
#[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn test_nested_arrays()  -> serde_json::Value {
    let mut matrix = vec ! [[0, 0], [0, 0], [0, 0]];
    let mut identity = vec ! [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    return(matrix, identity)
}