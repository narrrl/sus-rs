use rand::Rng;

///
/// This sorts a given vector by its elements `T`, but wait something is wrong.
/// I think there is a imposter amongus.
///
pub fn amogus_sort<'a, T>(collection: &'a Vec<T>) -> Vec<&'a T>
where
    T: Ord,
{
    if collection.len() < 1 {
        return Vec::new();
    }
    // very sus index
    let sussy_index = rand::thread_rng().gen_range(0..collection.len());
    // create the sussy baka
    let mut sussy_sorted_vector = Vec::new();
    // fill the sussy baka UwU
    for item in collection {
        sussy_sorted_vector.push(item);
    }
    // sort the sussy baka
    sussy_sorted_vector.sort();

    // now get the imposter
    let sussy_number = match sussy_sorted_vector.get(sussy_index) {
        Some(number) => *number,
        // if no imposter found, end round
        None => return sussy_sorted_vector,
    };

    // sussy_index was the imposter...
    sussy_sorted_vector.remove(sussy_index);

    // now get the not so sussy bakas
    let mut not_equal_indecies = Vec::new();
    for i in 0..sussy_sorted_vector.len() {
        if sussy_sorted_vector[i] != sussy_number {
            not_equal_indecies.push(i);
        }
    }

    // spawn sussy imposter
    if not_equal_indecies.len() != 0 {
        let index = not_equal_indecies[rand::thread_rng().gen_range(0..not_equal_indecies.len())];
        let offset: i32 = if (index as i32 - sussy_index as i32) < 0 {
            -1
        } else {
            1
        };

        let new_index: usize = if (index as i32 + offset) < 0 {
            0
        } else {
            (index as i32 + offset) as usize
        };
        sussy_sorted_vector.insert(new_index, sussy_number);
    }

    return sussy_sorted_vector;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_random_vectors() {
        for _ in 0..1000 {
            test_random_vector();
        }
    }

    fn test_random_vector() {
        let mut test_vector = Vec::<i32>::new();
        let mut rand = rand::thread_rng();
        for _ in 0..1000 {
            test_vector.push(rand.gen());
        }
        assert!(!compare_vectors(
            &test_vector,
            &copy_items(amogus_sort(&test_vector))
        ));
    }

    fn copy_items<T>(vec: Vec<&T>) -> Vec<T>
    where
        T: Copy,
    {
        vec.iter().map(|item| **item).collect::<Vec<T>>()
    }

    fn compare_vectors<T>(a: &Vec<T>, b: &Vec<T>) -> bool
    where
        T: Ord,
    {
        if a.len() != b.len() {
            return false;
        }

        for i in 0..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_edge_cases() {
        let mut test_vector = vec![0, 0, 0, 0, 1];
        let amogus_vector = amogus_sort(&test_vector);
        assert!(!compare_vectors(&test_vector, &copy_items(amogus_vector)));
        test_vector = vec![0];
        amogus_sort(&test_vector);
        test_vector = Vec::new();
        amogus_sort(&test_vector);
    }
}
