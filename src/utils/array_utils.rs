

use rand::{RngExt, rngs::ThreadRng};

// private static void ShuffleArray(int[] array, Random rand)
//         {
//             for (int i = array.Length - 1; i > 0; i--)
//             {
//                 int j = rand.Next(i + 1);
//                 (array[i], array[j]) = (array[j], array[i]);
//             }
//         }
//

/// maybe inefficient idk, but its only ever used during world gen
pub fn shuffle_vec<T>(vec: &Vec<T>, rng: &mut ThreadRng) -> Vec<T>
where
    T: Clone,
{
    let mut result = vec.clone();

    for i in (0..result.len()).rev() {
        let j = rng.random_range(0..=i);
        result.swap(i, j);
    }

    return result;
}
