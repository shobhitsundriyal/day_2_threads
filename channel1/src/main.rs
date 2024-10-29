use std::sync::mpsc;

enum Command {
    Hello,
    Goodbye
}
fn main() {
    let (channel_sender, channel_reciver) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command_recived) = channel_reciver.recv() { // wait for a message
            match command_recived {
                Command::Hello => println!("Hello"),
                Command::Goodbye => {
                    println!("Goodbye");
                    break;
                }
            }
        }
    });

    for _ in 0..10 {
        channel_sender.send(Command::Hello).unwrap();
    }

    println!("Send Goodbye");
    channel_sender.send(Command::Goodbye).unwrap();

    handle.join().unwrap();
}

// we have main thread and a spawned thread, swanped thread will wait for a message from main thread
/*
    Output:
    Send Goodbye  // main thread
    Hello
    Hello
    Hello
    Hello
    Hello
    Hello
    Hello
    Hello
    Hello
    Hello
    Goodbye
*/