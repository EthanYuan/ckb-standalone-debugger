use ckb_vm::instructions::*;
use ckb_vm::machine::VERSION2;

pub fn decode_instruction(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let instruction =
        if data.starts_with("0x") { u32::from_str_radix(&data[2..], 16)? } else { u32::from_str_radix(data, 10)? };
    let (ins, isa) = if let Some(i) = i::factory::<u64>(instruction, VERSION2) {
        let tagged_instruction: tagged::TaggedInstruction = tagged::TaggedInstruction::try_from(i).unwrap();
        (tagged_instruction.to_string(), "I")
    } else if let Some(i) = m::factory::<u64>(instruction, VERSION2) {
        let tagged_instruction = tagged::TaggedInstruction::try_from(i).unwrap();
        (tagged_instruction.to_string(), "M")
    } else if let Some(i) = a::factory::<u64>(instruction, VERSION2) {
        let tagged_instruction = tagged::TaggedInstruction::try_from(i).unwrap();
        (tagged_instruction.to_string(), "A")
    } else if let Some(i) = rvc::factory::<u64>(instruction, VERSION2) {
        let tagged_instruction = tagged::TaggedInstruction::try_from(i).unwrap();
        (tagged_instruction.to_string(), "C")
    } else if let Some(i) = b::factory::<u64>(instruction, VERSION2) {
        let tagged_instruction = tagged::TaggedInstruction::try_from(i).unwrap();
        (tagged_instruction.to_string(), "B")
    } else {
        ("?".to_string(), "?")
    };

    println!("       Assembly = {}", ins);
    if isa == "C" {
        println!("         Binary = {:016b}", instruction);
        println!("    Hexadecimal = {:04x}", instruction);
    } else {
        println!("         Binary = {:032b}", instruction);
        println!("    Hexadecimal = {:08x}", instruction);
    }
    println!("Instruction set = {}", isa);

    Ok(())
}
