use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = std::path::Path::new(&out_dir);

    let ocaml_p24m = std::fs::read("assets/ocaml.p24m").expect("assets/ocaml.p24m");
    std::fs::write(out_path.join("ocaml.p24m"), &ocaml_p24m).unwrap();
    println!("cargo:rerun-if-changed=assets/ocaml.p24m");

    let pvm_bin = std::fs::read("assets/pvm.bin").expect("assets/pvm.bin");
    std::fs::write(out_path.join("pvm.bin"), &pvm_bin).unwrap();
    println!("cargo:rerun-if-changed=assets/pvm.bin");

    let code_ptr_txt = std::fs::read_to_string("assets/code_ptr_addr.txt")
        .expect("assets/code_ptr_addr.txt");
    let code_ptr = code_ptr_txt.trim();
    println!("cargo:rerun-if-changed=assets/code_ptr_addr.txt");
    println!("cargo:rustc-env=CODE_PTR={code_ptr}");

    let sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    let host = Command::new("hostname")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    let timestamp = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    println!("cargo:rustc-env=BUILD_SHA={sha}");
    println!("cargo:rustc-env=BUILD_HOST={host}");
    println!("cargo:rustc-env=BUILD_TIMESTAMP={timestamp}");
}
