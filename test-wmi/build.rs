use std::{env, os::windows::process::CommandExt};

fn main() {
    let out_dir: std::path::PathBuf = std::env::var_os("OUT_DIR").unwrap().into();

    const VC_PATH: &str = r#"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community"#;
    let vc = if cfg!(target_arch = "x86") {
        "vcvars32.bat"
    } else if cfg!(target_arch = "x86_64") {
        "vcvars64.bat"
    } else {
        unimplemented!("{}", env::var("CARGO_CFG_TARGET_ARCH").unwrap())
    };
    let vcvar_bat = std::path::Path::new(VC_PATH)
        .join(r#"VC\Auxiliary\Build"#)
        .join(vc);

    // config environment by vcvars64/32.bat
    let vcvar_bat = vcvar_bat.to_str().unwrap();
    let src_path = std::path::Path::new(
        r#"C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\WbemCli.Idl"#,
    )
    .to_str()
    .unwrap();

    let midl_command_status = std::process::Command::new("cmd")
        .arg("/c")
        .raw_arg(format!(
            "\"\"{vcvar_bat}\" && midl \"{src_path}\" /tlb WbemCli.tlb\""
        ))
        .current_dir(&out_dir)
        .status()
        .unwrap();
    assert!(midl_command_status.success());

    let wbemcli_rs = {
        let wbemcli_rs = out_dir.join("wbemcli.rs");
        let wbemcli_rs = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(wbemcli_rs)
            .unwrap();
        std::io::BufWriter::new(wbemcli_rs)
    };

    let _ = winapi_tlb_bindgen::build(&out_dir.join("WbemCli.tlb"), false, wbemcli_rs)
        .map_err(|x| {
            eprintln!("{}", x);
            x
        })
        .unwrap();
}
