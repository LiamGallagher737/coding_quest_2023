const INPUT: &str = include_str!("../INPUT.txt");

fn main() {
    let mut sequenced_messages = vec![String::default(); 6];
    for packet in INPUT.lines() {
        if !packet.starts_with("5555") {
            continue;
        }
        let packet = packet.replacen("5555", "", 1);

        let decoded = decode_hex(packet);
        let _sender = u32::from_be_bytes(decoded[0..4].try_into().unwrap());
        let sequence = u8::from_be(decoded[4]);
        let checksum = u8::from_be(decoded[5]);

        let message = &decoded[6..30];
        let msg_checksum = message.iter().map(|n| *n as u32).sum::<u32>() % 256;

        if checksum != msg_checksum as u8 {
            continue;
        }

        let message_str = String::from_utf8(message.to_vec()).unwrap();

        sequenced_messages.insert(sequence as usize, message_str);
    }

    let full_message = sequenced_messages.join("");
    println!("Message: {full_message}");
}

fn decode_hex(input: String) -> Vec<u8> {
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect()
}
