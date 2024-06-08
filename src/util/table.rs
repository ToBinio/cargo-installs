pub fn get_column_width<T, F: Fn(&T) -> usize>(header: &str, data: &[T], length_fn: F) -> usize {
    return data
        .iter()
        .map(length_fn)
        .max()
        .unwrap_or(0)
        .max(header.len());
}
