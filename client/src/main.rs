mod lib;

use self::lib::{utils, net};

fn main() {
    let key = utils::random_key();

    // Connect to socket
    let mut socket = net::Sock {
        stream: net::connect(&"127.0.0.1".to_owned(), &"1234".to_owned()),
        key: key
    }; 

    // Manage our connection
    socket.manage();

    // Close the socket
    socket.close()
}
