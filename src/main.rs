extern crate rtmp_sys;
extern crate openssl_sys;

fn main() {
  unsafe {
      use rtmp_sys::rtmp;
      println!("{:?}", rtmp::RTMP_LibVersion());

      let rtmp = rtmp::RTMP_Alloc();
//      if (!rtmp.is_null()) {
//      println!("{:?}", rtmp);
        rtmp::RTMP_Init(rtmp);
//  }
//      rtmp::RTMP_Connect(rtmp, std::ptr::null_mut());
  }
}
