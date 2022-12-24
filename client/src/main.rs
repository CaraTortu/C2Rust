mod lib;

use self::lib::{utils, net};

fn main() {
    let key = utils::random_key();

    // Connect to socket
    let mut socket = net::Sock {
        stream: net::connect("127.0.0.1", "1234"),
        key: key
    }; 

    // Manage our connection
    socket.manage();

    // Close the socket
    socket.close()
}
