pub fn calculate(known_vulns: usize, dependencies_count: usize) -> u8 {
    if known_vulns > 0 {
        return 0;
    }
    
    if dependencies_count > 50 {
        return 50;
    } else if dependencies_count > 10 {
        return 80;
    }
    
    100
}
