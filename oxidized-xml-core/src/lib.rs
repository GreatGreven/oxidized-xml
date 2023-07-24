use std::fs;
use amxml::dom::*;
use amxml::xmlerror::XmlError;

pub fn retrieve_values_by(xpath: &str, infile_name: &str) -> Result<Vec<String>, XmlError> {
    let file_content = fs::read_to_string(infile_name).unwrap_or(String::new());
    retrieve_values_in_xml_by(xpath, file_content.as_str())
}

pub fn retrieve_values_in_xml_by(xpath: &str, content: &str) -> Result<Vec<String>, XmlError> {
    let mut values = Vec::new();
    let content = trim_whitespace(content);
    println!("{:?}", content);
    let doc = new_document(content.as_str()).expect("Could not parse XML");
    doc.each_node(xpath, |node| {
        let mut inner_values: Vec<String> = extract_value_from(node);
        values.append(&mut inner_values);
    }).expect("Could not find Nodes with xpath.");
    Ok(values)
}

fn trim_whitespace(buffer: &str) -> String {
    let mut trimmed_buffer = String::new();
    for line in buffer.lines() {
        trimmed_buffer.push_str(line.trim())
    }
    trimmed_buffer
}

fn extract_value_from(node: NodePtr) -> Vec<String>{
    let mut values = Vec::new();
    debug_node(node.clone());
    if node.children().is_empty() {
        //if node is dead-end
        //get the value even if it's empty
        values.push(node.value());
    } else {
        // get all the values from all the children
        // (we could recursively continue down the xml-structure but that will make the functions
        // misleading and we got to this point in the xml-structure by using xpath so manually
        // traversing down the xml-structure
        for child in node.children() {
            debug_node(child.clone());
            if ! child.value().is_empty(){
                values.push(child.value());
            }
        }
    };
    values
}

#[allow(dead_code)]
fn debug_node(node: NodePtr) {
    println!("node:{:?}", node);
    println!("name:{:?}", node.name());
    println!("children:{:?}", node.children());
    println!("value {:?}", node.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_value_in_file() {
        let xpath = "//*[@id='42007']/target";
        let file_path = "./sma_gentext.xml";
        let result = retrieve_values_by(xpath, file_path).unwrap();
        assert_eq!(result.get(0).expect("Vector is empty"), "Obs!");
    }

    #[test]
    fn test_empty_xml() {
        let doc = new_document("").unwrap();
    }
}
