use pi_play_lib::lasers::{Laser, Receiver};
use std::time::Duration;
use std::{fs, thread};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let mut laser = Laser::new();
    let mut receiver = Receiver::new();
    let message = fs::read_to_string("./src/temp_humid.rs").expect("error opening file");
    // let message = "Hello.".to_string();

    // Encode message with Huffman coding.
    let mut heap = BinaryHeap::new();
    let mut freq_map = HashMap::new();
    for char in message.chars() {
        let count = freq_map.entry(char).or_insert(0);
        *count += 1;
    }
    for (char, freq) in freq_map {
        heap.push((freq, char));
    }
    while heap.len() > 1 {
        let (freq1, _) = heap.pop().unwrap();
        let (freq2, _) = heap.pop().unwrap();
        heap.push((freq1 + freq2, '%'));
    }
    let mut code_map = HashMap::new();
    let mut code = String::new();

    // Recursively traverse Huffman tree to generate code map.
    fn traverse_tree(
        node: &mut BinaryHeap<(i32, char)>,
        code_map: &mut HashMap<char, String>,
        code: &mut String,
    ) {
        if let Some((_, char)) = node.peek() {
            if *char != '%' {
                code_map.insert(*char, code.clone());
            } else {
                code.push('0');
                traverse_tree(node, code_map, code);
                code.pop();
                code.push('1');
                traverse_tree(node, code_map, code);
                code.pop();
            }
        }
    }

    traverse_tree(&mut heap, &mut code_map, &mut code);
    println!("{:?}", code_map);



    let receiver_thread = thread::Builder::new()
        .name("receiver".to_string())
        .spawn(move || loop {
            receiver.print_message();
        });

    let laser_thread = thread::Builder::new()
        .name("laser".to_string())
        .spawn(move || loop {
            laser.send_message(message.clone());
            thread::sleep(Duration::from_millis(2500))
        });

    laser_thread.unwrap().join().unwrap();
    receiver_thread.unwrap().join().unwrap();
}
