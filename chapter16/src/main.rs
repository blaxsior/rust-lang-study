

fn main() {
    // {
    //     use std::thread;
    //     use std::time::Duration;

    //     let handle = thread::spawn(|| {
    //         for i in 1..10 {
    //             println!("spawned thread: {i}");
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("main thread: {i}");
    //     }
    //     handle.join().unwrap();
    // }

    // {
    //     use std::thread;

    //     let v = vec![1,2,3];
        
    //     let handle = thread::spawn(move || {
    //         println!("vector: {:#?}", v);
    //     });

    //     handle.join().unwrap();
    // }

    // // channel
    // {
    //     // multi producer single consumer
    //     use std::sync::mpsc; 
    //     use std::thread;
    //     use std::time::Duration;


    //     let (tx, rx) = mpsc::channel();

    //     thread::spawn( move || {
    //         let val = String::from("hi");
    //         tx.send(val).unwrap();
    //     });

    //     // recv: 무기한 대기
    //     // recv_timeout: 일정 시간만 대기
    //     // try_recv: 대기 안함

    //     // if let Ok(data) = rx.try_recv() {
    //     //     println!("get immediate data: {}", data);
    //     // }
    //     if let Ok(data) = rx.recv_timeout(Duration::from_secs(2)) {
    //         println!("data: {}", data);
    //     }
    // }

    // // 채널 여러 값 보내기
    // {
    //     use std::sync::mpsc; 
    //     use std::thread;
    //     use std::time::Duration;

    //     let (tx, rx) = mpsc::channel::<String>();
    //     let tx1 = tx.clone();
    //     thread::spawn(move|| {
    //         let vals = vec![
    //             String::from("hello"),
    //             String::from("world"),
    //             String::from("from"),
    //             String::from("thread"),
    //         ];

    //         for val in vals {
    //             tx.send(val).unwrap();
    //             thread::sleep(Duration::from_millis(100));
    //         }

    //         thread::sleep(Duration::from_secs(2));
    //     });

    //     thread::spawn(move|| {
    //         let vals = vec![
    //             String::from("this"),
    //             String::from("is"),
    //             String::from("th1"),
    //         ];

    //         for val in vals {
    //             tx1.send(val).unwrap();
    //             thread::sleep(Duration::from_millis(100));
    //         }

    //         thread::sleep(Duration::from_secs(2));
    //     });

    //     for rcv in rx {
    //         println!("received: {rcv}");
    //     }
    // }
    // mutex
    {
        use std::sync::Mutex;

        let m = Mutex::new(10);

        {
            let mut num = m.lock().unwrap();
            *num += 10;

            drop(num); // 없으면 아래 lock 때문에 무한정 대기

            let mut num2 = m.lock().unwrap();
            *num2 += 20;
        }

        println!("m = {:?}", m);
    }

    {
        use std::sync::Mutex;
        use std::thread;
        use std::sync::Arc;

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 1..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("result: {}", counter.lock().unwrap());
    }
}
