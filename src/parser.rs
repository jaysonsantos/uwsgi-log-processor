named!(date<&str, &str>, take_s!(24));
named!(worker_data<&str, &str>,
    ws!(delimited!(tag!("["), take_until!("]"), tag!("]"))));
named!(address<&str, &str>,
    take_until!(" "));
named!(verb<&str, &str>,
    ws!(alt!(tag!("GET") | tag!("POST") | tag!("PUT") | tag!("OPTIONS"))));
named!(url<&str, &str>,
    take_until!(" "));
named!(time<&str, &str>, terminated!(take_until!(" "), ws!(tag!("msecs"))));

named!(pub parse_log<&str, (&str, &str, &str, &str)>,
    do_parse!(
        date: date >>
        worker_data >>
        address >>
        take_until!("]") >>
        take!(1) >>
        verb: verb >>
        url: url >>
        take_until_and_consume!(" in ") >>
        time: time >>
        (date, verb, url, time)
    )
);
