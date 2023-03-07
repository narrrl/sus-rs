use rand::Rng;

///
/// This sorts a given vector by its elements `T`, but wait something is wrong.
/// I think there is a imposter amongus.
///
pub fn amogus_sort<'a, T>(collection: &'a Vec<T>) -> Vec<&'a T>
where
    T: Ord,
{
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
        let index = rand::thread_rng().gen_range(0..not_equal_indecies.len());
        sussy_sorted_vector.insert(not_equal_indecies[index], sussy_number);
    }

    return sussy_sorted_vector;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut test_vector = Vec::<i32>::new();
        let mut rand = rand::thread_rng();
        for _ in 0..20 {
            test_vector.push(rand.gen());
        }
        amogus_sort(&test_vector);
    }
}
