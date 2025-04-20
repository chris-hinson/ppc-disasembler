pub struct DolHeader {
    pub textoffs: [u32; 7],
    pub dataoffs: [u32; 11],
    pub load_addrs: [u32; 18],
    pub sect_sizes: [u32; 18],
    pub bss_addr: u32,
    pub bss_size: u32,
    pub entry_point: u32,
}

impl std::fmt::Display for DolHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, off) in self.textoffs.iter().enumerate() {
            writeln!(f, ".text section {}: {:#08X}", i, off)?
        }

        Ok(())
    }
}
