pub fn get_column_width<T, F: Fn(&T) -> usize>(header: &str, data: &[T], supplier: F) -> usize {
    return data
        .iter()
        .map(supplier)
        .max()
        .unwrap_or(0)
        .max(header.len());
}
