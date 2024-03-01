use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};


/// 클로저로 표현되는 각 작업 목록  
/// ThreadPool은 각 작업을 수신할 수 있는 Worker thread에게 전달
type Job = Box<dyn FnOnce() + Send + 'static>;


/// 스레드 풀 구조체  
/// Worker Thread에게 통신을 통해 작업(Job) 전달
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}


impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        // mpsc는 한번에 하나의 receiver만 존재 가능.
        // 하나의 receiver을 잘 공유해서 사용할 수 있도록 Arc + Mutex 스마트 포인터 필요
        // Arc => 멀티 스레드 환경에서도 안전한 참조 카운터
        // Mutex => 상호 배제(한 순간에 하나의 스레드만 자원 점유 가능)
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            // receiver을 복사하여 보냄
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            // 연결 종료를 위해 sender을 drop할 수 있어야 함 => Option 타입으로 지정
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, action: F)
    where
        F: FnOnce() + Send + 'static, // 단 한번만 실행될 클로저니까 FnOnce로 지정.
    {
        let job = Box::new(action);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("shutdown worker id: {}", worker.id);
            drop(self.sender.take()); //sender 소유권을 드랍 => 채널 닫힘.

            // 채널 닫히면 receiver들은 에러를 받음 => 각 스레드에서 실행 중인 루프 종료
            // 직접적인 작업은 Job에 해당
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

///
/// 스레드 풀의 각 스레드를 표현하는 구조체
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
// 초기에 여러 스레드를 만들어 놓고 처리
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // move에 의해 reveiver의 소유권이 클로저로 이동
            let message = receiver.lock().unwrap().recv();
            // receiver은 하나만 존재. Arc / Mutex 이용하여 한 번에 한 스레드만 사용할 수 있도록 보장.
            // 이 시점에 lock이 더 이상 현재 스레드에 존재하지 않음. 
            // lock 변수가 명시적으로 존재하지 않으므로, recv 이후에 drop됨.

            // 채널은 sender, receiver 중 하나가 끊기면 닫힘(closed).
            // 이에 따라 RecvError 받음. RecvError 받은 경우 스레드에서 실행 중인 루프 종료
            match message { 
                Ok(job) => {
                    println!("worker {id} got a job and executing!");
                    job();
                }
                Err(_) => {
                    println!("disconnected. worker {id} shutdown.");
                    break;
                }
            }
        });

        Worker {
            id,
            // 스레드도 명시적으로 drop될 수 있어야 함 => Option으로 관리
            thread: Some(thread),
        }
    }
}
