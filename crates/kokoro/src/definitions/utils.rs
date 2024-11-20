pub fn extend_add<T>(vec: Option<Vec<T>>, element: T) -> Vec<T> {
    if let Some(vec) = vec {
        let mut new = Vec::with_capacity(vec.len() + 1);
        new.extend(vec);
        new.push(element);
        new
    } else {
        vec![element]
    }
}
