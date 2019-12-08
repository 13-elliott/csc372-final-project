#[inline]
pub fn qsort<T: Ord + Send>(list: &mut [T]) {
    quicksort::sort(list);
}

#[inline]
pub fn msort<T: Ord + Send>(list: &mut [T]) {
    mergesort::sort(list);
}

#[inline]
pub fn msort_vec<T: Ord + Send>(list: Vec<T>) -> Vec<T> {
    mergesort::sort_vec(list)
}

#[inline]
pub fn inssort<T: Ord>(list: &mut [T]) {
    insertionsort::sort(list)
}

mod quicksort {
    use crossbeam_utils::thread::scope as scoped_threads;

    /// Sorts the given mutable slice in-place using quicksort. Uses a concurrent algorithm,
        /// thus `T` must be safe to `Send` across thread boundaries
    pub fn sort<T: Ord + Send>(list: &mut [T]) {
        if list.len() > 1 {
            let pivot_pos = find_pivot(list);
            let separator = partition(list, pivot_pos);
            // split into two disjoint, mutable slices
            let (prior, latter) = list.split_at_mut(separator);
            scoped_threads(|s| {
                // spawn new scoped threads
                // for sorting each half recursively
                s.spawn(|_| sort(prior));
                s.spawn(|_| sort(latter));
            })
            .unwrap();
        }
    }

    /// Partitions the given mutable slice such that TODO
    fn partition<T: Ord>(list: &mut [T], pivot_ind: usize) -> usize {
        let end = list.len() - 1;
        // move pivot to the end
        list.swap(pivot_ind, end);
        let mut i = 0;
        for j in 0..end {
            if list[j] <= list[end] {
                list.swap(i, j);
                i += 1;
            }
        }
        // swap the pivot value back into place
        list.swap(i, end);
        i
    }

    fn find_pivot<T: Ord>(list: &[T]) -> usize {
        // TODO: improve
        list.len() / 2
    }
}

mod mergesort {
    use std::ptr;

    use crossbeam_utils::thread::scope as scoped_threads;

    const MIN_SIZE: usize = 10;

    /// Copy the given slice into a new, owned Vec, circumventing
    /// the Clone trait
    unsafe fn slice_to_vec<T>(slc: &[T]) -> Vec<T> {
        slc.iter().map(|v| ptr::read(v)).collect()
    }

    /// Copy given vec back into the given slice. In order for this to be safe,
    /// the values from `src` should be the same values in `dest` TODO ...
    unsafe fn vec_to_slice<T>(src: Vec<T>, dest: &mut [T]) {
        assert_eq!(src.len(), dest.len());
        let dest = dest.as_mut_ptr();
        for (i, val) in src.into_iter().enumerate() {
            ptr::write(dest.add(i), val);
        }
    }

    /// Sorts a mutable slice using mergesort.
    /// Sneakily shallowly copies the values from `list` into an intermediary Vec
    /// which is then sorted with the `sort_vec` function.
    pub fn sort<T: Ord + Send>(list: &mut [T]) {
        let mut intermediate = unsafe { slice_to_vec(list) };
        intermediate = sort_vec(intermediate);
        unsafe { vec_to_slice(intermediate, list) }
    }

    pub fn sort_vec<T: Ord + Send>(mut list: Vec<T>) -> Vec<T> {
        if list.len() <= MIN_SIZE {
            super::inssort(&mut list);
            return list;
        }
        let mid = list.len() / 2;
        // split the other list off into a new Vec
        let other_list = list.drain(mid..).collect();
        // recurse in parallel
        let (a, b) = scoped_threads(|s| {
            const ERR_MSG: &str = "a thread panicked!";
            // "move" keyword specifies that the closures should take
            // ownership of external-scope variables (e.g. "list" and "other_list")
            // rather than to borrow them
            let h1 = s.spawn(move |_| sort_vec(list));
            let h2 = s.spawn(move |_| sort_vec(other_list));
            (h1.join().expect(ERR_MSG), h2.join().expect(ERR_MSG))
        })
        .unwrap();
        merge(a, b)
    }

