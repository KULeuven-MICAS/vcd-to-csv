use std::{collections::HashMap, fs::File, io::BufReader};

use vcd::{Command, IdCode, Parser, Scope, ScopeItem, Value, Var};

fn parse_vcd_to_data(vcd_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // open the vcd file
    let fs = File::open(vcd_file)?;
    let reader = BufReader::new(fs);
    let mut parser: Parser<BufReader<File>> = Parser::new(reader);

    // parse the header
    let header = parser.parse_header().unwrap();
    let clock: &Var = header.find_var(&["TOP", "clk_i"]).unwrap();
    let scope: &Scope = header
        .find_scope(&[
            "TOP",
            "testharness",
            "i_snax_KUL_cluster",
            "i_snax_core_0_acc_0_snax_streamer_gemmX",
            "i_snax_streamer_gemmX_streamer_wrapper",
            "i_snax_streamer_gemmX_streamer_top",
        ])
        .unwrap();

    let mut scope_identifiers: HashMap<IdCode, String> = HashMap::new();

    // save all variables in scope in scope_identifiers
    for item in &scope.items {
        if let ScopeItem::Var(var) = item {
            scope_identifiers.insert(var.code, var.reference.clone());
        }
    }

    // iterate through vcd file
    let mut frames: Vec<HashMap<IdCode, u64>> = Vec::new();

    // initialize empty current_frame
    let mut current_frame: HashMap<IdCode, u64> = HashMap::new();
    for id_code in scope_identifiers.keys() {
        current_frame.insert(*id_code, 0);
    }

    while let Some(command) = parser.next().transpose()? {
        match command {
            Command::ChangeScalar(id_code, value) => {
                if id_code == clock.code && value == Value::V1 {
                    // rising clock edge detected, push current frame on stack
                    frames.push(current_frame.clone());
                } else if scope_identifiers.contains_key(&id_code) {
                    match value {
                        Value::V1 => current_frame.insert(id_code, 1),
                        // mapping x and z values to 0
                        _ => current_frame.insert(id_code, 0),
                    };
                }
            }
            Command::ChangeVector(id_code, vector) => {
                if scope_identifiers.contains_key(&id_code) {
                    let mut result: u64 = 0;
                    for (i, value) in vector.iter().enumerate() {
                        let bit = match value {
                            Value::V0 | Value::X | Value::Z => 0,
                            Value::V1 => 1,
                        };
                        result |= bit << (vector.len() - i - 1);
                    }
                    current_frame.insert(id_code, result);
                }
            }
            _ => {}
        }
    }

    // Prepare CSV writer
    let mut wtr = csv::Writer::from_path("sim.csv")?;

    // Write headers (variable names)
    let mut headers: Vec<String> = Vec::new();
    for id_code in scope_identifiers.keys() {
        headers.push(scope_identifiers[id_code].clone());
    }
    wtr.write_record(&headers)?;

    // Wrtie data rows
    for frame in frames {
        let mut record: Vec<String> = Vec::new();
        for id_code in scope_identifiers.keys() {
            if let Some(value) = frame.get(id_code) {
                record.push(value.to_string());
            } else {
                record.push("".to_string());
            }
        }
        wtr.write_record(&record)?;
    }

    // Flush and close the CSV writer
    wtr.flush()?;

    Ok(())
}

fn main() {
    if let Err(e) = parse_vcd_to_data("sim.vcd") {
        eprintln!("Error parsing VCD file: {}", e);
    }
}
