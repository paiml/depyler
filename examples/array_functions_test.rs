#[doc = " Depyler: proven to terminate"] pub fn test_array_functions()  -> Result<serde_json::Value, IndexError>{
    let mut z1 = [0;
    5];
    let mut o1 = [1;
    10];
    let mut f1 = [42;
    8];
    let mut z2 = vec ! [0;
    100 as usize];
    return Ok(((z1.get(0 as usize).copied().unwrap_or_default() + o1.get(0 as usize).copied().unwrap_or_default()) + f1.get(0 as usize).copied().unwrap_or_default()))
}