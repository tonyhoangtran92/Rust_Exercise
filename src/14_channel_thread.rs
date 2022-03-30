//channel & sync channel
use crossbeam_channel::unbounded;
use std::thread;

enum Message {
    PrintMsg(String),
    Sum(i32, i32),
    Quit,
}

enum MainMsg {
    ResultSum(i32),
    MainQuit,
}

fn main () {
    let(worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();

    let a = thread::spawn(move|| loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                Message::PrintMsg(data) => println!("{}", data),
                Message::Sum(a,b) => {
                    main_tx.send(MainMsg::ResultSum(a+b));
                }
                Message::Quit => {
                    println!("Quit Program");
                    main_tx.send(MainMsg::MainQuit);
                    break;
                }                
            },
            Err(e) => {
                println!("Error data: {:?}", e);
                main_tx.send(MainMsg::MainQuit);
                break;
            }
        }
    });

    worker_tx.send(Message::PrintMsg("My name is Tony".to_owned()));
    worker_tx.send(Message::Sum(5,7));
    worker_tx.send(Message::Quit);

    a.join();
}