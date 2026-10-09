use cubecl::prelude::*;

#[cube]
fn top_level_sample<T: Float + CubeElement>(array: &Array<T>, i: usize) -> T {
    array[i]
}

#[cube]
pub fn works<T: Float + CubeElement>(array: &Array<T>) -> T {
    top_level_sample::<T>(array, 0) + top_level_sample::<T>(array, 1)
}

#[cube]
pub fn nested_index<T: Float + CubeElement>(array: &Array<T>) -> T {
    #[cube]
    fn sample<T: Float + CubeElement>(array: &Array<T>, i: usize) -> T {
        array[i]
    }

    sample::<T>(array, 0) + sample::<T>(array, 1)
}

#[cube]
pub fn nested_no_index<T: Float + CubeElement>(a: T, b: T) -> T {
    #[cube]
    fn add<T: Float + CubeElement>(a: T, b: T) -> T {
        a + b
    }

    add::<T>(a, b)
}
