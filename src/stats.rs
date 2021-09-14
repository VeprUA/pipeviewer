pub fn stats(is_silent: bool, num_read: usize, total_bytes: &mut usize, is_last: bool) {
    *total_bytes += num_read;
    if !is_silent {
        eprint!("\r{}", total_bytes);
        if is_last {
            eprintln!("\r{}", total_bytes);
        }
    }
}