    /// Merges the two given sorted vectors together into one sorted vector
    pub fn merge<T: Ord>(list_a: Vec<T>, list_b: Vec<T>) -> Vec<T> {
        let mut out = Vec::with_capacity(list_a.len() + list_b.len());
        // convert a & b into peekable iterators
        let mut a = list_a.into_iter().peekable();
        let mut b = list_b.into_iter().peekable();
        loop {
            // peek into both iterators
            let peeked_a = a.peek();
            let peeked_b = b.peek();
            out.push(
                if peeked_a == None {
                    // a is exhausted
                    out.extend(b);
                    return out;
                } else if peeked_b == None {
                    // b is exhausted
                    out.extend(a);
                    return out;
                } else if peeked_a < peeked_b {
                    a.next()
                } else {
                    b.next()
                } // unwraps with certainty; None cases handled above
                .unwrap(),
            );
        }
    }
}

mod insertionsort {
    use std::ptr;

    pub fn sort<T: Ord>(list: &mut [T]) {
        for unsorted_start in 1..list.len() {
            let sorted_list = &list[..unsorted_start];
            let ins_at = match sorted_list.binary_search(&list[unsorted_start]) {
                Ok(i) => rightmost(sorted_list, i),
                Err(i) => i,
            };

            shift_left(list, unsorted_start, ins_at);
        }
    }

    /// returns the index of the rightmost occurrence of the TODO
    fn rightmost<T: Eq>(slice: &[T], mut i: usize) -> usize {
        while i + 1 < slice.len() && slice[i] == slice[i + 1] {
            i += 1;
        }
        i
    }
    
    /// TODO: check over
    fn shift_left<T>(slice: &mut [T], from: usize, to: usize) {
        assert!(from >= to);
        let start = slice.as_mut_ptr();
        unsafe {
            let tmp = ptr::read(start.add(from));
            for i in (to+1..=from).rev() {
                let val = ptr::read(start.add(i-1));
                ptr::write(start.add(i), val);
            }
            // reinsert tmp
            ptr::write(start.add(to), tmp);
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions, Rng};

    const TIMES_PER_TEST: usize = 100;

    fn random_vec(v_size: usize, lower: isize, upper: isize) -> Vec<isize> {
        let mut rng = rand::thread_rng();
        (0..v_size).map(|_| rng.gen_range(lower, upper)).collect()
    }

    fn random_strings(v_size: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        (0..v_size)
            .map(|_| {
                // make a random String
                rng.sample_iter(&distributions::Alphanumeric)
                    // of random length [0, 10)
                    .take(rng.gen_range(0, 10))
                    .collect()
            })
            .collect()
    }

    #[test]
    pub fn qsort() {
        for _ in 0..TIMES_PER_TEST {
            let mut val = random_vec(30, -99, 100);
            println!("prior:  {:?}", &val);
            let mut v2 = val.clone();
            super::qsort(&mut val);
            v2.sort();
            println!("mine:   {:?}\nstdlib: {:?}\n", &val, &v2);
            assert_eq!(val, v2);
        }
    }

    #[test]
    pub fn msort() {
        for _ in 0..TIMES_PER_TEST {
            let mut val = random_vec(30, -99, 100);
            println!("prior:\t{:?}", &val);
            let mut v2 = val.clone();
            super::msort(&mut val);
            v2.sort();
            println!("msort:\t{:?}\nstdlib:\t{:?}\n", &val, &v2);
            assert_eq!(val, v2);
        }
    }

    #[test]
    pub fn msort_strings() {
        for _ in 0..TIMES_PER_TEST {
            let mut val = random_strings(100);
            let mut v2 = val.clone();
            println!("prior:\t{:?}", &val);
            super::msort(&mut val);
            v2.sort();
            println!("qsort:\t{:?}\nstdlib:\t{:?}\n", &val, &v2);
            assert_eq!(val, v2);
        }
    }
    
    

    #[test]
    pub fn inssort() {
        for _ in 0..TIMES_PER_TEST {
            let mut val = random_vec(30, -99, 100);
            println!("prior:\t{:?}", &val);
            let mut v2 = val.clone();
            super::inssort(&mut val);
            v2.sort();
            println!("isort:\t{:?}\nstdlib:\t{:?}\n", &val, &v2);
            assert_eq!(val, v2);
        }
    }
}
