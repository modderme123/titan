use crate::cpu::{Memory, State};
use crate::cpu::memory::Region;
use crate::elf::Elf;

pub const SMALL_HEAP_SIZE: u32 = 0x10000u32;

pub fn create_simple_state(elf: &Elf, heap_size: u32) -> State {
    let mut memory = Memory::new();

    for header in &elf.program_headers {
        let region = Region {
            start: header.virtual_address,
            data: header.data.clone(),
        };

        memory.mount(region)
    }

    let heap_end = 0x7FFFFFFCu32;

    let heap = Region {
        start: heap_end - heap_size,
        data: vec![0; heap_size as usize]
    };

    memory.mount(heap);

    let mut state = State::new(elf.header.program_entry, memory);
    state.registers[29] = heap_end;

    state
}
