#[doc = "Generate a number in range(simplified without random)."] #[doc = " Depyler: proven to terminate"] pub fn generate_number(min_val: i32, max_val: i32)  -> i32 {
    return((min_val + max_val) / 2);
   
}
#[doc = "Provide a hint based on the guess."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_hint(guess: i32, target: i32)  -> String {
    if(guess<target) {
    return "Marco!(Too low)".to_string();
   
}
else {
    if(guess>target) {
    return "Marco!(Too high)".to_string();
   
}
else {
    return "Polo!".to_string();
   
}
}
}
#[doc = "Calculate final score."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn calculate_score(attempts: i32, rounds: i32)  -> i32 {
    if(rounds == 0) {
    return 0;
   
}
let mut base_score  = (100 * rounds);
    let mut penalty  = (attempts * 5);
    let mut score  = (base_score - penalty);
    if(score<0) {
    return 0;
   
}
return score;
   
}
#[doc = "Get difficulty name from level."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn get_difficulty_name(level: i32)  -> & 'a str {
    if(level == 1) {
    return "Easy".to_string();
   
}
else {
    if(level == 2) {
    return "Medium".to_string();
   
}
else {
    if(level == 3) {
    return "Hard".to_string();
   
}
else {
    return "Unknown".to_string();
   
}
}
}
} #[doc = "Calculate average with safety check."] #[doc = " Depyler: proven to terminate"] pub fn calculate_average(total: i32, count: i32)  -> f64 {
    if(count == 0) {
    return 0;
   
}
return(total / count);
   
}
#[doc = "Format game statistics as string."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn format_statistics(score: i32, attempts: i32, rounds: i32)  -> String {
    let mut avg = calculate_average(attempts, rounds);
    let mut result = "Game Statistics:\n".to_string();
    result  = (((result + "Score: ".to_string()) + str(score)) + "\n".to_string());
    result  = (((result + "Attempts: ".to_string()) + str(attempts)) + "\n".to_string());
    result  = (((result + "Average: ".to_string()) + str(avg)) + "\n".to_string());
    return result;
   
}
#[doc = "Check if guess is in valid range."] #[doc = " Depyler: verified panic-free"] #[doc = " Depyler: proven to terminate"] pub fn validate_guess(guess: i32, min_val: i32, max_val: i32)  -> bool {
    if(guess<min_val) {
    return false;
   
}
if(guess>max_val) {
    return false;
   
}
return true;
   
}
#[doc = "Simulate a round with fixed guesses."] #[doc = " Depyler: verified panic-free"] pub fn play_simple_round(target: i32, max_attempts: i32)  -> i32 {
    let mut attempts = 0;
    let mut guess = 50;
    while(attempts<max_attempts) {
    attempts  = (attempts + 1);
    if(guess == target) {
    return attempts;
   
}
else {
    if(guess<target) {
    guess  = (guess + 10);
   
}
else {
    guess  = (guess - 5);
   
}
}
}
return attempts
}