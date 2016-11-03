extern crate openssl_sys;
extern crate rtmp_sys;

fn handle_client(stream: std::net::TcpStream) -> () {
    println!("{:?}, {:?}", stream.peer_addr(), stream.local_addr());
    unsafe {
        rtmp_sys::log::RTMP_LogSetLevel(rtmp_sys::log::RTMP_LogLevel::RTMP_LOGINFO);
        let rtmp = rtmp_sys::rtmp::RTMP_Alloc();
        rtmp_sys::rtmp::RTMP_Init(rtmp);
        if rtmp.is_null() {
            panic!("rtmp is null")
        }
        let packet: rtmp_sys::rtmp::RTMPPacket;
    }
}

fn main() {
    let version = unsafe { rtmp_sys::rtmp::RTMP_LibVersion() };
    println!("version = {}.{}.{}",
             version >> 16 & 0xff,
             version >> 8 & 0xff,
             version & 0xff);
    let listener = std::net::TcpListener::bind("0.0.0.0:4242").unwrap();
    let mut handle_thread = Vec::new();
    //  let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let builder = std::thread::Builder::new().name("handle_client".into());
                //        let thread_tx = tx.clone();

                match builder.spawn(move || handle_client(stream)) {
                    Ok(thread) => {
                        handle_thread.push(thread);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    for thread in handle_thread {
        match thread.join() {
            Ok(_) => {
                return;
            }
            Err(_) => {
                return;
            }
        }
    }
}
