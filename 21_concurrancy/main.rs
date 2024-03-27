fn main() {
    // ============= Concurrancy =============

    // Concurrent programming, where different parts of a program execute independently.
    // parallel programming, where different parts of a program execute at the same time.

    // Creating a new thread with spawn

    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // This line forces main thread to wait until the spawn thread completes.

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // This line forces main thread to wait until the spawn thread completes.

    let a = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("This is a vector {:?}", a);
    });

    handle.join().unwrap();

    use std::sync::mpsc; // mpsc means "multiple producers single consumer"

    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("Hi...");
        transmitter.send(value).unwrap();
    });

    let received = receiver.recv().unwrap();

    println!("Got: {}", received);
    // ==================
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("Hello"),
            String::from("Namaskar"),
            String::from("Onakkum"),
            String::from("Namaste")
        ];
        
        for value in values {
            transmitter.send(value).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in receiver {
        println!("Got: {}", received);
    }

    // =========================

    let (transmitter, receiver) = mpsc::channel();

    let transmitter_2 = transmitter.clone();

    thread::spawn(move || {
        let values = vec![
            String::from("Hello"),
            String::from("Namaskar"),
            String::from("Onakkum"),
            String::from("Namaste")
        ];
        
        for value in values {
            transmitter.send(value).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    
    thread::spawn(move || {
        let values = vec![
            String::from("Hello 2"),
            String::from("Namaskar 2"),
            String::from("Onakkum 2"),
            String::from("Namaste 2")
        ];
        
        for value in values {
            transmitter_2.send(value).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in receiver {
        println!("Got: {}", received);
    }

    // ======== Mutex - Mutual exclusion =========
    use std::sync::Mutex;

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num *= 5;
    }

    println!("m value is: {:?}", m);

    use std::sync::Arc; // Arc means atomic reference counter

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
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

    println!("Result: {}", *counter.lock().unwrap());
}