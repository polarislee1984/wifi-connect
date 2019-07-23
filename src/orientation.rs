use std::fs::File;
use std::io::Read;
use std::fs;

pub fn SetOrientation(orientation:String) {
    let mut file = File::open("/boot/config.txt").expect("unable to read");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    if contents.find("display_rotate=") == None {
        // add display_rotate
        contents = format!("{}\n{}{}", contents, "display_rotate=", orientation);
    } else {
        // update display_rotate
        let lines = contents.lines();
        let mut newContents = String::new();
        for line in lines {
            let mut lineStr = format!("{}", line);
            if lineStr.find("display_rotate=") != None {
                lineStr = format!("{}{}", "display_rotate=", orientation);
            }

            newContents.push_str(&lineStr);
            newContents.push_str("\n");
        }
        contents = newContents
    }

    print!("{}\n", contents);

    //replace content
    fs::write("/boot/config.txt", contents).expect("Unable to write file");
}
