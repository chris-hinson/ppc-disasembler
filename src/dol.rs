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

        for (i, off) in self.dataoffs.iter().enumerate() {
            writeln!(f, ".data section {}: {:#08X}", i, off)?
        }

        for (i, addr) in self.load_addrs.iter().enumerate() {
            writeln!(f, "load section {} at addr {:#08X}", i, addr)?
        }

        for (i, size) in self.sect_sizes.iter().enumerate() {
            writeln!(f, "section {} is {} bytes long", i, size)?
        }

        writeln!(f, "bss_addr: {:#08X}", self.bss_addr)?;
        writeln!(f, "bss_size: {:#08X}", self.bss_size)?;
        writeln!(f, "entry addr: {:#08X}", self.entry_point)?;

        Ok(())
    }
}
