use std::io::Read;

pub fn search(args: clap::ArgMatches) -> std::io::Result<()> {
    let filter = args.value_of("filter");
    if filter.is_none() { panic!("No filter specified") }

    let filter = filter.unwrap();
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();

    stdin.read_to_string(&mut buffer)?;


    let lines: Vec<String> = buffer.lines().map(|x| String::from(x)).filter(|x| x.contains(filter)).collect();

    println!("==========Lines that match your filter:==========");
    for line in lines {
        println!("{}", line)
    };
    println!("==========END OF FILE==========\n");

    Ok(())
}