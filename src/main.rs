let base_address = f.read_u16::<BigEndian>().expect("error");

let mut address = base_address as usize;
loop {
    match f.read_u16::<BigEndian>() {
        Ok(instruction) => {
            vm.write_memory(address, instruction);
            address += 1;
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                println!("OK")
            } else {
                print!("failed: {}", e);
            }
            break;
        }
    }
}