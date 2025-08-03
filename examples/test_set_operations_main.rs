use std::collections::HashSet;

mod test_set_operations_v2;

fn main() {
    // Test set intersection
    let intersection = test_set_operations_v2::test_set_intersection();
    println!("Intersection of {{1,2,3,4,5}} & {{4,5,6,7,8}} = {:?}", intersection);
    
    // Test set union
    let union = test_set_operations_v2::test_set_union();
    println!("Union of {{1,2,3}} | {{3,4,5}} = {:?}", union);
    
    // Test set difference
    let difference = test_set_operations_v2::test_set_difference();
    println!("Difference of {{1,2,3,4,5}} - {{4,5,6}} = {:?}", difference);
    
    // Test symmetric difference
    let sym_diff = test_set_operations_v2::test_set_symmetric_difference();
    println!("Symmetric difference of {{1,2,3,4}} ^ {{3,4,5,6}} = {:?}", sym_diff);
    
    // Test create_sets
    let (s1, s2) = test_set_operations_v2::create_sets();
    println!("Created sets: {:?} and {:?}", s1, s2);
}