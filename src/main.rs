extern crate rtmp_sys;

#[cfg(any(windows))]
use std::os::windows::io::IntoRawSocket;
#[cfg(any(unix))]
use std::os::unix::io::IntoRawFd;

#[cfg(any(windows))]
fn into_fd(stream: std::net::TcpStream) -> i32 {
    stream.into_raw_socket() as i32
}

#[cfg(any(unix))]
fn into_fd(stream: std::net::TcpStream) -> i32 {
    stream.into_raw_fd()
}

fn isready(r: *mut rtmp_sys::rtmp::RTMPPacket) -> bool {
    unsafe { (*r).m_nBytesRead == (*r).m_nBodySize }
}

fn handle_client(stream: std::net::TcpStream) -> () {
    println!("{:?}, {:?}", stream.peer_addr(), stream.local_addr());
    unsafe {
        rtmp_sys::log::RTMP_LogSetLevel(rtmp_sys::log::RTMP_LogLevel::RTMP_LOGALL);
        let rtmp = rtmp_sys::rtmp::RTMP_Alloc();
        rtmp_sys::rtmp::RTMP_Init(rtmp);
        if rtmp.is_null() {
            panic!("rtmp is null")
        }
        let mut packet: rtmp_sys::rtmp::RTMPPacket = Default::default();
        (*rtmp).m_sb.sb_socket = into_fd(stream);
        if rtmp_sys::rtmp::RTMP_Serve(rtmp) == 0 {
            panic!("handshake fail")
        }
        while rtmp_sys::rtmp::RTMP_IsConnected(rtmp) == 1 &&
              rtmp_sys::rtmp::RTMP_ReadPacket(rtmp, &mut packet) == 1 {
            if isready(&mut packet) {
                // ServePacket(server, rtmp, &packet);
                rtmp_sys::rtmp::RTMPPacket_Free(&mut packet);
            }
        }
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
