#[cfg(test)]
mod tests {
    use yaqoi::QoiHeader;

    #[test]
    fn qoi_header_size() {
        assert_eq!(size_of::<QoiHeader>(), 14); // Header should be 14 bytes long.
    }
}
