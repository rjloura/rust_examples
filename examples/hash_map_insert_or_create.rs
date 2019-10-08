
/// While rust provides the combinators `or_insert()` and `or_insert_with()`
/// they cannot handle errors that you may encounter while trying to create
/// the Value you plan to insert into the hash.
///
/// For example, say you are trying to manage a set of network connections, and
/// you want to either get a connection or create a new connection and add it to
/// the hashmap.

// XXX: Copied this code from some work I was doing.  Need to rework it to be
// more generic. i.e. add TcpStream
fn main() {
    let mclient = match client_hash.entry(shard) {
        Occupied(entry) => entry.into_mut(),
        Vacant(entry) => {
            let client = match moray_client::create_client(
                shard,
                job_action.domain_name,
                &job_action.log
            ) {
                Ok(client) => client,
                Err(e) => {
                    // TODO: persistent error for EvacuateObject
                    // in local DB
                    error!("MD Update Worker: failed to get moray \
                                    client for shard number {}. Cannot update \
                                    metadata for {:#?}", shard, mobj);

                    continue;
                }
            };
            entry.insert(client)
        }
    };

}
