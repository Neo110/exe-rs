use exe::*;

fn main() {

    // let image = VecPE::from_disk_file("../../test/cff_explorer.exe").unwrap();
    // let import_directory = ImportDirectory::parse(&image).unwrap();

    // for descriptor in import_directory.descriptors {
    // println!("Module: {}", descriptor.get_name(&image).unwrap().as_str().unwrap());
    // println!("Imports:");

    // for import in descriptor.get_imports(&image).unwrap() {
    //     match import {
    //         ImportData::Ordinal(x) => println!("   #{}", x),
    //         ImportData::ImportByName(s) => println!("   {}", s)
    //     }
    // }
    // }

    let pe = VecPE::from_disk_file("../../test/cff_explorer.exe").unwrap();
    let rsrc = ResourceDirectory::parse(&pe).unwrap();
    let icons = rsrc.get_first_icon_group(&pe).unwrap();
    
    // 定义一个变量
    let mut feileName = String::new();
    // 如何只取第一个图标
    // 定义一个变量
    // let mut file_name = String::new();

    // if let Some((id, dir)) = icons.iter().next() {
    //     let file_name = match id {
    //         ResolvedDirectoryID::ID(val) => format!("{}.ico", val),
    //         ResolvedDirectoryID::Name(name) => format!("{}.ico", name),
    //     };

    //     println!("Writing {}", file_name);
    //     if file_name == "1" {
    //         let icon_file = dir.to_icon_buffer(&pe).unwrap();
    //         icon_file.save(file_name.clone()).unwrap();
    
    //         println!("Icon dumped from executable: {:?}", file_name);
    //     }

        
    // } else {
    //     println!("No icons found in the executable.");
    // }
    for (id, dir) in &icons {
        // id可以控制取第一个图标

        let filename = match id {
            ResolvedDirectoryID::ID(val) => format!("{}.ico", val),
            ResolvedDirectoryID::Name(name) => format!("{}.ico", name),
        };

        println!("Writing {}", filename);
        if filename == "1.ico" {
            let icon_file = dir.to_icon_buffer(&pe).unwrap();
            icon_file.save(filename.clone()).unwrap();
            //  如何保存文件名 
            feileName = filename.clone();

            // filename 保存到变量中
            break;
        }
    }

    // println!("Icons dumped from executable {:?}",feileName);
}
