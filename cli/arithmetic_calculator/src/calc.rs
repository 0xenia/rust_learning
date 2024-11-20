pub struct Calc;

impl Calc {
    pub fn add(array: &[i32]) -> i32 {
        array.iter().sum()
    }

    pub fn multiply(array: &[i32]) -> i32 {
        array.iter().product()
    }

    pub fn divide(array: &[i32]) -> Option<f32> {
        if array.len() < 2 || array.iter().skip(1).any(|&item| item == 0) {
            return None;
        }

        let mut result = array[0] as f32;

        for &num in &array[1..] {
            result /= num as f32;
        }

        Some(result)
    }
    pub fn subtract(array: &[i32]) -> i32 {
        array.iter().skip(1).fold(array[0], |acc, &item| acc - item)
    }
}