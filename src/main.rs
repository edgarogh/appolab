use appolab::AppoLabConnection;

fn main() {
    let (mut conn, wms) = AppoLabConnection::open().unwrap();

    println!("{}", wms.join("\n"));

    conn.delegate_to_interactive();
}
