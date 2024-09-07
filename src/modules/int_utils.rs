/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Checks whether the supplied character is
/// an integer or not. Returns a boolean to this
/// effect.
pub fn is_int(subject: &String) -> bool{
   subject.parse::<usize>().is_ok()
}

/// Raises "base" to the power of "power" and
/// returns the result.
pub fn raise_to(base: &u32, power: &u32) -> u32 {
    base.pow(*power)
}