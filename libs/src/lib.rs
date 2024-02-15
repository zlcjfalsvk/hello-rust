pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn type_of<T>(_: &T) -> &'static str {
    // 함수의 반환은 return 및 `;` 없이 선언 가능
    // return std::any::type_name::<T>();
    std::any::type_name::<T>()
}