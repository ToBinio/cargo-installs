pub fn get_column_width<T, F: Fn(&T) -> usize>(header: &str, data: &Vec<T>, supplier: F) -> usize {
    return data
        .iter()
        .map(|data| supplier(data))
        .max()
        .unwrap_or(0)
        .max(header.len());
}
